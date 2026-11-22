//! Hebi-Ichigo (pzprjs `variety/hebi.js`, pid `hebi`, old name `snakes`).
//!
//! Cells hold an arrow direction in `qdir` (0 none, 1 up, 2 down, 3 left,
//! 4 right) together with a clue number in `qnum` (0–5); `qnum` -3 is `+` (an
//! arrow without a number) and -2 is `?`. The answer is a snake: `anum` numbers
//! its cells 1–5 and `qsub` 1 marks a dot. The URL is
//! `decodeArrowNumber16`; FileIO stores `qdir,qnum` per cell and then the
//! answer layer (`decodeCellAnumsub`). Rotations/reflections rotate `qdir`
//! (`adjustNumberArrow` in `BoardExec.adjustBoardData`).

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Hebi;

struct Hebi;

impl Genre for Hebi {
    fn pid(&self) -> &'static str {
        "hebi"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    /// pzprjs `Board.hascross` (the inherited default 2); the URL and FileIO
    /// store no cross data.
    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_arrow_number16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_arrow_number16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_cell_direc_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_direc_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cell.maxnum` is 5, `minnum` 0 outside play mode.
        if b.cells.iter().any(|c| {
            !(c.qnum == -3
                || c.qnum == -2
                || c.qnum == -1
                || (0..=5).contains(&c.qnum))
                || !(0..=4).contains(&c.qdir)
                || !(0..=4).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "hebi cell value outside clue 0-5/+/? or direction 0-4 / answer mark",
            );
        }
        if b.cells.iter().any(|c| {
            c.qnum >= 4096 || (c.qdir != 0 && !(c.qnum >= 0 || c.qnum == -2))
        }) {
            d.push(
                DiagnosticCode::LossyUrl,
                "hebi numbers of 4096 or more, or a direction without a number, are dropped from the URL",
            );
        }
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` → `adjustNumberArrow`
        // → `adjustCellArrowField(key, d, "qdir")`.
        adjust_cell_qdirs(b, t);
    }
}

/// `decodeArrowNumber16`: `+` is a direction-less number, `0`–`4` plus one hex
/// digit a direction (`.`) and number 0–15, `5`–`9` plus two hex digits numbers
/// 16–255, `-` plus one hex direction and three hex digits numbers 256–4095,
/// and `a`–`z` blank runs that skip `digit - 10` cells.
fn decode_arrow_number16(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let Some(cell) = b.cells.get_mut(c) else {
            break;
        };
        if ca == '+' {
            cell.qdir = 0;
            cell.qnum = -3;
        } else if ('0'..='4').contains(&ca) {
            cell.qdir = ca.to_digit(16).unwrap() as i32;
            match chars.get(i + 1).copied() {
                Some('.') => cell.qnum = -2,
                Some(ca1) if ca1.is_ascii_hexdigit() => {
                    cell.qnum = ca1.to_digit(16).unwrap() as i32;
                }
                _ => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "direction character is not followed by a hexadecimal number",
                ),
            }
            i += 1;
        } else if ('5'..='9').contains(&ca) {
            cell.qdir = ca.to_digit(16).unwrap() as i32 - 5;
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qn) => cell.qnum = qn as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal digits after a direction character",
                ),
            }
            i += 2;
        } else if ca == '-' {
            if let Some(dir) = jsnum::parse_int(&jsnum::substr(&chars, i + 1, 1), 16) {
                cell.qdir = dir as i32;
            } else {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal direction after '-'",
                );
            }
            if let Some(qn) = jsnum::parse_int(&jsnum::substr(&chars, i + 2, 3), 16) {
                cell.qnum = qn as i32;
            } else {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal number after a direction",
                );
            }
            i += 4;
        } else if ca.is_ascii_lowercase() {
            c += jsnum::digit(ca, 36).unwrap() as usize - 10;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} is skipped"),
            );
        }
        c += 1;
        if c >= n {
            break;
        }
        i += 1;
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    // pzprjs consumed the character at `i` before breaking; the rest starts after it.
    chars[(i + 1).min(chars.len())..].iter().collect()
}

/// `encodeArrowNumber16`: the inverse of [`decode_arrow_number16`].
fn encode_arrow_number16(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let (dir, qn) = (cell.qdir, cell.qnum);
        let pstr = if qn == -2 {
            Some(format!("{dir}."))
        } else if qn == -3 {
            Some("+".into())
        } else if (0..16).contains(&qn) {
            Some(format!("{dir}{}", jsnum::to_radix(qn as u32, 16)))
        } else if (16..256).contains(&qn) {
            // pzprjs adds 5 to the direction, then appends the two hex digits.
            Some(format!("{}{}", dir + 5, jsnum::to_radix(qn as u32, 16)))
        } else if (256..4096).contains(&qn) {
            Some(format!("-{dir}{}", jsnum::to_radix(qn as u32, 16)))
        } else {
            None
        };
        match pstr {
            Some(pstr) => {
                if count == 0 {
                    cm.push_str(&pstr);
                } else {
                    cm.push_str(&jsnum::to_radix(count + 9, 36));
                    cm.push_str(&pstr);
                    count = 0;
                }
            }
            None => {
                count += 1;
                if count == 26 {
                    cm.push_str(&jsnum::to_radix(count + 9, 36));
                    count = 0;
                }
            }
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(count + 9, 36));
    }
    cm
}

/// `decodeCellDirecQnum`: `#` is a cell with an arrow but no number, otherwise
/// `dir,qnum` with `-` for `?`.
fn decode_cell_direc_qnum(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "#" {
            cell.qdir = 0;
            cell.qnum = -3;
        } else if ca != "." {
            let mut inp = ca.split(',');
            let dir = inp.next().unwrap_or("");
            let num = inp
                .next()
                .ok_or_else(|| format!("expected \"dir,qnum\", found {ca:?}"))?;
            cell.qdir = if dir != "0" {
                fileio::int(dir)?
            } else {
                0
            };
            cell.qnum = if num != "-" {
                fileio::int(num)?
            } else {
                -2
            };
        }
        Ok(())
    })
}

/// `encodeCellDirecQnum`: `#` / `.` / `dir,qnum` per cell.
fn encode_cell_direc_qnum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qnum == -3 {
            "# ".into()
        } else if cell.qnum != -1 {
            format!(
                "{},{} ",
                if cell.qdir != 0 {
                    cell.qdir.to_string()
                } else {
                    "0".into()
                },
                if cell.qnum != -2 {
                    cell.qnum.to_string()
                } else {
                    "-".into()
                }
            )
        } else {
            ". ".into()
        }
    });
}

/// pzprjs `BoardExec.getTranslateDir`, as one direction value mapping.
fn translate_dir(op: Turn, dir: i32) -> Option<i32> {
    match (op, dir) {
        (Turn::Right, 1) => Some(4),
        (Turn::Right, 2) => Some(3),
        (Turn::Right, 3) => Some(1),
        (Turn::Right, 4) => Some(2),
        (Turn::Left, 1) => Some(3),
        (Turn::Left, 2) => Some(4),
        (Turn::Left, 3) => Some(2),
        (Turn::Left, 4) => Some(1),
        (Turn::FlipX, 3) => Some(4),
        (Turn::FlipX, 4) => Some(3),
        (Turn::FlipY, 1) => Some(2),
        (Turn::FlipY, 2) => Some(1),
        _ => None,
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Right,
    Left,
    FlipX,
    FlipY,
}

/// The pzprjs board operations a pzprst transform is composed of, in order
/// (`rot90` = turnr, `transpose` = turnl + flipy, …).
fn turn_ops(t: Transform) -> &'static [Turn] {
    match t {
        Transform::Identity => &[],
        Transform::Rot90 => &[Turn::Right],
        Transform::Rot180 => &[Turn::Right, Turn::Right],
        Transform::Rot270 => &[Turn::Left],
        Transform::FlipX => &[Turn::FlipX],
        Transform::FlipY => &[Turn::FlipY],
        Transform::Transpose => &[Turn::Left, Turn::FlipY],
        Transform::AntiTranspose => &[Turn::Right, Turn::FlipY],
    }
}

/// `adjustCellArrowField(key, d, "qdir")`: each board operation turns the
/// arrow directions of all cells before the coordinates move.
fn adjust_cell_qdirs(b: &mut Board, t: Transform) {
    for op in turn_ops(t) {
        for cell in &mut b.cells {
            if let Some(dir) = translate_dir(*op, cell.qdir) {
                cell.qdir = dir;
            }
        }
    }
}

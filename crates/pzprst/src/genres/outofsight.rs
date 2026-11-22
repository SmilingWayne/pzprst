//! Out of Sight (pzprjs `variety/yajikazu.js`, pid `outofsight`).
//!
//! Cells hold a clue number (`qnum`), `?` (-2) or `+` (an arrow without a
//! number, -3), and an arrow direction (`qdir` 0 none, 1 up, 2 down, 3 left,
//! 4 right). The URL is `decodeArrowNumber16`; FileIO stores `dir,qnum` per
//! cell, then the shading layer and the border line layer. Rotations and
//! reflections turn the arrows (`adjustNumberArrow` in
//! `BoardExec.adjustBoardData`).

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &OutOfSight;

struct OutOfSight;

impl Genre for OutOfSight {
    fn pid(&self) -> &'static str {
        "outofsight"
    }

    fn hasborder(&self) -> u8 {
        1
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
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_direc_qnum(w, b);
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line_if_present(w, b);
        Ok(())
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` → `adjustNumberArrow`
        // → `adjustCellArrowField(key, d, "qdir")`.
        for op in turn_ops(t) {
            for cell in &mut b.cells {
                if let Some(dir) = translate_dir(*op, cell.qdir) {
                    cell.qdir = dir;
                }
            }
        }
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell@outofsight`: `minnum` 1, `maxnum` 6; the URL/FileIO also keep
        // 0 (a direct-assignment leftover) and the -3 "arrow without number".
        if b.cells.iter().any(|c| {
            !(c.qnum == -3 || c.qnum == -2 || c.qnum == -1 || (0..=6).contains(&c.qnum))
                || !(0..=4).contains(&c.qdir)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "outofsight cell value outside clue 0-6/?/+ or direction 0-4",
            );
        }
        if b.cells
            .iter()
            .any(|c| !(0..=1).contains(&c.qans) || !(0..=1).contains(&c.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "outofsight cell answer value outside shade 0/1 or background mark 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| e.line < 0 || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "outofsight border value outside line >= 0 or cross -1",
            );
        }
    }
}

/// `decodeArrowNumber16`: `+` is an arrow without a number, `0`–`4` plus one
/// hex digit a direction and number 0–15 (`.` for `?`), `5`–`9` plus two hex
/// digits numbers 16–255, `-` plus one hex direction and three hex digits
/// numbers 256–4095, and `a`–`z` runs of empty cells.
fn decode_arrow_number16(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let mut used = 1usize;
        if ca == '+' {
            let cell = &mut b.cells[c];
            cell.qdir = 0;
            cell.qnum = -3;
        } else if ('0'..='4').contains(&ca) {
            let cell = &mut b.cells[c];
            cell.qdir = ca.to_digit(16).unwrap() as i32;
            used = 2;
            let next = chars.get(i + 1).copied();
            if next == Some('.') {
                cell.qnum = -2;
            } else if let Some(v) = next.and_then(|ch| jsnum::digit(ch, 16)) {
                cell.qnum = v as i32;
            } else {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("arrow number at character {i} is not hexadecimal"),
                );
            }
        } else if ('5'..='9').contains(&ca) {
            let cell = &mut b.cells[c];
            cell.qdir = ca.to_digit(16).unwrap() as i32 - 5;
            used = 3;
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(v) => cell.qnum = v as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("arrow number at character {i} is not hexadecimal"),
                ),
            }
        } else if ca == '-' {
            let cell = &mut b.cells[c];
            used = 5;
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 1), 16) {
                Some(v) => cell.qdir = v as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("arrow direction at character {i} is not hexadecimal"),
                ),
            }
            match jsnum::parse_int(&jsnum::substr(&chars, i + 2, 3), 16) {
                Some(v) => cell.qnum = v as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("arrow number at character {i} is not hexadecimal"),
                ),
            }
        } else if ca.is_ascii_lowercase() {
            c += jsnum::digit(ca, 36).unwrap() as usize - 10;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} is skipped"),
            );
        }
        c += 1;
        i += used;
        if c >= n {
            break;
        }
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    chars[i.min(chars.len())..].iter().collect()
}

/// `encodeArrowNumber16`: the inverse of [`decode_arrow_number16`], with
/// `9 + count` in base 36 for blank runs, flushed after 26 blanks like pzprjs.
fn encode_arrow_number16(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let dir = cell.qdir;
        let qn = cell.qnum;
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

/// `decodeCellDirecQnum`: `#` is an arrow without a number, otherwise
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
            let dir = cell.qdir.to_string();
            let num = cell.qnum.to_string();
            format!(
                "{},{} ",
                if cell.qdir != 0 { dir } else { "0".into() },
                if cell.qnum != -2 { num } else { "-".into() }
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

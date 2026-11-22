//! Crossstitch (pzprjs `variety/crossstitch.js`, pid `crossstitch`).
//!
//! Cells hold a clue number in `qnum`, an arrow direction in `qdir` (0 none,
//! 1 up, 2 down, 3 left, 4 right; `qnum` -3 is `+`, -2 is `?`), a slash answer
//! in `qans` (31 `\`, 32 `/`, 33 both), a direction mark bitmask in `qsub` and
//! the "clue done" mark in `qcmp`. Crosses hold dots in `qsub`; pzprst has no
//! dot objects, so the FileIO cross layer lives on the crosses.
//!
//! The URL is `decodeArrowNumber16`; FileIO stores `decodeCellDirecQnum`, the
//! answer layer written by hand (`o`–`u` for `qsub` 1–7, a digit for
//! `qans`-30, `c` for `qcmp`) and the cross dot layer. Rotations/reflections
//! swap `qans` 31/32 (`33` stays) and rotate `qdir` (`adjustNumberArrow` in
//! `BoardExec.adjustBoardData`), once per pzprjs turn/flip operation.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Crossstitch;

struct Crossstitch;

impl Genre for Crossstitch {
    fn pid(&self) -> &'static str {
        "crossstitch"
    }

    fn hasborder(&self) -> u8 {
        0
    }

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
        r.decode_cell(b, d, |cell, ca| {
            let mut ca = ca;
            if let Some(c) = ca.chars().next() {
                let code = c as i32;
                if code > 110 && code <= 117 {
                    cell.qsub = code - 110;
                    ca = &ca[c.len_utf8()..];
                }
            }
            if ca == "c" {
                cell.qcmp = 1;
            } else if let Some(n) = jsnum::integer(ca) {
                if n != 0 {
                    cell.qans = n + 30;
                }
            }
            Ok(())
        })?;
        r.decode_cross(b, d, false, |cross, ca| {
            cross.qsub = fileio::int(ca)?;
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_direc_qnum(w, b);
        w.encode_cell(b, |cell| {
            let mut s = String::new();
            if cell.qsub > 0 {
                s.push(char::from_u32((110 + cell.qsub) as u32).unwrap_or(char::REPLACEMENT_CHARACTER));
            }
            if cell.qans > 30 {
                s.push_str(&(cell.qans - 30).to_string());
            } else if cell.qcmp != 0 {
                s.push('c');
            } else if s.is_empty() {
                s.push('.');
            }
            s.push(' ');
            s
        });
        w.encode_cross(b, false, |cross| format!("{} ", cross.qsub));
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cell.minnum` 0 and `maxnum` = max(4, max(cols, rows) >> 1);
        // -3 is the arrow-only `+` cell, -2 is `?`. `qans` is the slash answer
        // (31 `\`, 32 `/`, 33 both), `qsub` the 3-bit direction mark. Crosses
        // hold the two dot states.
        let maxnum = 4.max((b.cols().max(b.rows()) >> 1) as i32);
        let mut bad = false;
        let mut lossy = false;
        for cell in &b.cells {
            let qnum_ok = cell.qnum == -1
                || cell.qnum == -2
                || cell.qnum == -3
                || (0..=maxnum).contains(&cell.qnum);
            if !qnum_ok
                || !(0..=4).contains(&cell.qdir)
                || !(cell.qans == 0 || (31..=33).contains(&cell.qans))
                || !(0..=7).contains(&cell.qsub)
                || !(0..=1).contains(&cell.qcmp)
            {
                bad = true;
            }
            if cell.qnum >= 4096 || (cell.qdir != 0 && !(cell.qnum >= 0 || cell.qnum == -2)) {
                lossy = true;
            }
        }
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!(
                    "crossstitch cell value outside clue 0-{maxnum} / ? / +, direction 0-4, slash 0/31/32/33, sub 0-7 or mark 0/1"
                ),
            );
        }
        if lossy {
            d.push(
                DiagnosticCode::LossyUrl,
                "crossstitch numbers of 4096 or more, or a direction without a number, are dropped from the URL",
            );
        }
        if b.crosses
            .iter()
            .any(|cross| !(0..=2).contains(&cross.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "crossstitch cross dot outside none / 1 / 2",
            );
        }
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData`: each turn/flip swaps `\` and `/`
        // (`33` is both and stays) and `adjustNumberArrow` rotates the arrow
        // directions before the coordinates move.
        for op in turn_ops(t) {
            for cell in &mut b.cells {
                cell.qans = match cell.qans {
                    31 => 32,
                    32 => 31,
                    v => v,
                };
                if let Some(dir) = translate_dir(*op, cell.qdir) {
                    cell.qdir = dir;
                }
            }
        }
    }
}

/// `decodeArrowNumber16`: arrow cells, two characters per cell
/// (`dir` + hex number, `.` for `?`), `+` for a cell with an arrow but no
/// number, and base-36 run lengths for empty cells.
fn decode_arrow_number16(b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = body.chars().collect();
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
            // `dir` + one hex digit, or `.` for `?`.
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
            // `5` + `dir` + two hex digits (`dir` 0-4).
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
            // `-` + `dir` + three hex digits.
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
    chars.iter().skip(i).collect()
}

/// `encodeArrowNumber16`: the inverse of [`decode_arrow_number16`].
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

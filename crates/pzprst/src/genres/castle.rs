//! Castle Wall (pzprjs `variety/castle.js`, pid `castle`).
//!
//! Cell clues are an arrow direction (`qdir`: 0 none, 1 up, 2 down, 3 left,
//! 4 right) with a number (`qnum`: -2 `?`, otherwise the clue) and a wall
//! shade (`ques`: 0 gray, 1 white, 2 black). The URL stores each non-empty cell
//! as `<ques><dir><number>` with base-36 run lengths for empty cells; FileIO
//! stores `<dir>,<qnum>,<ques|qcmp<<2>` per cell and then the border line
//! layer. `qcmp` is part of the FileIO cell token only.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Castle;

/// pzprjs `Cell.maxnum`: the largest clue the board can hold.
fn maxnum(b: &Board) -> i32 {
    b.cols().max(b.rows()) as i32 - 2
}

struct Castle;

impl Genre for Castle {
    fn pid(&self) -> &'static str {
        "castle"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_castle(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_castle(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "." {
                return Ok(());
            }
            let mut inp = ca.split(',');
            let dir = inp.next().unwrap_or("");
            let qnum = inp
                .next()
                .ok_or_else(|| format!("expected \"dir,qnum,ques\", found {ca:?}"))?;
            let flags = inp
                .next()
                .ok_or_else(|| format!("expected \"dir,qnum,ques\", found {ca:?}"))?;
            cell.qdir = if dir != "0" {
                fileio::int(dir)?
            } else {
                0
            };
            cell.qnum = if qnum != "-" {
                fileio::int(qnum)?
            } else {
                -2
            };
            let num = fileio::int(flags)?;
            cell.ques = num & 3;
            cell.qcmp = if num & 4 != 0 { 1 } else { 0 };
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            let dir = if cell.qdir != 0 {
                cell.qdir.to_string()
            } else {
                "0".into()
            };
            let qnum = if cell.qnum != -2 {
                cell.qnum.to_string()
            } else {
                "-".into()
            };
            let num = (cell.ques & 3) | ((cell.qcmp & 1) << 2);
            format!("{dir},{qnum},{num} ")
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = maxnum(b);
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
                || !(0..=4).contains(&c.qdir)
                || !(0..=3).contains(&c.ques)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "castle cell value outside empty/?/clue 0..max(cols,rows)-2, direction 0-4, shade 0-2 or cmp 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "castle border value outside line 0/1 or cross -1",
            );
        }
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` → `adjustNumberArrow`
        // → `adjustCellArrowField(key, d, "qdir")`.
        adjust_cell_qdirs(b, t);
    }
}

/// `decodePzpr`: one cell per `<ques><dir><number>` group, with base-36 run
/// lengths for empty cells.
fn decode_castle(b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = body.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() && c < n {
        let ca = chars[i];
        if ca.is_ascii_lowercase() {
            c += jsnum::digit(ca, 36).unwrap() as usize - 9;
            i += 1;
            continue;
        }

        // `cell.ques = +bstr.charAt(i)`; anything but a digit would be NaN.
        if let Some(ques) = ca.to_digit(10) {
            b.cells[c].ques = ques as i32;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected shade character {ca:?} is skipped"),
            );
        }
        i += 1;

        let ca = chars.get(i).copied();
        if let Some(ca) = ca.filter(|ch| ('0'..='4').contains(ch)) {
            let cell = &mut b.cells[c];
            cell.qdir = ca.to_digit(16).unwrap() as i32;
            match chars.get(i + 1).copied() {
                Some('.') => cell.qnum = -2,
                next => match next.and_then(|ch| jsnum::digit(ch, 16)) {
                    Some(v) => cell.qnum = v as i32,
                    None => d.push(
                        DiagnosticCode::UnexpectedChar,
                        "castle cell number is not one hexadecimal digit",
                    ),
                },
            }
            i += 1;
        } else if let Some(ca) = ca.filter(|ch| ('5'..='9').contains(ch)) {
            let cell = &mut b.cells[c];
            cell.qdir = ca.to_digit(16).unwrap() as i32 - 5;
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(v) => cell.qnum = v as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "castle cell number is not hexadecimal",
                ),
            }
            i += 2;
        } else if ca == Some('-') {
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 1), 16) {
                Some(v) => b.cells[c].qdir = v as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "castle cell direction is not hexadecimal",
                ),
            }
            match jsnum::parse_int(&jsnum::substr(&chars, i + 2, 3), 16) {
                Some(v) => b.cells[c].qnum = v as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "castle cell number is not hexadecimal",
                ),
            }
            i += 4;
        }
        c += 1;
        i += 1;
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    // `this.outbstr = bstr.substr(i + 1)`: pzprjs also drops the character at
    // the position the loop stopped on.
    chars.iter().skip((i + 1).min(chars.len())).collect()
}

/// `encodePzpr`, the inverse of [`decode_castle`].
fn encode_castle(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let dir = cell.qdir;
        let qn = cell.qnum;
        let qs = cell.ques;
        let shade = jsnum::to_radix(qs as u32, 16);
        let pstr = if qn == -2 {
            Some(format!("{shade}{dir}."))
        } else if (0..16).contains(&qn) {
            Some(format!("{shade}{dir}{}", jsnum::to_radix(qn as u32, 16)))
        } else if (16..256).contains(&qn) {
            // `dir + 5` is a numeric addition, then the two hex digits follow.
            Some(format!(
                "{shade}{}{}",
                dir + 5,
                jsnum::to_radix(qn as u32, 16)
            ))
        } else if (256..4096).contains(&qn) {
            Some(format!("{shade}-{dir}{}", jsnum::to_radix(qn as u32, 16)))
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

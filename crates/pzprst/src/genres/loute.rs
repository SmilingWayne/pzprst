//! L-route (pzprjs `variety/loute.js`, pid `loute`). A cell's `qdir` holds the
//! route direction (1 up, 2 down, 3 left, 4 right), 5 for a circle, -2 for `?`
//! and 0 for an empty cell. FileIO stores the answer layer (`qans` shading,
//! `qsub` mark) and answer borders.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Loute;

struct Loute;

impl Genre for Loute {
    fn pid(&self) -> &'static str {
        "loute"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_loute(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_loute(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if let Some(rest) = ca.strip_prefix('o') {
                cell.qdir = 5;
                if !rest.is_empty() {
                    cell.qnum = fileio::int(rest)?;
                }
            } else if ca == "-" {
                cell.qdir = -2;
            } else if ca == "#" {
                cell.qans = 1;
            } else if ca == "_" {
                cell.qsub = 1;
            } else if ca != "." {
                cell.qdir = fileio::int(ca)?;
            }
            Ok(())
        })?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.qdir == -2 {
                "- ".into()
            } else if cell.qdir != 0 {
                format!("{} ", cell.qdir)
            } else if cell.qsub != 0 {
                "_ ".into()
            } else if cell.qans != 0 {
                "# ".into()
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qdir == -2 || (0..=5).contains(&c.qdir))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "loute cell value outside direction 0..4 / circle 5 / ?, or answer mark",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "loute border answer flag outside 0/1",
            );
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` → `adjustNumberArrow`
        // → `adjustCellArrowField(key, d, "qdir")`.
        adjust_cell_qdirs(b, t);
    }
}

/// `decodeLoute`: one hex digit per cell (`qdir`), `.` for `?`, base-36 run
/// lengths for empty cells.
fn decode_loute(b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = body.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        if ca.is_ascii_digit() || ('a'..='f').contains(&ca) {
            b.cells[c].qdir = ca.to_digit(16).unwrap() as i32;
        } else if ca == '.' {
            b.cells[c].qdir = -2;
        } else if ca.is_ascii_lowercase() {
            c += jsnum::digit(ca, 36).unwrap() as usize - 16;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} skips one cell"),
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
    // pzprjs assigns `substr(i)` here, which leaves the last processed
    // character in `outbstr`; nothing reads it, so the port consumes it.
    chars.iter().skip(i + 1).collect()
}

/// `encodeLoute`: the inverse of [`decode_loute`].
fn encode_loute(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let dir = cell.qdir;
        let pstr = if dir == -2 {
            Some(".".to_string())
        } else if dir != 0 {
            Some(to_hex(dir))
        } else {
            None
        };
        match pstr {
            Some(pstr) => {
                if count == 0 {
                    cm.push_str(&pstr);
                } else {
                    cm.push_str(&jsnum::to_radix(count + 15, 36));
                    cm.push_str(&pstr);
                    count = 0;
                }
            }
            None => {
                count += 1;
                if count == 20 {
                    cm.push_str(&jsnum::to_radix(count + 15, 36));
                    count = 0;
                }
            }
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(count + 15, 36));
    }
    cm
}

/// JavaScript `n.toString(16)` for an integer (negative values keep their sign).
fn to_hex(n: i32) -> String {
    if n < 0 {
        format!("-{:x}", -i64::from(n))
    } else {
        format!("{n:x}")
    }
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
/// direction of all cells.
fn adjust_cell_qdirs(b: &mut Board, t: Transform) {
    for op in turn_ops(t) {
        for cell in &mut b.cells {
            if let Some(dir) = translate_dir(*op, cell.qdir) {
                cell.qdir = dir;
            }
        }
    }
}

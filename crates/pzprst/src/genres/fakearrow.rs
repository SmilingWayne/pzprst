//! Fake Arrow (pzprjs `variety/nagare.js`, pid `fakearrow`).
//!
//! `hasborder` 1. The URL is the inner-border wall layer (`decodeBorder`)
//! followed by a `genericDecodeNumber16` cell layer: 1–4 are arrow directions
//! (`qdir`), 5 marks a cell that does not exist (`ques` 7). FileIO stores the
//! border walls, the cell direction or `#`, the border lines with an optional
//! `u`/`d`/`l`/`r` aux mark (`decodeBorderArrowAns`) and the answer layers
//! (`decodeCellQanssub`). Rotations and reflections turn cell arrows and the
//! border aux marks (`BoardExec.adjustBoardData` in `variety/nagare.js`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &FakeArrow;

struct FakeArrow;

impl Genre for FakeArrow {
    fn pid(&self) -> &'static str {
        "fakearrow"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        encode::generic_decode_number16(b.cells.len(), &rest, d, |c, val| {
            let cell = &mut b.cells[c];
            if val == 5 {
                // `cell.setValid(7)`: an empty cell (`isEmpty` is `ques === 7`).
                cell.ques = 7;
                cell.qnum = -1;
                cell.qans = 0;
                cell.qsub = 0;
            } else if val >= 0 {
                cell.qdir = val;
            }
        })
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let cells = encode::generic_encode_number16(b.cells.len(), |c| {
            let cell = &b.cells[c];
            if cell.ques == 7 {
                5
            } else if cell.qdir != 0 {
                cell.qdir
            } else {
                -1
            }
        });
        Ok(format!("{}{}", encode::encode_border(b), cells))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_border_ques(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            if ca == "#" {
                cell.ques = 7;
            } else if let Some(n) = jsnum::integer(ca) {
                // pzprjs: `else if (+ca > 0) cell.qdir = +ca`; `.` and
                // non-positive tokens leave the cell empty.
                if n > 0 {
                    cell.qdir = n;
                }
            }
            Ok(())
        })?;
        decode_border_arrow_ans(r, b, d)?;
        fileio::decode_cell_qanssub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_border_ques(w, b);
        w.encode_cell(b, |cell| {
            if cell.ques == 7 {
                "# ".into()
            } else if cell.qdir != 0 {
                format!("{} ", cell.qdir)
            } else {
                ". ".into()
            }
        });
        encode_border_arrow_ans(w, b);
        fileio::encode_cell_qanssub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.ques == 0 || c.ques == 7)
                || !(0..=4).contains(&c.qdir)
                || !(0..=1).contains(&c.qans)
                || !(0..=4).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "fakearrow cell value outside empty 0 / no-cell 7, arrow 0-4, shade 0/1 or background 0-4",
            );
        }
        if b.borders.iter().any(|e| {
            !(e.ques == 0 || e.ques == 1)
                || !(0..=1).contains(&e.line)
                || !matches!(e.qsub, 0 | 2 | 11..=14)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "fakearrow border value outside wall 0/1, line 0/1 or aux mark 0/cross/u/d/l/r",
            );
        }
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData`: `adjustCellArrow` turns every
        // cell's `qdir`, then the `TURNFLIP` block turns border `qsub` 11–14
        // (the `u`/`d`/`l`/`r` aux marks) through `getTranslateDir`.
        for op in turn_ops(t) {
            for cell in &mut b.cells {
                if let Some(dir) = translate_dir(*op, cell.qdir) {
                    cell.qdir = dir;
                }
            }
            for border in &mut b.borders {
                if let Some(dir) = translate_dir(*op, border.qsub - 10) {
                    border.qsub = dir + 10;
                }
            }
        }
    }
}

/// `decodeBorderArrowAns`: border line, or `-n` for a line plus cross
/// (`qsub` 2), with an optional trailing `u`/`d`/`l`/`r` aux mark (`qsub`
/// 11–14). Any other lowercase letter is stripped without setting a mark.
fn decode_border_arrow_ans(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_border(b, d, |border, ca| {
        let mut ca = ca.to_string();
        if let Some(lca) = ca.chars().last() {
            if lca.is_ascii_lowercase() {
                border.qsub = match lca {
                    'u' => 11,
                    'd' => 12,
                    'l' => 13,
                    'r' => 14,
                    _ => border.qsub,
                };
                ca.pop();
            }
        }
        if !ca.is_empty() && ca != "0" {
            if ca.starts_with('-') {
                border.line = -fileio::int(&ca)? - 1;
                border.qsub = 2;
            } else {
                border.line = fileio::int(&ca)?;
            }
        }
        Ok(())
    })
}

/// `encodeBorderArrowAns`: `-1-line` for a line plus cross, the line itself
/// otherwise, then the `u`/`d`/`l`/`r` aux mark; `0` when all are empty.
fn encode_border_arrow_ans(w: &mut FileWriter, b: &Board) {
    w.encode_border(b, |border| {
        let mut ca = String::new();
        if border.qsub == 2 {
            ca.push_str(&(-1 - border.line).to_string());
        } else if border.line > 0 {
            ca.push_str(&border.line.to_string());
        }
        if let Some(dir) = match border.qsub {
            11 => Some('u'),
            12 => Some('d'),
            13 => Some('l'),
            14 => Some('r'),
            _ => None,
        } {
            ca.push(dir);
        }
        if ca.is_empty() {
            "0 ".into()
        } else {
            format!("{ca} ")
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

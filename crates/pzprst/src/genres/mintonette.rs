//! Mintonette (pzprjs `variety/firefly.js`, pid `mintonette`).
//!
//! Cells hold a clue number in `qnum` (0–999, `?` is -2); the shared codecs
//! also accept a direction in `qdir` (0 none, 1 up, 2 down, 3 left, 4 right)
//! and `qnum` -3 for `#`. The URL is `decodeNumber16` and stores numbers only
//! (no directions); FileIO stores `qdir,qnum` per cell and a single loop on the
//! borders (`decodeBorderLine`). Rotations/reflections rotate `qdir`
//! (`adjustNumberArrow` in `BoardExec.adjustBoardData`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Mintonette;

struct Mintonette;

impl Genre for Mintonette {
    fn pid(&self) -> &'static str {
        "mintonette"
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
        encode::decode_number16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_cell_direc_qnum(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_direc_qnum(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cell.maxnum` is 999, `minnum` 0.
        if b.cells.iter().any(|c| {
            !(c.qnum == -3
                || c.qnum == -2
                || c.qnum == -1
                || (0..=999).contains(&c.qnum))
                || !(0..=4).contains(&c.qdir)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "mintonette cell value outside clue 0-999/?/# or direction 0-4",
            );
        }
        if b.cells
            .iter()
            .any(|c| c.qdir != 0 || c.qnum == -3)
        {
            d.push(
                DiagnosticCode::LossyUrl,
                "mintonette directions and # are dropped from the URL: it stores numbers only",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "mintonette border value outside line 0/1, cross -1",
            );
        }
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` → `adjustNumberArrow`
        // → `adjustCellArrowField(key, d, "qdir")`.
        adjust_cell_qdirs(b, t);
    }
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

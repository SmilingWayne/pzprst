//! Battenberg Painting (pzprjs `variety/batten.js`, pid `batten`).
//!
//! Clue numbers sit in the excell block above every column and to the left of
//! every row (one entry per line, `hasexcell` 1), and the checkerboard symbols
//! are black dots on the cross grid (`hascross` 1, inner crosses only). The URL
//! is `decodeCrossMark` followed by `decodeNumber16ExCell`; FileIO stores
//! `decodeCrossNum` followed by the combined cell/excell layer
//! (`decodeCellExCell`). Board transforms replay pzprjs's
//! `BoardExec.adjustExCellTopLeft_1`/`_2` pair.

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Batten;

struct Batten;

impl Genre for Batten {
    fn pid(&self) -> &'static str {
        "batten"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn hascross(&self) -> u8 {
        1
    }

    fn hasexcell(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_cross_mark(b, body, d);
        encode::decode_number16_excell(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_cross_mark(b);
        body.push_str(&encode::encode_number16_excell(b));
        Ok(body)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cross_num(r, b, d)?;
        fileio::decode_cell_excell(r, b, d, |obj, group, ca| {
            if ca == "." {
                return Ok(());
            }
            if group == Group::ExCell {
                let mut token = ca;
                if let Some(rest) = token.strip_prefix('c') {
                    obj.qcmp = 1;
                    token = rest;
                }
                obj.qnum = fileio::int(token)?;
            } else if group == Group::Cell {
                if ca == "#" {
                    obj.qans = 1;
                } else if ca == "+" {
                    obj.qsub = 1;
                }
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cross_num(w, b);
        fileio::encode_cell_excell(w, b, |obj, group| {
            if group == Group::ExCell && obj.qnum != -1 {
                let cmp = if obj.qcmp != 0 { "c" } else { "" };
                return format!("{cmp}{} ", obj.qnum);
            }
            if group == Group::Cell {
                if obj.qans == 1 {
                    return "# ".into();
                } else if obj.qsub == 1 {
                    return "+ ".into();
                }
            }
            ". ".into()
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| c.qnum != -1 || !(0..=1).contains(&c.qans) || !(0..=1).contains(&c.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "batten cell value outside no clue with answer mark #/+",
            );
        }
        // `Cross.maxnum` is 1 (`minnum` 1); -1 is empty and FileIO can store
        // `?` (-2) or a explicit 0.
        if b.crosses
            .iter()
            .any(|cross| !(-2..=1).contains(&cross.qnum))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "batten cross value outside empty / ? / 0-1",
            );
        }
        // `ExCell`: minnum 0, `ExCell.maxnum` is the length of the column (a
        // top clue) or row (a left clue) it belongs to; -2 is `?`.
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let qc = b.cols() * b.geom.excell_rows;
        let bad = b.excells.iter().enumerate().any(|(i, ex)| {
            let max = if i < qc { rows } else { cols };
            !(-2..=max).contains(&ex.qnum) || !(0..=1).contains(&ex.qcmp)
        });
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "batten excell value outside empty / ? / 0-maxnum or qcmp 0/1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`: the shared `adjustExCellTopLeft_1`
    /// only captures the top-block and left-block numbers, so with pzprst's
    /// once-per-transform hook the capture and its `_2` rewrite are replayed
    /// together in `adjust_board_data`.
    fn adjust_board_data_before(&self, _t: Transform, _b: &mut Board) {}

    /// pzprjs `BoardExec.adjustBoardData2`: the shared `adjustExCellTopLeft_2`
    /// (without `preserve`). Replayed for every pzprjs operation of the
    /// composed transform in `tools/gen_fixtures.js` order: the core already
    /// ran `turnflipGroup("excell")` on the whole board, so undo that pass
    /// first, then apply the capture/move/rewrite net effect of each
    /// operation. Batten has one excell per column and row, so a flip only
    /// mirrors its own block (the `_2` reversal is the identity) and a turn
    /// moves the captured columns into the new left rows (reversed) and the
    /// captured rows into the new top columns (mirrored).
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        let ops = crate::board::Geometry::op_sequence(t);
        if ops.is_empty() || b.geom.hasexcell != 1 || b.excells.is_empty() {
            return;
        }
        let mut flat: Vec<i32> = b.excells.iter().map(|excell| excell.qnum).collect();
        let (mut cols, mut rows) = (b.cols(), b.rows());
        // Undo `turnflipGroup`: a flip mirrors its own block (an involution) and
        // a turn only swaps the dimensions, leaving the object ids (and this
        // flat order) alone, like pzprjs `setposAll`.
        for &op in ops.iter().rev() {
            match op {
                Transform::FlipX => mirror_top(&mut flat, cols, rows),
                Transform::FlipY => mirror_left(&mut flat, cols, rows),
                _ => {}
            }
            if op.swaps_axes() {
                std::mem::swap(&mut cols, &mut rows);
            }
        }
        // `flat` is the pre-transform board again. `turnflipGroup` mirrors the
        // block of a flip and moves no excell on a turn; `adjustExCellTopLeft_2`
        // then writes the captured top columns into the new left rows and the
        // captured left rows into the new top columns.
        for &op in ops {
            flat = match op {
                Transform::FlipX => mirror_top_copy(&flat, cols, rows),
                Transform::FlipY => mirror_left_copy(&flat, cols, rows),
                Transform::Rot90 => turn(&flat, cols, rows, true),
                Transform::Rot270 => turn(&flat, cols, rows, false),
                _ => flat,
            };
            if op.swaps_axes() {
                std::mem::swap(&mut cols, &mut rows);
            }
        }
        for (excell, qnum) in b.excells.iter_mut().zip(flat) {
            excell.qnum = qnum;
        }
    }
}

/// Split the flat excell vector into the top and left clue blocks (one entry
/// per column and row, in `Board.excells` id order).
fn blocks(flat: &[i32], cols: usize, rows: usize) -> (Vec<i32>, Vec<i32>) {
    let top = flat.get(..cols).map(<[i32]>::to_vec).unwrap_or_default();
    let left = flat
        .get(cols..cols + rows)
        .map(<[i32]>::to_vec)
        .unwrap_or_default();
    (top, left)
}

fn join(cols: usize, rows: usize, top: &[i32], left: &[i32]) -> Vec<i32> {
    let mut out = vec![-1; cols + rows];
    out[..top.len()].copy_from_slice(top);
    out[cols..cols + left.len()].copy_from_slice(left);
    out
}

/// `turnflipGroup("excell", FLIPX)`: reverse the top block's columns.
fn mirror_top(flat: &mut [i32], cols: usize, rows: usize) {
    let op = mirror_top_copy(flat, cols, rows);
    flat.copy_from_slice(&op);
}

/// `turnflipGroup("excell", FLIPY)`: reverse the left block's rows.
fn mirror_left(flat: &mut [i32], cols: usize, rows: usize) {
    let op = mirror_left_copy(flat, cols, rows);
    flat.copy_from_slice(&op);
}

fn mirror_top_copy(flat: &[i32], cols: usize, rows: usize) -> Vec<i32> {
    let (mut top, left) = blocks(flat, cols, rows);
    top.reverse();
    join(cols, rows, &top, &left)
}

fn mirror_left_copy(flat: &[i32], cols: usize, rows: usize) -> Vec<i32> {
    let (top, mut left) = blocks(flat, cols, rows);
    left.reverse();
    join(cols, rows, &top, &left)
}

/// One pzprjs turn together with its `adjustExCellTopLeft_2` rewrite: the
/// captured top columns become the new left rows and the captured left rows
/// become the new top columns. `TURNR` reverses `qnumh` (the top block) and
/// `TURNL` reverses `qnumw` (the left block).
fn turn(flat: &[i32], cols: usize, rows: usize, clockwise: bool) -> Vec<i32> {
    let (top, left) = blocks(flat, cols, rows);
    let (new_cols, new_rows) = (rows, cols);
    let mut new_top = Vec::with_capacity(new_cols);
    let mut new_left = Vec::with_capacity(new_rows);
    for i in 0..new_rows {
        let value = if clockwise {
            top.get(i).copied().unwrap_or(-1)
        } else {
            top.get(cols - 1 - i).copied().unwrap_or(-1)
        };
        new_left.push(value);
    }
    for j in 0..new_cols {
        let value = if clockwise {
            left.get(rows - 1 - j).copied().unwrap_or(-1)
        } else {
            left.get(j).copied().unwrap_or(-1)
        };
        new_top.push(value);
    }
    join(new_cols, new_rows, &new_top, &new_left)
}

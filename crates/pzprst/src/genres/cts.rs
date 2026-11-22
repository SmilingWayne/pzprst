//! Cross the Streams (pzprjs `variety/nonogram.js`, pid `cts`).
//!
//! A nonogram whose clue numbers live in the excell block above every column
//! and to the left of every row, `excellRows = (rows + 1) >> 1` by
//! `excellCols = (cols + 1) >> 1` entries each (`hasexcell` 1). The URL is the
//! flushed `decodeNumber16ExCell` layer; FileIO stores one combined
//! cell/excell layer (`decodeCellExCell`) where an excell is a clue number with
//! an optional `c` (`qcmp`) prefix and a cell is only its answer mark
//! (`#` shaded, `+` the sub mark).
//!
//! `cts` differs from its sibling `nonogram` only in the excell value range:
//! `"ExCell@cts"` sets `minnum: 0` and allows a `?` (`disInputHatena: false`).

use super::Genre;
use crate::board::{Board, Geometry, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Cts;

struct Cts;

impl Genre for Cts {
    fn pid(&self) -> &'static str {
        "cts"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn hasexcell(&self) -> u8 {
        1
    }

    /// pzprjs `Board.excellRows`.
    fn excell_rows(&self, _cols: usize, rows: usize) -> usize {
        row_count(rows)
    }

    /// pzprjs `Board.excellCols`.
    fn excell_cols(&self, cols: usize, _rows: usize) -> usize {
        col_count(cols)
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16_excell(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16_excell_flushed(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
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
                "cts cell value outside no clue with answer mark #/+",
            );
        }
        // `"ExCell@cts"`: minnum 0, `ExCell.maxnum` is the length of the
        // column (a top clue) or row (a left clue) it belongs to; -2 is `?`.
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let qc = b.cols() * b.geom.excell_rows;
        let bad = b.excells.iter().enumerate().any(|(i, ex)| {
            let max = if i < qc { rows } else { cols };
            !(-2..=max).contains(&ex.qnum) || !(0..=1).contains(&ex.qcmp)
        });
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "cts excell value outside empty / ? / 0-maxnum or qcmp 0/1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`: the shared `adjustExCellTopLeft_1`
    /// only captures the top-block and left-block numbers, so with pzprst's
    /// once-per-transform hook the capture and its `_2` rewrite are replayed
    /// together in `adjust_board_data`.
    fn adjust_board_data_before(&self, _t: Transform, _b: &mut Board) {}

    /// pzprjs `BoardExec.adjustBoardData2`: the shared `adjustExCellTopLeft_2`
    /// (without `preserve`, so a flip also reverses the captured clue lists).
    /// Replayed for every pzprjs operation of the composed transform in
    /// `tools/gen_fixtures.js` order: the core already ran
    /// `turnflipGroup("excell")` on the whole board, so undo that pass first,
    /// then apply the capture/move/rewrite net effect of each operation.
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        let ops = Geometry::op_sequence(t);
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
        // `flat` is the pre-transform board again. Replay the pzprjs calls:
        // `adjustExCellTopLeft_1` captures, `turnflipGroup` moves, and
        // `adjustExCellTopLeft_2` rewrites. Without `preserve` each flip also
        // reverses the captured lists, which is exactly the content reversal
        // of the block that `turnflipGroup` did not move.
        for &op in ops {
            flat = match op {
                Transform::FlipX => flip_x(&flat, cols, rows),
                Transform::FlipY => flip_y(&flat, cols, rows),
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

/// pzprjs `Board.excellRows`.
fn row_count(rows: usize) -> usize {
    (rows + 1) >> 1
}

/// pzprjs `Board.excellCols`.
fn col_count(cols: usize) -> usize {
    (cols + 1) >> 1
}

/// The top and left clue blocks of the excell grid at a `cols` x `rows` board:
/// `top[col]` holds a column's numbers and `left[row]` a row's, each ordered
/// innermost first, which is the order of the ids in `Board.excells`.
fn blocks(flat: &[i32], cols: usize, rows: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let exrows = row_count(rows);
    let excols = col_count(cols);
    let qc = cols * exrows;
    let top = flat
        .get(..qc)
        .map(|slice| slice.chunks(exrows).map(|chunk| chunk.to_vec()).collect())
        .unwrap_or_default();
    let left = flat
        .get(qc..)
        .map(|slice| slice.chunks(excols).map(|chunk| chunk.to_vec()).collect())
        .unwrap_or_default();
    (top, left)
}

fn write_blocks(flat: &mut [i32], cols: usize, rows: usize, top: &[Vec<i32>], left: &[Vec<i32>]) {
    let exrows = row_count(rows);
    let excols = col_count(cols);
    let qc = cols * exrows;
    for (col, values) in top.iter().enumerate() {
        let start = col * exrows;
        if let Some(dst) = flat.get_mut(start..start + exrows) {
            if dst.len() == values.len() {
                dst.copy_from_slice(values);
            }
        }
    }
    for (row, values) in left.iter().enumerate() {
        let start = qc + row * excols;
        if let Some(dst) = flat.get_mut(start..start + excols) {
            if dst.len() == values.len() {
                dst.copy_from_slice(values);
            }
        }
    }
}

/// pzprjs `turnflipGroup("obj", FLIPX)` / `adjustExCellTopLeft_2(FLIPX)` for
/// the whole board: the top block's columns are mirrored and every left row's
/// clue list is reversed.
fn flip_x(flat: &[i32], cols: usize, rows: usize) -> Vec<i32> {
    let (mut top, mut left) = blocks(flat, cols, rows);
    top.reverse();
    for values in &mut left {
        values.reverse();
    }
    let mut out = flat.to_vec();
    write_blocks(&mut out, cols, rows, &top, &left);
    out
}

/// pzprjs `turnflipGroup("obj", FLIPY)` / `adjustExCellTopLeft_2(FLIPY)`:
/// every top column's clue list is reversed and the left block's rows are
/// mirrored.
fn flip_y(flat: &[i32], cols: usize, rows: usize) -> Vec<i32> {
    let (mut top, mut left) = blocks(flat, cols, rows);
    for values in &mut top {
        values.reverse();
    }
    left.reverse();
    let mut out = flat.to_vec();
    write_blocks(&mut out, cols, rows, &top, &left);
    out
}

/// `turnflipGroup("excell", FLIPX)` for the top block: reverse the columns.
fn mirror_top(flat: &mut [i32], cols: usize, rows: usize) {
    let (mut top, left) = blocks(flat, cols, rows);
    top.reverse();
    write_blocks(flat, cols, rows, &top, &left);
}

/// `turnflipGroup("excell", FLIPY)` for the left block: reverse the rows.
fn mirror_left(flat: &mut [i32], cols: usize, rows: usize) {
    let (top, mut left) = blocks(flat, cols, rows);
    left.reverse();
    write_blocks(flat, cols, rows, &top, &left);
}

/// One pzprjs turn: `turnflipGroup` moves no excell, and
/// `adjustExCellTopLeft_2` rewrites both blocks from the captured lists. For a
/// right turn the captured top columns become the new left rows with their
/// lists reversed, and the captured left rows become the new top columns
/// (`TURNR` reverses `qnumh` only, so `qnumw` is written back as captured); a
/// left turn swaps the roles of the two lists.
fn turn(flat: &[i32], cols: usize, rows: usize, clockwise: bool) -> Vec<i32> {
    let (top, left) = blocks(flat, cols, rows);
    let (new_cols, new_rows) = (rows, cols);
    let mut new_top = Vec::with_capacity(new_cols);
    let mut new_left = Vec::with_capacity(new_rows);
    for i in 0..new_rows {
        let mut values = if clockwise {
            top.get(i).cloned().unwrap_or_default()
        } else {
            top.get(cols - 1 - i).cloned().unwrap_or_default()
        };
        if clockwise {
            values.reverse();
        }
        new_left.push(values);
    }
    for j in 0..new_cols {
        let mut values = if clockwise {
            left.get(rows - 1 - j).cloned().unwrap_or_default()
        } else {
            left.get(j).cloned().unwrap_or_default()
        };
        if !clockwise {
            values.reverse();
        }
        new_top.push(values);
    }
    let mut out = vec![-1; flat.len()];
    write_blocks(&mut out, new_cols, new_rows, &new_top, &new_left);
    out
}

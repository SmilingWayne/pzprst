//! Snake (pzprjs `variety/snake.js`, pid `snake`).
//!
//! The URL is the `decodeCircle` cell layer (clue 1/2) followed by the
//! `decodeNumber16ExCell` excell layer. FileIO stores one combined cell/excell
//! layer (`decodeCellExCell`): a cell packs its clue and answer mark as
//! `qnum | qans * 4 | qsub * 8`, an excell stores a clue number with an optional
//! `c` (`qcmp`) prefix. The top row and left column are excells
//! (`hasexcell` 1); pzprjs's `adjustExCellTopLeft` hooks redistribute them
//! under board transforms.

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Snake;

struct Snake;

impl Genre for Snake {
    fn pid(&self) -> &'static str {
        "snake"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn hasexcell(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_circle(b, body, d);
        encode::decode_number16_excell(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_circle(b)?;
        body.push_str(&encode::encode_number16_excell(b));
        Ok(body)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_excell(r, b, d, |obj, group, ca| {
            if ca == "." {
                return Ok(());
            }
            if group == Group::Cell {
                let val = fileio::int(ca)?;
                let ques = val & 3;
                let ans = (val & 12) >> 2;
                // pzprjs stores 0 here; pzprst uses -1 for "no clue" so that
                // FileIO round-trips (0 encodes the same way as -1).
                obj.qnum = if ques > 0 { ques } else { -1 };
                if ans == 1 {
                    obj.qans = 1;
                } else if ans == 2 {
                    obj.qsub = 1;
                }
            } else if group == Group::ExCell {
                let mut token = ca;
                if let Some(rest) = token.strip_prefix('c') {
                    obj.qcmp = 1;
                    token = rest;
                }
                obj.qnum = fileio::int(token)?;
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_excell(w, b, |obj, group| {
            if group == Group::ExCell {
                if obj.qnum != -1 {
                    let cmp = if obj.qcmp != 0 { "c" } else { "" };
                    return format!("{cmp}{} ", obj.qnum);
                }
                ". ".into()
            } else {
                let mut i = obj.qnum.max(0);
                if obj.qans == 1 {
                    i |= 4;
                } else if obj.qsub == 1 {
                    i |= 8;
                }
                format!("{i} ")
            }
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(-1..=2).contains(&c.qnum)
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "snake cell value outside empty / circle 1-2 or its answer marks",
            );
        }
        // `ExCell.maxnum`: the top row allows `rows`, the left column `cols`.
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let bad = b.excells.iter().enumerate().any(|(i, ex)| {
            let max = if i < b.cols() { rows } else { cols };
            !(-2..=max).contains(&ex.qnum) || !(0..=1).contains(&ex.qcmp)
        });
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "snake excell value outside empty / ? / 0-maxnum or qcmp 0/1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`: snake's `adjustExCellTopLeft_1`
    /// only captures the top-row and left-column numbers, so with pzprst's
    /// once-per-transform hook the capture and its `_2` redistribution are
    /// replayed together in `adjust_board_data`.
    fn adjust_board_data_before(&self, _t: Transform, _b: &mut Board) {}

    /// pzprjs `BoardExec.adjustBoardData2` (`adjustExCellTopLeft_1`/`_2`),
    /// replayed for every pzprjs operation of the composed transform in the
    /// order `tools/gen_fixtures.js` composes them.
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        if b.geom.hasexcell != 1 || b.excells.is_empty() {
            return;
        }
        let (cols, rows) = (b.cols(), b.rows());
        // Board::transform already ran `turnflipGroup("excell", …)`; undo its
        // line reversal to recover pzprjs's adjustBoardData input.
        let mut vals: Vec<i32> = b.excells.iter().map(|o| o.qnum).collect();
        // Dimensions before the transform (turns swap them).
        let (mut c, mut r) = if t.swaps_axes() {
            (rows, cols)
        } else {
            (cols, rows)
        };
        // The only turnflipGroup step that reorders excells is the final flip
        // of a transpose/antitranspose, applied in the transformed dimensions.
        match t {
            Transform::FlipX => vals[..c].reverse(),
            Transform::FlipY => vals[c..].reverse(),
            Transform::Transpose | Transform::AntiTranspose => vals[cols..].reverse(),
            _ => {}
        }
        let mut top: Vec<i32> = vals[..c].to_vec();
        let mut left: Vec<i32> = vals[c..].to_vec();
        for &op in operations(t) {
            // adjustExCellTopLeft_1: capture every column's / row's numbers.
            let qnumw = left.clone();
            let qnumh = top.clone();
            // turnflipGroup("excell", key, d) + setposAll. Snake has one
            // excell per row/column, so a flip reverses its own line and a
            // turn leaves the object order alone while the dimensions swap.
            match op {
                Transform::FlipX => top.reverse(),
                Transform::FlipY => left.reverse(),
                Transform::Rot90 | Transform::Rot270 => std::mem::swap(&mut c, &mut r),
                _ => {}
            }
            // adjustExCellTopLeft_2: write the captured numbers back.
            match op {
                Transform::FlipX => left = qnumw,
                Transform::FlipY => top = qnumh,
                Transform::Rot90 => {
                    top = qnumw.iter().rev().copied().collect();
                    left = qnumh;
                }
                Transform::Rot270 => {
                    top = qnumw;
                    left = qnumh.iter().rev().copied().collect();
                }
                _ => {}
            }
        }
        if top.len() != cols || left.len() != rows {
            return;
        }
        for (i, v) in top.iter().enumerate() {
            b.excells[i].qnum = *v;
        }
        for (j, v) in left.iter().enumerate() {
            b.excells[cols + j].qnum = *v;
        }
    }
}

/// The pzprjs board operations a `Transform` stands for, in
/// `tools/gen_fixtures.js` order.
fn operations(t: Transform) -> &'static [Transform] {
    match t {
        Transform::Identity => &[],
        Transform::Rot90 => &[Transform::Rot90],
        Transform::Rot180 => &[Transform::Rot90, Transform::Rot90],
        Transform::Rot270 => &[Transform::Rot270],
        Transform::FlipX => &[Transform::FlipX],
        Transform::FlipY => &[Transform::FlipY],
        Transform::Transpose => &[Transform::Rot270, Transform::FlipY],
        Transform::AntiTranspose => &[Transform::Rot90, Transform::FlipY],
    }
}

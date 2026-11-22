//! Ant Mill (pzprjs `variety/scrin.js`, pid `antmill`).
//!
//! The URL stores two border marks per cell except the last: `right * 3 +
//! bottom`, where `right` is the `ques` of the cell's right border and
//! `bottom` the `ques` of its bottom border. FileIO stores the border `ques`
//! layer and the cell answer layer.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Antmill;

struct Antmill;

/// `cell.adjborder.right` (`dx`/`dy` = 1/0) or `cell.adjborder.bottom` (0/1);
/// `None` where pzprjs has a null border.
fn cell_border(b: &Board, c: usize, dx: i32, dy: i32) -> Option<usize> {
    let (bx, by) = b.geom.cell_pos(c);
    b.geom.border_index(bx + dx, by + dy)
}

impl Genre for Antmill {
    fn pid(&self) -> &'static str {
        "antmill"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        // pzprjs `genericDecodeNumber16(bd.cell.length - 1, …)`.
        let length = b.cells.len().saturating_sub(1);
        encode::generic_decode_number16(length, body, d, |c, val| {
            if let Some(i) = cell_border(b, c, 1, 0) {
                b.borders[i].ques = val / 3;
            }
            if let Some(i) = cell_border(b, c, 0, 1) {
                b.borders[i].ques = val % 3;
            }
        })
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let length = b.cells.len().saturating_sub(1);
        Ok(encode::generic_encode_number16(length, |c| {
            let r = cell_border(b, c, 1, 0).map_or(0, |i| b.borders[i].ques);
            let bottom = cell_border(b, c, 0, 1).map_or(0, |i| b.borders[i].ques);
            if r == 0 && bottom == 0 {
                -1
            } else {
                r * 3 + bottom
            }
        }))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_border(b, d, |border, ca| {
            border.ques = fileio::int(ca)?;
            Ok(())
        })?;
        fileio::decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_border(b, |border| format!("{} ", border.ques));
        fileio::encode_cell_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            c.qnum != -1
                || c.ques != 0
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "antmill cell value outside empty, shading 0/1, mark 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| e.line != 0 || e.qans != 0 || e.qsub != 0)
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "antmill border value outside ques marks",
            );
        }
    }
}

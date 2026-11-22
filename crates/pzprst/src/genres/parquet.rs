//! Parquet (pzprjs `variety/parquet.js`, pid `parquet`).
//!
//! The URL has no cell clues: `genericDecodeNumber16` stores one value per cell
//! as `3 * right border ques + bottom border ques`, so each value is 0–8 and the
//! right/bottom borders of the last column/row are skipped (their pzprjs
//! `adjborder` is the null object, which reads as 0). FileIO stores the border
//! `ques` layer and the shaded-cell answer layer (`decodeCellAns`).

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Parquet;

struct Parquet;

impl Genre for Parquet {
    fn pid(&self) -> &'static str {
        "parquet"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let n = b.cells.len();
        encode::generic_decode_number16(n, body, d, |c, val| {
            let (bx, by) = b.geom.cell_pos(c);
            if let Some(border) = b.obj_mut(Group::Border, bx + 1, by) {
                border.ques = val / 3;
            }
            if let Some(border) = b.obj_mut(Group::Border, bx, by + 1) {
                border.ques = val % 3;
            }
        })
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::generic_encode_number16(b.cells.len(), |c| {
            let (bx, by) = b.geom.cell_pos(c);
            let right = b.obj(Group::Border, bx + 1, by).map_or(0, |e| e.ques);
            let bottom = b.obj(Group::Border, bx, by + 1).map_or(0, |e| e.ques);
            right * 3 + bottom
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
        // -2 is what a `.` in the number layer stores in the bottom border
        // (`-2 % 3`); pzprjs reproduces it, so it is not out of range here.
        if b.borders.iter().any(|e| !matches!(e.ques, -2 | 0 | 1 | 2)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "parquet border value outside tile edge 0/1/2",
            );
        }
        if b.cells
            .iter()
            .any(|c| !matches!(c.qans, 0 | 1) || !matches!(c.qsub, 0 | 1))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "parquet cell answer value outside shading 0/1 and mark 0/1",
            );
        }
    }
}

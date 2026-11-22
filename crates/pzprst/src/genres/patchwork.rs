//! Patchwork (pzprjs `variety/patchwork.js`, pid `patchwork`).
//!
//! Cells carry the clue numbers only (`Number16`); `Cell` allows 0 up to
//! `cols * rows - 1` and `minnum` 0. FileIO stores the clue/shading/sub layer
//! (`decodeCellQnumAns`) and the border answer markings (`decodeBorderAns`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Patchwork;

struct Patchwork;

impl Genre for Patchwork {
    fn pid(&self) -> &'static str {
        "patchwork"
    }

    fn hasborder(&self) -> u8 {
        1
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
        fileio::decode_cell_qnum_ans(r, b, d)?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum_ans(w, b);
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows()) as i32 - 1;
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "patchwork cell value outside empty / ? / clue 0-cols*rows-1",
            );
        }
        if b.cells
            .iter()
            .any(|c| !matches!(c.qans, 0 | 1) || !matches!(c.qsub, 0 | 1))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "patchwork cell answer value outside shading 0/1 and mark 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| !matches!(e.qans, 0 | 1) || !matches!(e.qsub, 0 | 1))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "patchwork border answer value outside 0/1",
            );
        }
    }
}

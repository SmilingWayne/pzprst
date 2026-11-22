//! Yin-Yang (pzprjs `variety/yinyang.js`, pid `yinyang`).
//!
//! Question data is one circle per cell: `qnum` 1 is a white circle, 2 a black
//! one. The URL stores those values with `decodeCircle`/`encodeCircle`; FileIO
//! also stores the solved shading as an answer number and the usual cell marks.
//! The board has inner borders (`hasborder` 1) used only as player memos.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Yinyang;

struct Yinyang;

impl Genre for Yinyang {
    fn pid(&self) -> &'static str {
        "yinyang"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_circle(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        encode::encode_circle(b)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        for cell in &b.cells {
            if cell.qnum == -2 {
                d.push(
                    DiagnosticCode::LossyUrl,
                    "an unknown-colour circle is dropped from a yinyang URL",
                );
            } else if !matches!(cell.qnum, -1 | 1 | 2) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    "yinyang cell value outside empty / white 1 / black 2",
                );
            }
            if !matches!(cell.anum, -1 | 1 | 2)
                || !(0..=4).contains(&cell.qsub)
                || !(0..=1).contains(&cell.qcmp)
            {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    "yinyang answer value outside number 1/2, sub 0-4, cmp 0/1",
                );
            }
        }
    }
}

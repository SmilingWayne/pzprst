//! Tentaisho (pzprjs `variety/tentaisho.js`, pid `tentaisho`).
//!
//! The URL is the shared dot layer (values 1–2 on cells, borders and crosses)
//! followed by the shared `decodeEmpty` layer: cells with `ques` 7. FileIO
//! stores the dot grid (an `X` on a piece whose `ques` is 7), the border answer
//! layer and the cell `qsub` layer.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Tentaisho;

struct Tentaisho;

impl Genre for Tentaisho {
    fn pid(&self) -> &'static str {
        "tentaisho"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_dot(b, body, d, |dot, _, val| dot.qnum = val);
        encode::decode_empty(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_dot(b),
            encode::encode_empty(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_dot_file(r, b, d, |dot, _, val| dot.qnum = val)?;
        fileio::decode_border_ans(r, b, d)?;
        fileio::decode_cell_qsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_dot_file(w, b);
        fileio::encode_border_ans(w, b);
        fileio::encode_cell_qsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let dots = || b.cells.iter().chain(&b.borders).chain(&b.crosses);
        if dots().any(|dot| !matches!(dot.qnum, -2..=9) || !matches!(dot.ques, 0 | 7)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tentaisho dot value outside empty / ? / 1-9, or X mark outside 0/7",
            );
        }
        if dots().any(|dot| dot.qnum == -2) {
            d.push(
                DiagnosticCode::LossyUrl,
                "tentaisho '?' dots are dropped from the URL",
            );
        }
        if b.borders
            .iter()
            .chain(&b.crosses)
            .any(|dot| dot.ques == 7)
        {
            d.push(
                DiagnosticCode::LossyUrl,
                "tentaisho X marks on borders and crosses are dropped from the URL",
            );
        }
        if b.borders
            .iter()
            .any(|border| !matches!(border.qans, 0 | 1) || !matches!(border.qsub, 0 | 1))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tentaisho border answer outside none / marked / cross",
            );
        }
        if b.cells.iter().any(|cell| cell.qsub < 0) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tentaisho cell sub mark is negative",
            );
        }
    }
}

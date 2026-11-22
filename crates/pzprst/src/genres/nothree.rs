//! No Three (pzprjs `variety/nothree.js`, pid `nothree`).
//!
//! Dot values live on cells, borders and crosses; the URL stores values 1–2
//! only (`decodeDot`/`encodeDot`), FileIO stores the whole dot grid, then the
//! cell answer layer, then the border line layer when any border carries one.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &NoThree;

struct NoThree;

impl Genre for NoThree {
    fn pid(&self) -> &'static str {
        "nothree"
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
        encode::decode_dot(b, body, d, |dot, _, val| dot.qnum = val)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_dot(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_dot_file(r, b, d, |dot, _, val| dot.qnum = val)?;
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_dot_file(w, b);
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line_if_present(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let dots = || b.cells.iter().chain(&b.borders).chain(&b.crosses);
        if dots().any(|dot| !matches!(dot.qnum, -2..=9) || !matches!(dot.ques, 0 | 7)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "nothree dot value outside empty / ? / 1-9, or X mark outside 0/7",
            );
        }
        if dots().any(|dot| dot.qnum == -2 || dot.ques == 7) {
            d.push(
                DiagnosticCode::LossyUrl,
                "nothree '?' dots and X marks are dropped from the URL",
            );
        }
        if b.borders
            .iter()
            .any(|border| border.line < 0 || !matches!(border.qsub, 0 | 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "nothree border line outside 0.. or sub mark outside 0/-1",
            );
        }
    }
}

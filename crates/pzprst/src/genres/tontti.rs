//! Tonttiraja (pzprjs `variety/tontti.js`, pid `tontti`).
//!
//! The URL is the `decodeNumber16` clue layer; FileIO stores the clue layer,
//! the border line layer and the cell background (`qsub`) layer. The border and
//! cross grids extend to the frame (`hasborder`/`hascross` 2) and pzprjs
//! allocates excell cells on all four sides (`hasexcell` 2, for its LineGraph);
//! neither crosses nor excells are serialized. `Cell.maxnum` is the board area.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Tontti;

struct Tontti;

impl Genre for Tontti {
    fn pid(&self) -> &'static str {
        "tontti"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn hasexcell(&self) -> u8 {
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
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_border_line(r, b, d)?;
        fileio::decode_cell_qsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_border_line(w, b);
        fileio::encode_cell_qsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum`: `board.cols * board.rows`; `Cell.minnum` is the default 1.
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(0..=2).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("tontti cell value outside empty / ? / clue 1-{maxnum} or background 0-2"),
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tontti border value outside line 0/1 or cross mark",
            );
        }
    }
}

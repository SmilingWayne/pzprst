//! U-Turns (pzprjs `variety/kusabi.js`, pid `uturns`).
//!
//! The same board as Kusabi Link: clues 1–3 in the cells (`numberAsObject`)
//! and the shared line/cross layer on the borders. `Cell@uturns` only turns off
//! hatena input (`disInputHatena`, so `?` is a FileIO-only value) and
//! `Border@uturns` only sets `enableLineNG`; neither is board data, and the
//! URL/FileIO codecs are the `kusabi.js` `Encode`/`FileIO` sections unchanged.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Uturns;

struct Uturns;

impl Genre for Uturns {
    fn pid(&self) -> &'static str {
        "uturns"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number10(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        encode::encode_number10(b)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !matches!(c.qnum, -2 | -1 | 1..=3))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "uturns cell value outside clue 1-3/?",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "uturns border value outside line 0/1 or cross -1",
            );
        }
    }
}

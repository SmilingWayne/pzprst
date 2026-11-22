//! Sukoro (pzprjs `variety/sukoro.js`, pid `sukoro`).
//!
//! `Cell@sukoro` caps clues at 4 (`maxnum: 4`, `minnum` 1 from the base
//! `Cell`), and `Encode@sukoro` stores them with `decodeNumber10` /
//! `encodeNumber10`. Sukoro-room (`sukororoom`) lives in the same file and
//! adds a `hasborder` wall layer; this module is the plain board only.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Sukoro;

struct Sukoro;

impl Genre for Sukoro {
    fn pid(&self) -> &'static str {
        "sukoro"
    }

    fn hasborder(&self) -> u8 {
        0
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
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !matches!(c.qnum, -2 | -1 | 1 | 2 | 3 | 4))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "sukoro cell value outside clue 1-4 or ?",
            );
        }
    }
}

//! Renban-Madoguchi (pzprjs `variety/renban.js`, pid `renban`).
//!
//! Cells hold a puzzle clue 1–999 (`minnum` 1, `maxnum` 999) and the FileIO
//! answer layer also stores `snum` sub-number arrays for `Cell.enableSubNumberArray`
//! cells; pzprst has no `snum`, and pzprjs only writes a `[...]` suffix when a
//! sub-number was set, so an empty array stays byte-identical. The URL stores
//! the border walls (`decodeBorder`) and then the clues (`decodeNumber16`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Renban;

struct Renban;

impl Genre for Renban {
    fn pid(&self) -> &'static str {
        "renban"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        encode::decode_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_border(b),
            encode::encode_number16(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_border_ques(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_border_ques(w, b);
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (1..=999).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "renban cell value outside empty / ? / clue 1-999",
            );
        }
        if b.borders.iter().any(|e| !(0..=1).contains(&e.ques)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "renban border value outside wall 0/1",
            );
        }
    }
}

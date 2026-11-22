//! Sukoro-room (pzprjs `variety/sukoro.js`, pid `sukororoom`).
//!
//! The same clue set as Sukoro (1–4, `?`) on a `hasborder` 1 board: the URL
//! writes the shared `encodeBorder` wall layer first and `encodeNumber10`
//! after it (`Encode@sukororoom`), and FileIO writes a `decodeBorderQues`
//! layer before the cell layers.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Sukororoom;

struct Sukororoom;

impl Genre for Sukororoom {
    fn pid(&self) -> &'static str {
        "sukororoom"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        encode::decode_number10(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_border(b),
            encode::encode_number10(b)?
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
            .any(|c| !matches!(c.qnum, -2 | -1 | 1 | 2 | 3 | 4))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "sukororoom cell value outside clue 1-4 or ?",
            );
        }
        if b.borders.iter().any(|e| !(0..=1).contains(&e.ques)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "sukororoom border value outside wall 0/1",
            );
        }
    }
}

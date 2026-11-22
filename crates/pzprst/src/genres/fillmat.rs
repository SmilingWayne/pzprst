//! Fillmat (pzprjs `variety/fillmat.js`, pid `fillmat`).
//!
//! Clues are 1–4; the tatami borders are answer data. The URL is the shared
//! `decodeNumber10`/`encodeNumber10`; `usotatami` shares the file but has its
//! own `Encode@usotatami` (number10or16) and is not part of this pid.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Fillmat;

struct Fillmat;

impl Genre for Fillmat {
    fn pid(&self) -> &'static str {
        "fillmat"
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
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell@fillmat` sets `maxnum: 4`; the shared minimum is 1.
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (1..=4).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "fillmat cell value outside empty / ? / clue 1-4",
            );
        }
        if b
            .borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "fillmat border value outside answer 0/1 or background 0/1",
            );
        }
    }
}

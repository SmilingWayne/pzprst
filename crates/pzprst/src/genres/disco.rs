//! Disconnection (pzprjs `variety/shimaguni.js`, pid `disco`).
//!
//! The grid is divided by border walls (`border.ques`); the URL stores only that
//! wall layer (`decodeBorder`/`encodeBorder`). FileIO also stores the shading
//! answer (`cell.qans`) and background marks (`cell.qsub`). The genre stores no
//! cell numbers, so `Cell.maxnum` never comes into play.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Disco;

struct Disco;

impl Genre for Disco {
    fn pid(&self) -> &'static str {
        "disco"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_border(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_border(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_border_ques(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_border_ques(w, b);
        fileio::encode_cell_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !(0..=1).contains(&c.qans) || !(0..=1).contains(&c.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "disco cell value outside shading 0/1, background 0/1",
            );
        }
        if b.borders.iter().any(|e| !(0..=1).contains(&e.ques)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "disco border value outside wall 0/1",
            );
        }
    }
}

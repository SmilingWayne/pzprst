//! Circles and Squares (pzprjs `variety/circlesquare.js`, pid `circlesquare`).
//! `Cell.numberAsObject` draws qnum 1/2 as circles; `disInputHatena` only
//! affects the UI, so FileIO can still carry a `?`.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Circlesquare;

struct Circlesquare;

impl Genre for Circlesquare {
    fn pid(&self) -> &'static str {
        "circlesquare"
    }

    fn hasborder(&self) -> u8 {
        0
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
        fileio::decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        for cell in &b.cells {
            if cell.qnum == -2 {
                d.push(
                    DiagnosticCode::LossyUrl,
                    "a ? clue is dropped from a circlesquare URL",
                );
            } else if !(cell.qnum == -1 || (1..=2).contains(&cell.qnum)) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    "circlesquare cell value outside empty / white 1 / black 2",
                );
                break;
            }
        }
        if b.cells
            .iter()
            .any(|c| !(0..=1).contains(&c.qans) || !(0..=1).contains(&c.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "circlesquare cell value outside shade 0/1 or cross 0/1",
            );
        }
    }
}

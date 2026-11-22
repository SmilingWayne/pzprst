//! Curving Road (pzprjs `variety/curving.js`, pid `curving`).
//!
//! Circles (`qnum` 1) are the only question data in a URL; shaded cells
//! (`qans`), background marks (`qsub`) and border lines are answer data stored
//! by FileIO only. Crosses are enabled (pzprjs's default `hascross` 2) but no
//! cross data is stored, so pzprst's board model covers the genre.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Curving;

struct Curving;

impl Genre for Curving {
    fn pid(&self) -> &'static str {
        "curving"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_binary(b, body, d, |cell| cell.qnum = 1)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_binary(b, |cell| cell.qnum == 1, false))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line_if_present(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        for cell in &b.cells {
            if cell.qnum == -2 {
                d.push(
                    DiagnosticCode::LossyUrl,
                    "a ? clue is dropped from a curving URL",
                );
            } else if !(cell.qnum == -1 || cell.qnum == 1) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    "curving cell value outside empty / circle 1",
                );
            }
        }
        if b.cells
            .iter()
            .any(|c| !(0..=1).contains(&c.qans) || !(0..=1).contains(&c.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "curving cell answer value outside shading 0/1, background 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "curving border value outside line 0/1, cross -1",
            );
        }
    }
}

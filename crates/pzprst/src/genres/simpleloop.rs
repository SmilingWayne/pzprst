//! Simple Loop (pzprjs `variety/simpleloop.js`, pid `simpleloop`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &SimpleLoop;

struct SimpleLoop;

impl Genre for SimpleLoop {
    fn pid(&self) -> &'static str {
        "simpleloop"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_empty(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_empty(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_empty(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_empty(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| !(c.ques == 0 || c.ques == 7)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "simpleloop cell value outside empty 0 / shaded 7",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "simpleloop border value outside line 0/1, cross -1",
            );
        }
    }
}

/// `decodeEmpty`: `*` marks a shaded cell.
fn decode_empty(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "*" {
            cell.ques = 7;
        }
        Ok(())
    })
}

/// `encodeEmpty`: `* ` for a shaded cell, `. ` otherwise.
fn encode_empty(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.ques == 7 {
            "* ".into()
        } else {
            ". ".into()
        }
    });
}

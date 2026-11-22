//! Light Up / Akari (pzprjs `variety/lightup.js`, pid `lightup`). `Cell.maxnum`
//! is 4 and `minnum` 0; `-2` is an unnumbered wall, `qans = 1` a lamp, `qsub = 1`
//! the FileIO `+` mark and `qcmp = 1` the "done" mark.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Lightup;

struct Lightup;

impl Genre for Lightup {
    fn pid(&self) -> &'static str {
        "lightup"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_4cell(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_4cell(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum_ans(r, b, d)?;
        decode_cell_qcmp(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum_ans(w, b);
        encode_cell_qcmp(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=4).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "lightup cell value outside clue 0-4/?, lamp, sub mark or completion mark",
            );
        }
    }
}

/// `decodeCellQcmp`: `-` marks a cell as completed.
fn decode_cell_qcmp(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "-" {
            cell.qcmp = 1;
        }
        Ok(())
    })
}

/// `encodeCellQcmp`: write the layer only when some cell is completed.
fn encode_cell_qcmp(w: &mut FileWriter, b: &Board) {
    if !b.cells.iter().any(|c| c.qcmp == 1) {
        return;
    }
    w.encode_cell(b, |cell| {
        if cell.qcmp == 1 {
            "- ".into()
        } else {
            ". ".into()
        }
    });
}

//! Oasis (pzprjs `variety/oasis.js`, pid `oasis`). Shade cells so the numbered
//! circles count the circles reachable through empty unshaded cells, and the
//! unshaded area is connected without 2x2 blocks. Borders are `hasborder` 1;
//! the extra `+`/`-`/`1` answer marks are stored by FileIO only.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Oasis;

struct Oasis;

impl Genre for Oasis {
    fn pid(&self) -> &'static str {
        "oasis"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        decode_cell_qanssubcmp(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        encode_cell_qanssubcmp(w, b);
        fileio::encode_border_line_if_present(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows()) as i32 - 1;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "oasis cell value outside clue 1..cols*rows-1 / ? or answer mark 0-1",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "oasis border value outside line 0/1, cross -1",
            );
        }
    }
}

/// `decodeCellQanssubcmp`: shading (`1`), sub mark (`+`) or cmp mark (`-`).
fn decode_cell_qanssubcmp(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "+" {
            cell.qsub = 1;
        } else if ca == "-" {
            cell.qcmp = 1;
        } else if ca == "1" {
            cell.qans = 1;
        }
        Ok(())
    })
}

/// `encodeCellQanssubcmp`: shading first, then sub mark, then cmp mark.
fn encode_cell_qanssubcmp(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qans == 1 {
            "1 ".into()
        } else if cell.qsub == 1 {
            "+ ".into()
        } else if cell.qcmp == 1 {
            "- ".into()
        } else {
            ". ".into()
        }
    });
}

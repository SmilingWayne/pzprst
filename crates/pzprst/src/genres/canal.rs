//! Canal View (pzprjs `variety/nurikabe.js`, pid `canal`). Nurikabe's FileIO
//! with a third answer mark (`c`, the "clue completed" mark). The sibling
//! nurikabe/nuribou/mochikoro/… pids are not modelled.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Canal;

struct Canal;

impl Genre for Canal {
    fn pid(&self) -> &'static str {
        "canal"
    }

    fn hasborder(&self) -> u8 {
        0
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
        decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        encode_cell_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() + b.rows() - 2) as i32;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "canal cell value outside clue 0..cols+rows-2 / ? or answer mark",
            );
        }
    }
}

/// `decodeCellAns`: `#` shades the cell, `+` its background and `c` its
/// "clue completed" mark.
fn decode_cell_ans(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "#" {
            cell.qans = 1;
        } else if ca == "+" {
            cell.qsub = 1;
        } else if ca == "c" {
            cell.qcmp = 1;
        }
        Ok(())
    })
}

/// `encodeCellAns`: shading, then background, then the completed mark.
fn encode_cell_ans(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qans == 1 {
            "# ".into()
        } else if cell.qsub == 1 {
            "+ ".into()
        } else if cell.qcmp == 1 {
            "c ".into()
        } else {
            ". ".into()
        }
    });
}

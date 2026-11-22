//! Trizone (pzprjs `variety/lapaz.js`, pid `trizone`). Shading in three-cell
//! rooms, sharing La Paz's FileIO/URL codecs.
//!
//! pzprjs declares `Board.hasborder` 2, but both FileIO border calls pass an
//! explicit `1` (`decodeBorderAns(1)` / `encodeBorderAns(1)`): the outer frame
//! is never serialized, and the URL stores no border data either. pzprst models
//! only the inner border layer (`hasborder` 1) so its shared border codec
//! reproduces those inner-only layers byte for byte.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Trizone;

struct Trizone;

impl Genre for Trizone {
    fn pid(&self) -> &'static str {
        "trizone"
    }

    fn hasborder(&self) -> u8 {
        // Inner borders only: pzprjs's FileIO calls pass `decodeBorderAns(1)`.
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('g', "trizone_ghost")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_number16(b, body, d);
        encode::decode_empty(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_number16(b),
            encode::encode_empty(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "g", "trizone_ghost");
        decode_cell_qnum(r, b, d)?;
        fileio::decode_border_ans(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "g", "trizone_ghost");
        encode_cell_qnum(w, b);
        fileio::encode_border_ans(w, b);
        fileio::encode_cell_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=6).contains(&c.qnum))
                || !(c.ques == 0 || c.ques == 7)
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "trizone cell value outside clue 0-6 / ? / shaded or answer mark",
            );
        }
        if b
            .borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "trizone border value outside answer mark 0/1",
            );
        }
    }
}

/// `decodeCellQnum`: `#` shades the cell, `-` is `?`, `.` leaves it empty.
fn decode_cell_qnum(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "#" {
            cell.ques = 7;
        } else if ca == "-" {
            cell.qnum = -2;
        } else if ca != "." {
            cell.qnum = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQnum`: `#` shaded, then the number, `?` as `-`, else `.`.
fn encode_cell_qnum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.ques == 7 {
            "# ".into()
        } else if cell.qnum >= 0 {
            format!("{} ", cell.qnum)
        } else if cell.qnum == -2 {
            "- ".into()
        } else {
            ". ".into()
        }
    });
}

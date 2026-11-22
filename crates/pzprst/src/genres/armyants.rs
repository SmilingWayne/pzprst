//! Army Ants (pzprjs `variety/kaero.js`, pid `armyants`).
//!
//! Numbers 1-N (`Cell.maxnum` is the cell count, at most 999) are the ants'
//! order; the path itself is the answer, stored as border lines. The URL stores
//! border walls first and then the numbers with `decodeNumber16`; FileIO stores
//! numbers, marks, border walls and border lines.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Armyants;

struct Armyants;

impl Genre for Armyants {
    fn pid(&self) -> &'static str {
        "armyants"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        encode::decode_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_border(b),
            encode::encode_number16(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_qanssub(r, b, d)?;
        fileio::decode_border_ques(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_qanssub(w, b);
        fileio::encode_border_ques(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows()).min(999) as i32;
        for cell in &b.cells {
            if !matches!(cell.qnum, -2 | -1) && !(1..=maxnum).contains(&cell.qnum) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    format!("armyants cell value outside empty / ? / number 1-{maxnum}"),
                );
            }
            if !(0..=1).contains(&cell.qans) || !(0..=4).contains(&cell.qsub) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    "armyants cell answer value outside shading 0/1, background 0-4",
                );
            }
        }
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.ques)
                || !(0..=1).contains(&e.line)
                || !(e.qsub == 0 || e.qsub == 2)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "armyants border value outside wall 0/1, line 0/1, cross -1",
            );
        }
    }
}

//! Soulmates (pzprjs `variety/soulmates.js`, pid `soulmates`).
//!
//! Cells hold one number each; `ques = 7` is a cell outside the grid, encoded
//! as `0` in the URL and `#` in FileIO.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Soulmates;

struct Soulmates;

impl Genre for Soulmates {
    fn pid(&self) -> &'static str {
        "soulmates"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let n = b.cells.len();
        let cells = &mut b.cells;
        encode::generic_decode_number16(n, body, d, |c, val| {
            if val == 0 {
                cells[c].ques = 7;
            } else {
                cells[c].qnum = val;
            }
        })
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::generic_encode_number16(b.cells.len(), |c| {
            let cell = &b.cells[c];
            if cell.ques == 7 {
                0
            } else {
                cell.qnum
            }
        }))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows()) as i32 - 1;
        if b.cells.iter().any(|c| {
            !(c.ques == 0 || c.ques == 7)
                || !(c.qnum == -1 || (1..=maxnum).contains(&c.qnum))
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("soulmates cell value outside empty / number 1-{maxnum} / outside-grid"),
            );
        }
    }
}

/// `decodeCellQnum`: `#` is a cell outside the grid, `.` leaves it empty.
fn decode_cell_qnum(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "#" {
            cell.ques = 7;
        } else if ca != "." {
            cell.qnum = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQnum`.
fn encode_cell_qnum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qnum >= 0 {
            format!("{} ", cell.qnum)
        } else if cell.ques == 7 {
            "# ".into()
        } else {
            ". ".into()
        }
    });
}

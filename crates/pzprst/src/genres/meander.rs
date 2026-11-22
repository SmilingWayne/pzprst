//! Meander (pzprjs `variety/ripple.js`, pid `meander`). Borders split the
//! cells into rooms; the answer numbers each room along a consecutive path.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Meander;

struct Meander;

/// `decodeCellQnum`: `#` is a shaded cell, `.` leaves the number empty.
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

/// `encodeCellQnum`: `#` for a shaded cell, then number / `?` / `.`.
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

impl Genre for Meander {
    fn pid(&self) -> &'static str {
        "meander"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        let rest = encode::decode_number16(b, &rest, d);
        encode::decode_empty(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut out = encode::encode_border(b);
        out.push_str(&encode::encode_number16(b));
        out.push_str(&encode::encode_empty(b));
        Ok(out)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_border_ques(r, b, d)?;
        decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_border_ques(w, b);
        encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs maxnum is the size of the cell's room; rooms are not modelled,
        // so cols*rows (the largest possible room) bounds the numbers.
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(c.ques == 0 || c.ques == 7)
                || !(c.anum == -1 || (1..=maxnum).contains(&c.anum))
                || !(0..=4).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("meander cell value outside number 1-{maxnum} / ? / shaded or answer mark"),
            );
        }
        if b.borders.iter().any(|e| !(0..=1).contains(&e.ques)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "meander border value outside wall 0/1",
            );
        }
    }
}

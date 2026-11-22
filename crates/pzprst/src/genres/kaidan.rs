//! Stairwell / Kaidan (pzprjs `variety/kaidan.js`, pid `kaidan`).
//!
//! The URL stores only cell numbers (`decode4Cell`). FileIO adds a cell bit mask
//! for answer marks, then the border line layer.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Kaidan;

struct Kaidan;

impl Genre for Kaidan {
    fn pid(&self) -> &'static str {
        "kaidan"
    }

    fn hasborder(&self) -> u8 {
        1
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
        fileio::decode_cell_qnum(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            let val = fileio::int(ca)?;
            if val & 1 != 0 {
                cell.qans = 1;
            }
            if val & 2 != 0 {
                cell.line = 1;
            }
            if val & 4 != 0 {
                cell.qsub = 1;
            }
            if val & 8 != 0 {
                cell.qcmp = 1;
            }
            if val & 16 != 0 {
                cell.qsub = 2;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        w.encode_cell(b, |cell| {
            let ans = cell.qans == 1;
            let sub = cell.qsub == 1;
            let sub2 = cell.qsub == 2;
            let value = (ans as i32)
                | (cell.line << 1)
                | ((sub as i32) << 2)
                | (cell.qcmp << 3)
                | ((sub2 as i32) << 4);
            format!("{value} ")
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let bad_cell = b.cells.iter().any(|c| {
            !(-2..=4).contains(&c.qnum)
                || !(0..=1).contains(&c.qans)
                || !(0..=2).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
                || !(0..=1).contains(&c.line)
        });
        if bad_cell {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kaidan cell value outside clue -2..4 or marks 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kaidan border value outside line 0/1, cross -1",
            );
        }
    }
}

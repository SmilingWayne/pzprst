//! Orbital (pzprjs `variety/nagenawa.js`, pid `orbital`). Nagenawa and Ringring
//! share that script but override `Encode`/`FileIO`, so they are separate pids.
//!
//! The URL stores shaded (`ques` 6) cells in a base-32 bit layer (`decodeIce`),
//! then the clue numbers (`decodeNumber16`). FileIO writes `#` for a shaded
//! cell, `-` for a `?` and the number for a clue. `Cell@orbital.maxnum` is
//! `(cols + rows - 2) * 2`; `minnum` is 0.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Orbital;

struct Orbital;

impl Genre for Orbital {
    fn pid(&self) -> &'static str {
        "orbital"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_ice(b, body, d);
        encode::decode_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_ice(b),
            encode::encode_number16(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "#" {
                cell.ques = 6;
            } else if ca == "-" {
                cell.qnum = -2;
            } else if ca != "." {
                cell.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 6 {
                "# ".into()
            } else if cell.qnum == -2 {
                "- ".into()
            } else if cell.qnum >= 0 {
                format!("{} ", cell.qnum)
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = ((b.cols() + b.rows()) as i32 - 2) * 2;
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "orbital cell number outside 0..(cols + rows - 2) * 2",
            );
        }
        if b.cells.iter().any(|c| !matches!(c.ques, 0 | 6)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "orbital cell mark outside empty/shaded",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !matches!(e.qsub, 0 | 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "orbital border value outside line 0/1, cross -1",
            );
        }
    }
}

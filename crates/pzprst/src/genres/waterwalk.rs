//! Water Walk (pzprjs `variety/icewalk.js`, pid `waterwalk`).
//!
//! Shared walk format: `encodeIce` (a base-32 layer of `ques = 6` ice cells)
//! followed by `encodeNumber16` (clues, `.` for `?`), with URL flag `f`
//! (`loop_full`). FileIO stores ice as `#` (or `A`/`B`/`C` together with the
//! answer arc `qans` 1–3), `?` as `-`, clues as their decimal number, and the
//! border lines/answer crosses through `decodeBorderLine`.
//!
//! pzprjs allocates the `(cols + 1) x (rows + 1)` cross grid for every walk
//! pid (`Board.hascross` is the default 2), but only `firewalk` stores cross
//! data; the FileIO cross layer of the shared variety file is gated on
//! `this.pid === "firewalk"`, so none is decoded or encoded here.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Waterwalk;

struct Waterwalk;

impl Genre for Waterwalk {
    fn pid(&self) -> &'static str {
        "waterwalk"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    /// pzprjs `Board.hascross` (the inherited default 2: crosses exist on the
    /// even/even points of the whole board, outer frame included).
    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('f', "loop_full")]
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
        r.decode_config_flag(b, "f", "loop_full");
        r.decode_cell(b, d, |cell, ca| {
            if ca == "A" {
                cell.ques = 6;
                cell.qans = 1;
            } else if ca == "B" {
                cell.ques = 6;
                cell.qans = 2;
            } else if ca == "C" {
                cell.ques = 6;
                cell.qans = 3;
            } else if ca == "#" {
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
        w.encode_config_flag(b, "f", "loop_full");
        w.encode_cell(b, |cell| {
            if cell.qans > 0 {
                // pzprjs `String.fromCharCode(64 + cell.qans)`.
                let ca = char::from_u32(64 + cell.qans as u32).unwrap_or('\u{fffd}');
                format!("{ca} ")
            } else if cell.ques == 6 {
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
        // pzprjs `Cell.maxnum`: cols * rows (minnum is the default 1).
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.ques == 0 || c.ques == 6)
                || !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(0..=3).contains(&c.qans)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!(
                    "waterwalk cell value outside empty/ice, clue 1-{maxnum}/?, or answer arc 0-3"
                ),
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "waterwalk border value outside line 0/1, cross -1",
            );
        }
    }
}

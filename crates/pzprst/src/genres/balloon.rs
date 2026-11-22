//! Balloon Box (pzprjs `variety/balloon.js`, pid `balloon`).
//!
//! The URL is the `decodeIce` bit layer (cell `ques` 6) followed by
//! `decodeNumber16` cell clues; FileIO stores the ice cells as `#`, the clues
//! as decimal numbers, then the border answer and line layers.
//!
//! The `a` URL flag / FileIO line is pzprjs's `balloon_adjacent` config, which
//! pzprjs writes when the config is *false*. `Board.rules` follows the pzprst
//! convention (the flag letter is present), so the stored boolean is the
//! presence of the flag, i.e. the negation of the pzprjs config value.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Balloon;

struct Balloon;

impl Genre for Balloon {
    fn pid(&self) -> &'static str {
        "balloon"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('a', "balloon_adjacent")]
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
        r.decode_config_flag(b, "a", "balloon_adjacent");
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
        fileio::decode_border_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "a", "balloon_adjacent");
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
        fileio::encode_border_ans(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum` is a function of the board size.
        let maxnum = (b.cols() * b.rows()) as i32 - b.cols().min(b.rows()) as i32;
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "balloon cell value outside empty / ? / clue 1..cols*rows-min(cols,rows)",
            );
        }
        if b.cells.iter().any(|c| c.ques != 0 && c.ques != 6) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "balloon cell value outside plain 0 / ice 6",
            );
        }
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.qans)
                || !(0..=1).contains(&e.qsub)
                || !(0..=1).contains(&e.line)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "balloon border value outside answer 0/1, cross 0/1 or line 0/1",
            );
        }
    }
}

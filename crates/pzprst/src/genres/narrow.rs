//! Narrowfence (pzprjs `variety/nawabari.js`, pid `narrow`). Rectangular rooms
//! separated by borders, each with one clue equal to its fence count. Borders
//! are `hasborder` 2 (the outer frame included); `?` is legal in the formats
//! even though the UI disables hatena input.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Narrow;

struct Narrow;

impl Genre for Narrow {
    fn pid(&self) -> &'static str {
        "narrow"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
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
        r.decode_cell(b, d, |cell, ca| {
            cell.ques = 0;
            if ca == "*" {
                cell.ques = 7;
            } else if ca == "-" {
                cell.qnum = -2;
            } else if ca != "." {
                cell.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 7 {
                "* ".into()
            } else if cell.qnum == -2 {
                "- ".into()
            } else if cell.qnum >= 0 {
                format!("{} ", cell.qnum)
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=4).contains(&c.qnum))
                || !(c.ques == 0 || c.ques == 7)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "narrow cell value outside clue 1-4 / ? / shaded",
            );
        }
        if b
            .borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "narrow border value outside answer mark 0/1",
            );
        }
    }
}

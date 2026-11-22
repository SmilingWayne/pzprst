//! Family Photo (pzprjs `variety/familyphoto.js`, pid `familyphoto`).
//!
//! Cells hold a clue (`qnum`, `?` = -2) and the `ques = 6` photo mark. The URL
//! stores the photo layer with `decodeIce` followed by the clue layer
//! (`decodeNumber16`); FileIO writes `#` before the clue token and stores the
//! answer border marks with `BorderAns`.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Familyphoto;

struct Familyphoto;

impl Genre for Familyphoto {
    fn pid(&self) -> &'static str {
        "familyphoto"
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
            let ca = match ca.strip_prefix('#') {
                Some(rest) => {
                    cell.ques = 6;
                    rest
                }
                None => ca,
            };
            if ca == "-" {
                cell.qnum = -2;
            } else if let Some(n) = jsnum::integer(ca) {
                // pzprjs: `else if (+ca > 0) cell.qnum = +ca`; `.`, an empty
                // `#` token or garbage leaves the cell empty.
                if n > 0 {
                    cell.qnum = n;
                }
            }
            Ok(())
        })?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            let mut ca = String::new();
            if cell.ques == 6 {
                ca.push('#');
            }
            if cell.qnum == -2 {
                ca.push('-');
            } else if cell.qnum > 0 {
                ca.push_str(&cell.qnum.to_string());
            }
            if ca.is_empty() {
                ca.push('.');
            }
            format!("{ca} ")
        });
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.ques == 0 || c.ques == 6)
                || !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!(
                    "familyphoto cell value outside empty / ? / clue 0-{maxnum} or photo mark"
                ),
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "familyphoto border value outside answer 0/1 or cross 0/1",
            );
        }
    }
}

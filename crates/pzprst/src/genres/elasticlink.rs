//! Elastic Link (pzprjs `variety/elasticlink.js`, pid `elasticlink`).
//!
//! The URL has an ice-cell bit field, the cell numbers and a shaded-cell bit
//! field, in that order. FileIO packs ice (`O` prefix), the number and shading
//! into one token per cell plus the answer line layer.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Elasticlink;

struct Elasticlink;

impl Genre for Elasticlink {
    fn pid(&self) -> &'static str {
        "elasticlink"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_ice(b, body, d);
        let rest = encode::decode_number16(b, &rest, d);
        encode::decode_empty(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}{}",
            encode::encode_ice(b),
            encode::encode_number16(b),
            encode::encode_empty(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "#" {
                cell.ques = 7;
                return Ok(());
            }
            let mut rest = ca;
            if let Some(stripped) = rest.strip_prefix('O') {
                cell.ques = 6;
                rest = stripped;
            }
            if rest == "-" {
                cell.qnum = -2;
            } else if rest != "." {
                // pzprjs stores `+ca` only when it is greater than zero.
                if let Some(n) = jsnum::integer(rest) {
                    if n > 0 {
                        cell.qnum = n;
                    }
                }
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 7 {
                return "# ".into();
            }
            let mut ca = String::new();
            if cell.ques == 6 {
                ca.push('O');
            }
            if cell.qnum == -2 {
                ca.push('-');
            } else if cell.qnum > 0 {
                ca.push_str(&cell.qnum.to_string());
            }
            if ca.is_empty() {
                ca = ".".into();
            }
            format!("{ca} ")
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum` is `max(cols, rows)`.
        let maxnum = b.cols().max(b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(c.ques == 0 || c.ques == 6 || c.ques == 7)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("elasticlink cell value outside empty / ? / clue 1-{maxnum}, ice 6 or shade 7"),
            );
        }
        if b
            .borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "elasticlink border value outside line 0/1, cross -1",
            );
        }
    }
}

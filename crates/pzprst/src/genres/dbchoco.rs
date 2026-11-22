//! Double Choco (pzprjs `variety/cbblock.js`, pid `dbchoco`). Combi Block
//! (`cbblock`), NIKOJI (`nikoji`) and Mirror Block (`mirrorbk`) share that
//! script but override `Encode`/`FileIO`, so they are separate pids.
//!
//! The URL is the `decodeIce` bit layer (cell `ques` 6) followed by
//! `decodeNumber16` cell clues. FileIO writes an ice cell with a leading `-`,
//! `0` for a question mark and the clue itself otherwise.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Dbchoco;

struct Dbchoco;

impl Genre for Dbchoco {
    fn pid(&self) -> &'static str {
        "dbchoco"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_dbchoco(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_dbchoco(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        let mut ignored = false;
        r.decode_cell(b, d, |cell, ca| {
            let token = match ca.strip_prefix('-') {
                Some(rest) => {
                    cell.ques = 6;
                    rest
                }
                None => ca,
            };
            if token == "0" {
                cell.qnum = -2;
            } else if token != "." && !token.is_empty() {
                // pzprjs keeps `+ca` only when it is a positive number and
                // silently ignores anything else; non-integers cannot be
                // stored in an i32, so they are reported and dropped.
                match jsnum::integer(token) {
                    Some(n) if n > 0 => cell.qnum = n,
                    _ => ignored = true,
                }
            }
            Ok(())
        })?;
        if ignored {
            d.push(
                DiagnosticCode::UnexpectedChar,
                "a dbchoco cell token that is not a positive number is read as an empty cell",
            );
        }
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            let mut ca = String::new();
            if cell.ques == 6 {
                ca.push('-');
            }
            if cell.qnum == -2 {
                ca.push('0');
            } else if cell.qnum != -1 {
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
        // `Cell.maxnum` is a function of the board size.
        let maxnum = ((b.cols() * b.rows()) >> 1) as i32;
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "dbchoco cell value outside empty / ? / clue 1..cols*rows/2",
            );
        }
        if b.cells.iter().any(|c| c.ques != 0 && c.ques != 6) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "dbchoco cell value outside plain 0 / ice 6",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "dbchoco border value outside answer 0/1 or cross 0/1",
            );
        }
    }
}

/// `decodeDBChoco`: the `decodeIce` bit layer (cell `ques` 6) followed by
/// `decodeNumber16` cell clues.
fn decode_dbchoco(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let rest = encode::decode_ice(b, bstr, d);
    encode::decode_number16(b, &rest, d)
}

/// `encodeDBChoco`: `encodeIce` then `encodeNumber16`.
fn encode_dbchoco(b: &Board) -> String {
    format!("{}{}", encode::encode_ice(b), encode::encode_number16(b))
}

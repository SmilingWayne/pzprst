//! Goishi (pzprjs `variety/goishi.js`, pid `goishi`).
//!
//! pzprjs `Cell.ques` defaults to 7 (no stone); pzprst objects start at their
//! shared default 0, so both decoders set every cell to 7 before reading.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Goishi;

struct Goishi;

impl Genre for Goishi {
    fn pid(&self) -> &'static str {
        "goishi"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        for cell in &mut b.cells {
            cell.ques = 7;
        }
        decode_goishi(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_goishi(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        for cell in &mut b.cells {
            cell.ques = 7;
        }
        r.decode_cell(b, d, |cell, ca| {
            if ca != "." {
                cell.ques = 0;
                if ca != "0" {
                    cell.anum = fileio::int(ca)?;
                }
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 0 {
                if cell.anum != -1 {
                    format!("{} ", cell.anum)
                } else {
                    "0 ".into()
                }
            } else {
                ". ".into()
            }
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !(c.ques == 0 || c.ques == 7) || c.anum < -1)
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "goishi cell value outside stone 0/7, answer -1 or positive",
            );
        }
    }
}

/// `decodeGoishi`: five base-32 bits per character, a set bit is an empty cell.
fn decode_goishi(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let twi = [16, 8, 4, 2, 1];
    let mut c = 0usize;
    let mut i = 0usize;
    while i < chars.len() {
        let ca = chars[i];
        let num = match jsnum::digit(ca, 32) {
            Some(v) => v as i32,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("unexpected character {ca:?} decodes as 0"),
                );
                0
            }
        };
        for w in twi {
            if c < n {
                b.cells[c].ques = if num & w != 0 { 7 } else { 0 };
                c += 1;
            }
        }
        if c >= n {
            break;
        }
        i += 1;
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    chars.iter().skip(i + 1).collect()
}

/// `encodeGoishi`.
fn encode_goishi(b: &Board) -> String {
    let twi = [16u32, 8, 4, 2, 1];
    let mut cm = String::new();
    let mut count = 0usize;
    let mut pass = 0u32;
    for cell in &b.cells {
        if cell.ques == 7 {
            pass += twi[count];
        }
        count += 1;
        if count == 5 {
            cm.push_str(&jsnum::to_radix(pass, 32));
            count = 0;
            pass = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(pass, 32));
    }
    cm
}

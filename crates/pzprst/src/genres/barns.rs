//! Barns (pzprjs `variety/barns.js`, pid `barns`). Ice barns are `cell.ques` 6
//! and walls are `border.ques` 1; both the URL and FileIO store the wall layer,
//! the URL cell layer is a bit field of five cells per base-32 character.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Barns;

struct Barns;

impl Genre for Barns {
    fn pid(&self) -> &'static str {
        "barns"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = decode_barns(b, body, d);
        encode::decode_border(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!("{}{}", encode_barns(b), encode::encode_border(b)))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "1" {
                cell.ques = 6;
            }
            Ok(())
        })?;
        fileio::decode_border_ques(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 6 {
                "1 ".into()
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_ques(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| c.ques != 0 && c.ques != 6) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "barns cell value outside empty 0 / ice barn 6",
            );
        }
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.ques)
                || !(0..=1).contains(&e.line)
                || !(e.qsub == 0 || e.qsub == 2)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "barns border value outside wall 0/1, line 0/1, cross -1",
            );
        }
    }
}

/// `decodeBarns`: one base-32 character per five cells, bit 16 setting
/// `ques` 6 (ice barn) and every other bit clearing it.
fn decode_barns(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let twi = [16, 8, 4, 2, 1];
    let mut c = 0usize;
    let mut i = 0usize;
    while i < chars.len() {
        let ca = match jsnum::digit(chars[i], 32) {
            Some(v) => v,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("unexpected character {:?} clears five cells", chars[i]),
                );
                0
            }
        };
        for t in twi {
            if c < n {
                b.cells[c].ques = if ca & t != 0 { 6 } else { 0 };
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
    chars[(i + 1).min(chars.len())..].iter().collect()
}

/// `encodeBarns`: one base-32 character per five cells, bit 16 for `ques` 6.
fn encode_barns(b: &Board) -> String {
    let twi = [16, 8, 4, 2, 1];
    let mut cm = String::new();
    let mut num = 0usize;
    let mut pass = 0u32;
    for cell in &b.cells {
        if cell.ques == 6 {
            pass += twi[num];
        }
        num += 1;
        if num == 5 {
            cm.push_str(&jsnum::to_radix(pass, 32));
            num = 0;
            pass = 0;
        }
    }
    if num > 0 {
        cm.push_str(&jsnum::to_radix(pass, 32));
    }
    cm
}

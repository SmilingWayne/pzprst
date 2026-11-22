//! Return Home (pzprjs `variety/kaero.js`, pid `kaero`).
//!
//! Numbers 1-52 (`Cell.maxnum` 52, drawn as letters in pzprjs) tell in which
//! order the path visits the cells; the path itself is the answer, stored as
//! border lines and background marks. The URL stores the border walls first and
//! then a custom base-36 number encoding (`decodeKaero`/`encodeKaero`);
//! FileIO stores numbers, marks, border walls and border lines.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Kaero;

struct Kaero;

impl Genre for Kaero {
    fn pid(&self) -> &'static str {
        "kaero"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        decode_kaero(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!("{}{}", encode::encode_border(b), encode_kaero(b)))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_qanssub(r, b, d)?;
        fileio::decode_border_ques(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_qanssub(w, b);
        fileio::encode_border_ques(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        for cell in &b.cells {
            if !matches!(cell.qnum, -2 | -1) && !(1..=52).contains(&cell.qnum) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    "kaero cell value outside empty / ? / number 1-52",
                );
            }
            if !(0..=1).contains(&cell.qans) || !(0..=4).contains(&cell.qsub) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    "kaero cell answer value outside shading 0/1, background 0-4",
                );
            }
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.ques) || !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kaero border value outside wall 0/1, line 0/1, cross -1",
            );
        }
    }
}

/// `decodeKaero`: numbers as base-36 letters, `.` for `?`, `-X` for 37-72 and
/// base-36 run lengths for empty cells.
fn decode_kaero(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut a, mut i) = (0usize, 0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        if ca.is_ascii_digit() {
            b.cells[c].qnum = jsnum::digit(ca, 36).unwrap() as i32 + 27;
        } else if ca.is_ascii_uppercase() {
            b.cells[c].qnum = jsnum::digit(ca, 36).unwrap() as i32 - 9;
        } else if ca == '-' {
            if i + 1 < chars.len() {
                match jsnum::digit(chars[i + 1], 36) {
                    Some(v) => b.cells[c].qnum = v as i32 + 37,
                    None => d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("unexpected character {:?} after '-'", chars[i + 1]),
                    ),
                }
                i += 1;
            } else {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    "'-' at the end of the body has no number",
                );
            }
        } else if ca == '.' {
            b.cells[c].qnum = -2;
        } else if ca.is_ascii_lowercase() {
            c += jsnum::digit(ca, 36).unwrap() as usize - 10;
        }
        c += 1;
        i += 1;
        if c >= n {
            a = i;
            break;
        }
    }
    chars[a.min(chars.len())..].iter().collect()
}

/// `encodeKaero`: one character per cell, base-36 run lengths for empty cells.
fn encode_kaero(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0usize;
    for cell in &b.cells {
        let qnum = cell.qnum;
        let mut pstr = String::new();
        if qnum == -2 {
            pstr = ".".into();
        } else if (1..=26).contains(&qnum) {
            pstr = jsnum::to_radix((qnum + 9) as u32, 36).to_uppercase();
        } else if (27..=36).contains(&qnum) {
            pstr = (qnum - 27).to_string();
        } else if (37..=72).contains(&qnum) {
            pstr = format!("-{}", jsnum::to_radix((qnum - 37) as u32, 36).to_uppercase());
        } else {
            count += 1;
        }
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 26 {
            cm.push_str(&jsnum::to_radix(9 + count as u32, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(9 + count as u32, 36));
    }
    cm
}

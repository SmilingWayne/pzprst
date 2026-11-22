//! Heteromino (pzprjs `variety/nawabari.js`, pid `heteromino`). It shares the
//! `decodeFivecells`/`encodeFivecells` URL codec and the plain `Cell` /
//! `BorderAns` FileIO layers with `fivecells`; only the cell range differs.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Heteromino;

struct Heteromino;

impl Genre for Heteromino {
    fn pid(&self) -> &'static str {
        "heteromino"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_fivecells(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_fivecells(b))
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
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (1..=999).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "heteromino cell value outside empty / ? / clue 1-999",
            );
        }
        if b.cells.iter().any(|c| c.ques != 0 && c.ques != 7) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "heteromino cell value outside unshaded 0 / shaded 7",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "heteromino border value outside answer 0/1 or cross 0/1",
            );
        }
    }
}

/// `decodeFivecells`: one character per cell before base-36 blank runs, `7` for
/// a shaded cell (question 7) and `.` for a question mark.
fn decode_fivecells(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    for cell in &mut b.cells {
        cell.ques = 0;
    }
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        if ca == '7' {
            b.cells[c].ques = 7;
        } else if ca == '.' {
            b.cells[c].qnum = -2;
        } else if ca.is_ascii_digit() {
            b.cells[c].qnum = ca as i32 - '0' as i32;
        } else if ca.is_ascii_lowercase() {
            c += jsnum::digit(ca, 36).unwrap() as usize - 10;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} skips one cell"),
            );
        }
        c += 1;
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
    // pzprjs keeps the overflowing character (`outbstr = bstr.substr(i)`) but
    // never reads it back, so consume it like `decodeNumber10` does.
    chars.iter().skip(i + 1).collect()
}

/// `encodeFivecells`: `7` for a shaded cell, `.` for `?`, one decimal digit per
/// clue and base-36 run lengths (`9 + count`) for blank cells.
fn encode_fivecells(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let mut pstr = String::new();
        if cell.ques == 7 {
            pstr.push('7');
        } else if cell.qnum == -2 {
            pstr.push('.');
        } else if cell.qnum != -1 {
            pstr = cell.qnum.to_string();
        } else {
            count += 1;
        }
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 26 {
            cm.push_str(&jsnum::to_radix(9 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(9 + count, 36));
    }
    cm
}

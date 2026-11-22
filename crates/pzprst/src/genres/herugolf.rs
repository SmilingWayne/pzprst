//! Herugolf (pzprjs `variety/herugolf.js`, pid `herugolf`).
//!
//! Two cell properties: `ques` 6 (water) and 31 (hole), and a clue `qnum`.
//! The URL encodes water in a base-32 bit layer (`decodeIce`) followed by the
//! clue/hole layer (`decodeNumber16_herugolf`); FileIO stores water as `i`,
//! holes as `H` and clues as their decimal number.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Herugolf;

struct Herugolf;

impl Genre for Herugolf {
    fn pid(&self) -> &'static str {
        "herugolf"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_ice(b, body, d);
        decode_number16_herugolf(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_ice(b),
            encode_number16_herugolf(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "H" {
                cell.ques = 31;
            } else if ca == "i" {
                cell.ques = 6;
            } else if ca != "." {
                cell.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 31 {
                "H ".into()
            } else if cell.ques == 6 {
                "i ".into()
            } else if cell.qnum > 0 {
                format!("{} ", cell.qnum)
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        for (i, cell) in b.cells.iter().enumerate() {
            let (bx, by) = b.geom.cell_pos(i);
            let (cx, cy) = (bx >> 1, by >> 1);
            let maxnum = cx.max(cy).max(cols - 1 - cx).max(rows - 1 - cy);
            if !matches!(cell.ques, 0 | 6 | 31) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    "herugolf cell mark outside empty/water/hole",
                );
            }
            if cell.qnum == -2 {
                d.push(
                    DiagnosticCode::LossyUrl,
                    "a ? clue is dropped from a herugolf URL",
                );
            } else if !(cell.qnum == -1 || (1..=maxnum).contains(&cell.qnum)) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    "herugolf cell number outside 1..cell maximum",
                );
            } else if cell.qnum >= 256 {
                d.push(
                    DiagnosticCode::LossyUrl,
                    "cell numbers of 256 or more are dropped from a herugolf URL",
                );
            } else if cell.ques == 31 && cell.qnum != -1 {
                d.push(
                    DiagnosticCode::LossyUrl,
                    "a numbered hole keeps only its number in a herugolf URL",
                );
            }
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "herugolf border value outside line 0/1, cross -1",
            );
        }
    }
}

/// `decodeNumber16_herugolf`: `0`–`f` numbers, `h` holes, `i`–`z` blanks.
///
/// Unlike the shared `decodeNumber16`, `.` is not a clue: a dot is an ordinary
/// one-cell blank, so a `?` never survives a herugolf URL round trip. A gap
/// marker wider than the remaining board ends the loop like pzprjs's
/// `if (!bd.cell[c]) break`.
fn decode_number16_herugolf(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let mut c = 0usize;
    let mut i = 0usize;
    while i < chars.len() {
        let ca = chars[i];
        let Some(cell) = b.cells.get_mut(c) else {
            break;
        };
        if ca.is_ascii_digit() || ('a'..='f').contains(&ca) {
            cell.qnum = ca.to_digit(16).unwrap() as i32;
        } else if ca == '-' {
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qn) => cell.qnum = qn as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal digits after '-' leave the cell empty",
                ),
            }
            i += 2;
        } else if ca == 'h' {
            cell.ques = 31;
        } else if ('i'..='z').contains(&ca) {
            c += jsnum::digit(ca, 36).unwrap() as usize - 18;
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
    chars[(i + 1).min(chars.len())..].iter().collect()
}

/// `encodeNumber16_herugolf`: numbers in hex (`-` escape for 16–255), `h` holes,
/// `17 + count` in base 36 for blank runs, flushed after 18 blanks like pzprjs.
fn encode_number16_herugolf(b: &Board) -> String {
    let mut count = 0u32;
    let mut cm = String::new();
    for cell in &b.cells {
        let mut pstr = String::new();
        let (qn, qu) = (cell.qnum, cell.ques);
        if qn == -2 {
            pstr = ".".into();
        } else if (0..16).contains(&qn) {
            pstr = jsnum::to_radix(qn as u32, 16);
        } else if (16..256).contains(&qn) {
            pstr = format!("-{}", jsnum::to_radix(qn as u32, 16));
        } else if qu == 31 {
            pstr = "h".into();
        } else {
            count += 1;
        }
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 18 {
            cm.push_str(&jsnum::to_radix(17 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(17 + count, 36));
    }
    cm
}

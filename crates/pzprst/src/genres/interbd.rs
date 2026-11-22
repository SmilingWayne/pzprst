//! International Borders (pzprjs `variety/interbd.js`, pid `interbd`).
//!
//! A cell stores one packed value for URL and FileIO: the clue number
//! (`qnum`, -2 is `?`, 0..4 are regular clues) and the colour (`ques`, 0 is no
//! colour, 1..6 are clue colours). `qans` is the answer shading and `qsub` a
//! dot.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Interbd;

struct Interbd;

/// `getNumColor`: the single number the URL and FileIO store. An empty cell is
/// -1; otherwise the colour is the tens digit and the clue is one more than
/// `qnum` (`0` for no clue yet).
fn get_num_color(cell: &crate::Obj) -> i32 {
    if cell.qnum == -1 && cell.ques == 0 {
        return -1;
    }
    let num = if cell.qnum < 0 { 0 } else { cell.qnum + 1 };
    cell.ques * 10 + num
}

/// `setNumColor`: split a packed value into `ques` / `qnum` and clear the
/// answer marks. Negative values clear the cell.
fn set_num_color(cell: &mut crate::Obj, val: i32) {
    if val < 0 {
        cell.qnum = -1;
        cell.ques = 0;
    } else {
        let num = val % 10;
        cell.qnum = if num > 0 { num - 1 } else { -2 };
        cell.ques = val / 10;
        cell.qans = 0;
        cell.qsub = 0;
    }
}

impl Genre for Interbd {
    fn pid(&self) -> &'static str {
        "interbd"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let chars: Vec<char> = body.chars().collect();
        let n = b.cells.len();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() && c < n {
            let ca = chars[i];
            let (val, used) = encode::read_number16(&chars, i);
            match val {
                Some(v) if v != -1 => {
                    set_num_color(&mut b.cells[c], v);
                    i += used;
                    c += 1;
                }
                // `g`-`z` skip base-36 digits minus 15 cells.
                Some(_) => {
                    if ('g'..='z').contains(&ca) {
                        c += jsnum::digit(ca, 36).unwrap() as usize - 15;
                        i += 1;
                    } else {
                        d.push(
                            DiagnosticCode::UnexpectedChar,
                            format!("unexpected character {ca:?} is skipped"),
                        );
                        i += 1;
                    }
                }
                // `setNumColor(NaN)` takes the `val < 0`-free path with
                // `NaN % 10`, so pzprjs stores a `?` with no colour.
                None => {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("escape at character {i} is not hexadecimal; the cell becomes a blank question mark"),
                    );
                    set_num_color(&mut b.cells[c], 0);
                    i += used;
                    c += 1;
                }
            }
        }
        if c < n {
            d.push(
                DiagnosticCode::TruncatedBody,
                "body ends before the last cell",
            );
        }
        chars[i.min(chars.len())..].iter().collect()
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut count = 0u32;
        let mut cm = String::new();
        for cell in &b.cells {
            let pstr = encode::write_number16(i64::from(get_num_color(cell)));
            if pstr.is_empty() {
                count += 1;
            }
            if count == 0 {
                cm.push_str(&pstr);
            } else if !pstr.is_empty() || count == 20 {
                cm.push_str(&jsnum::to_radix(count + 15, 36));
                cm.push_str(&pstr);
                count = 0;
            }
        }
        if count > 0 {
            cm.push_str(&jsnum::to_radix(count + 15, 36));
        }
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "#" {
                cell.qans = 1;
            } else if ca == "+" {
                cell.qsub = 1;
            } else if ca != "." {
                set_num_color(cell, fileio::int(ca)?);
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            let num = get_num_color(cell);
            if num >= 0 {
                format!("{num} ")
            } else if cell.qans == 1 {
                "# ".into()
            } else if cell.qsub == 1 {
                "+ ".into()
            } else {
                ". ".into()
            }
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=4).contains(&c.qnum))
                || !(0..=6).contains(&c.ques)
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "interbd cell value outside empty / ? / clue 0-4, colour 0-6, answer 0/1, dot 0/1",
            );
        }
    }
}

//! Light and Shadow (pzprjs `variety/lightshadow.js`, pid `lightshadow`).
//! `nibunnogo` shares that script but overrides `Encode`/`FileIO`, so it is a
//! separate pid with its own genre file.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Lightshadow;

struct Lightshadow;

impl Genre for Lightshadow {
    fn pid(&self) -> &'static str {
        "lightshadow"
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
                // 0 and 1 are white/black blanks (`qnum -2`); even/odd values
                // are white/black clues. `.` arrives here as -2 and decodes as
                // an empty cell, like pzprjs's `-2 % 2 === 0` branch.
                Some(v) if v != -1 => {
                    let cell = &mut b.cells[c];
                    if v == 0 {
                        cell.ques = 0;
                        cell.qnum = -2;
                    } else if v == 1 {
                        cell.ques = 1;
                        cell.qnum = -2;
                    } else if v % 2 == 0 {
                        cell.ques = 0;
                        cell.qnum = v / 2;
                    } else {
                        cell.ques = 1;
                        cell.qnum = (v - 1) / 2;
                    }
                    i += used;
                    c += 1;
                }
                // `g`-`z` skip base-36 digits minus 15 cells.
                _ if ('g'..='z').contains(&ca) => {
                    c += jsnum::digit(ca, 36).unwrap() as usize - 15;
                    i += 1;
                }
                // A truncated/non-hex escape is NaN in pzprjs: the cell is
                // consumed but its value is dropped by the encoder anyway.
                _ if val.is_none() => {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("escape at character {i} is not hexadecimal; the cell is left empty"),
                    );
                    i += used;
                    c += 1;
                }
                _ => {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("unexpected character {ca:?} is skipped"),
                    );
                    i += 1;
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
            // White n is 2n, black n is 2n+1; a white/black blank with no clue
            // is 0/1. `qnum -1` writes nothing, so runs are counted here.
            let pstr = match cell.qnum {
                -1 => encode::write_number16(-1),
                -2 => encode::write_number16(i64::from(cell.ques)),
                qn => encode::write_number16(i64::from(qn) * 2 + i64::from(cell.ques)),
            };
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
            if ca == "." {
                return Ok(());
            }
            let mut inp = ca.split(',');
            let ques = fileio::int(inp.next().unwrap_or(""))?;
            let qnum = fileio::int(
                inp.next()
                    .ok_or_else(|| format!("expected \"ques,qnum\", found {ca:?}"))?,
            )?;
            cell.ques = ques;
            cell.qnum = qnum;
            Ok(())
        })?;
        r.decode_cell(b, d, |cell, ca| {
            cell.qans = fileio::int(ca)?;
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.qnum != -1 {
                format!("{},{} ", cell.ques, cell.qnum)
            } else {
                ". ".into()
            }
        });
        w.encode_cell(b, |cell| format!("{} ", cell.qans));
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(0..=1).contains(&c.ques)
                || !(0..=2).contains(&c.qans)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "lightshadow cell value outside empty / ? / clue 1..cols*rows, shade 0/1, answer 0-2",
            );
        }
    }
}

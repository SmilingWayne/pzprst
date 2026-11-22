//! Voxas (pzprjs `variety/voxas.js`, pid `voxas`).
//!
//! Both URL and FileIO store only border numbers (1–5) and crosses; the URL
//! uses the compact nibble/run-length encoding with a `t` (tatami) flag.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Voxas;

struct Voxas;

/// Voxas `Encode.decodePzpr`: border `ques` numbers with run-length blanks.
fn decode_border_ques(b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = body.chars().collect();
    let n = b.borders.len();
    let mut c = 0usize;
    let mut i = 0usize;
    let mut consumed = 0usize;
    while i < chars.len() {
        if c >= n {
            // pzprjs indexes bd.border[c] here and throws; stop instead.
            break;
        }
        let ca = chars[i];
        match ca {
            '0'..='4' => b.borders[c].ques = ca as i32 - '0' as i32 + 1,
            '5'..='9' => {
                b.borders[c].ques = ca as i32 - '5' as i32 + 1;
                c += 1;
            }
            'a'..='e' => {
                b.borders[c].ques = ca as i32 - 'a' as i32 + 1;
                c += 2;
            }
            'g'..='z' => c += jsnum::digit(ca, 36).unwrap() as usize - 16,
            _ => d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} is skipped"),
            ),
        }
        c += 1;
        i += 1;
        consumed = i;
        if c >= n {
            break;
        }
    }
    chars[consumed.min(chars.len())..].iter().collect()
}

/// Voxas `Encode.encodePzpr`.
fn encode_border_ques(b: &Board) -> String {
    let borders = &b.borders;
    let mut count = 0u32;
    let mut cm = String::new();
    let mut c = 0usize;
    while c < borders.len() {
        let mut pstr = String::new();
        let qn = borders[c].ques - 1;
        if qn >= 0 {
            if c + 1 < borders.len() && borders[c + 1].ques != 0 {
                pstr = jsnum::to_radix(qn as u32, 16);
            } else if c + 2 < borders.len() && borders[c + 2].ques != 0 {
                pstr = jsnum::to_radix(5 + qn as u32, 16);
                c += 1;
            } else {
                pstr = jsnum::to_radix(10 + qn as u32, 16);
                c += 2;
            }
        } else {
            count += 1;
        }
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 20 {
            cm.push_str(&jsnum::to_radix(count + 15, 36));
            cm.push_str(&pstr);
            count = 0;
        }
        c += 1;
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(count + 15, 36));
    }
    cm
}

impl Genre for Voxas {
    fn pid(&self) -> &'static str {
        "voxas"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('t', "voxas_tatami")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_border_ques(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_border_ques(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "t", "voxas_tatami");
        r.decode_border(b, d, |border, ca| {
            if ca == "-0" {
                border.qsub = 1;
                return Ok(());
            }
            let n = fileio::int(ca)?;
            if n < 0 {
                border.qsub = 1;
                let n = -(n as i64);
                if n < 5 {
                    border.ques = n as i32;
                } else {
                    border.qans = 1;
                }
            } else if n == 0 {
            } else if n < 5 {
                border.ques = n;
            } else {
                border.qans = 1;
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "t", "voxas_tatami");
        w.encode_border(b, |border| {
            if border.ques == 0 && border.qans == 0 {
                return if border.qsub != 0 { "-0 ".into() } else { "0 ".into() };
            }
            let n = if border.qans != 0 { 5 } else { border.ques };
            let value = if border.qsub != 0 { -n } else { n };
            format!("{value} ")
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.borders.iter().any(|e| {
            !(0..=5).contains(&e.ques) || !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "voxas border value outside number 0-5, answer/cross 0/1",
            );
        }
    }
}

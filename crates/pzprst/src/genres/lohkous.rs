//! Lohkous: ordered lists of segment lengths and answer borders.
use super::Genre;
use crate::error::{Diagnostics, Result};
use crate::fileio::{FileReader, FileWriter};
use crate::{fileio, jsnum, Board};
pub(crate) static GENRE: &dyn Genre = &Lohkous;
struct Lohkous;
impl Genre for Lohkous {
    fn pid(&self) -> &'static str {
        "lohkous"
    }
    fn hasborder(&self) -> u8 {
        1
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn decode_url(&self, b: &mut Board, body: &str, _: &mut Diagnostics) -> String {
        let chars: Vec<char> = body.chars().collect();
        let (mut c, mut i, mut off) = (0usize, 0usize, false);
        while i < chars.len() && c < b.cells.len() {
            let ch = chars[i];
            if ch.is_ascii_digit() {
                let mut values = Vec::new();
                while i < chars.len() && chars[i].is_ascii_digit() {
                    values.push(if chars[i] == '0' {
                        -2
                    } else {
                        chars[i].to_digit(10).unwrap() as i32
                    });
                    i += 1;
                }
                b.cells[c].qnums = values;
                c += 1;
                off = true;
            } else {
                if ch.is_ascii_lowercase() {
                    c += (ch as u8 - b'a' + 1) as usize;
                    if off {
                        c -= 1;
                    }
                    off = false;
                }
                i += 1;
            }
            if c >= b.cells.len() {
                break;
            }
        }
        chars[i..].iter().collect()
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        let (mut count, mut out) = (0u32, String::new());
        for cell in &b.cells {
            let clue: String = cell
                .qnums
                .iter()
                .map(|&v| if v == -2 { "0".into() } else { v.to_string() })
                .collect();
            if clue.is_empty() {
                count += 1;
            }
            if count == 0 {
                out.push_str(&clue);
                count = 1;
            } else if !clue.is_empty() || count == 26 {
                out.push_str(&jsnum::to_radix(9 + count, 36));
                out.push_str(&clue);
                count = u32::from(!clue.is_empty());
            }
        }
        if count > 1 {
            out.push_str(&jsnum::to_radix(9 + count, 36));
        }
        Ok(out)
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, token| {
            if token != "." {
                cell.qnums = token
                    .split(',')
                    .map(|v| if v == "-" { Ok(-2) } else { fileio::int(v) })
                    .collect::<std::result::Result<Vec<_>, _>>()?;
            }
            Ok(())
        })?;
        fileio::decode_border_ans(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.qnums.is_empty() {
                ". ".into()
            } else {
                format!(
                    "{} ",
                    cell.qnums
                        .iter()
                        .map(|n| if *n < 0 { "-".into() } else { n.to_string() })
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
        });
        fileio::encode_border_ans(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
}

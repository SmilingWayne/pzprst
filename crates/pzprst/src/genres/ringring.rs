//! Ringring's run-length encoded blocked cells and line answers.
use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Ringring;
struct Ringring;

impl Genre for Ringring {
    fn pid(&self) -> &'static str {
        "ringring"
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

    fn decode_url(&self, b: &mut Board, body: &str, _d: &mut Diagnostics) -> String {
        let mut cell = 0usize;
        let mut end = body.len();
        for (i, ch) in body.char_indices() {
            if ch.is_ascii_digit() || ch.is_ascii_lowercase() {
                cell += jsnum::digit(ch, 36).unwrap() as usize;
                if let Some(c) = b.cells.get_mut(cell) {
                    c.ques = 1;
                }
            } else if ch == '.' {
                cell += 35;
            }
            cell += 1;
            if cell >= b.cells.len() {
                end = i + ch.len_utf8();
                break;
            }
        }
        body[end..].to_string()
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut out = String::new();
        let mut count = 0;
        for cell in &b.cells {
            if cell.ques == 1 {
                out.push_str(&jsnum::to_radix(count, 36));
                count = 0;
            } else {
                count += 1;
                if count == 36 {
                    out.push('.');
                    count = 0;
                }
            }
        }
        Ok(out)
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |c, token| {
            if token == "1" {
                c.ques = 1;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |c| {
            if c.ques == 1 {
                "1 ".into()
            } else {
                "0 ".into()
            }
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }
    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| !(0..=1).contains(&c.ques)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "ringring blocked cell must be 0 or 1",
            );
        }
    }
}

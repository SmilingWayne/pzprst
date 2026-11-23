//! Usoone room borders, numeric clues and liar marks.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Usoone;
struct Usoone;
impl Genre for Usoone {
    fn pid(&self) -> &'static str {
        "usoone"
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
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        encode::decode_4cell(b, &rest, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_border(b),
            encode::encode_4cell(b)
        ))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_area_room(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        r.decode_cell(b, d, |cell, token| {
            if token == "1" {
                cell.qans = 1;
            } else if token != "." {
                if token.starts_with('+') {
                    cell.qsub = 1;
                }
                if token.ends_with('o') {
                    cell.qcmp = 1;
                }
                if token.ends_with('x') {
                    cell.qcmp = 2;
                }
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_area_room(w, b);
        fileio::encode_cell_qnum(w, b);
        w.encode_cell(b, |cell| {
            if cell.qans == 1 {
                return "1 ".into();
            }
            let mut out = String::new();
            if cell.qsub == 1 {
                out.push('+');
            }
            if cell.qcmp == 1 {
                out.push('o');
            } else if cell.qcmp == 2 {
                out.push('x');
            }
            if out.is_empty() {
                out.push('.');
            }
            out.push(' ');
            out
        });
        fileio::encode_border_line_if_present(w, b);
        Ok(())
    }
    fn validate(&self, _b: &Board, _d: &mut Diagnostics) {}
}

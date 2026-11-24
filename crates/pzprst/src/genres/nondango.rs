//! Nondango room borders, circles and player marks.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "nondango"
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
    fn init_board(&self, b: &mut Board) {
        for c in &mut b.cells {
            c.ques = 8;
        }
    }
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, s, d);
        let n = b.cells.len();
        encode::generic_decode_binary(n, &rest, d, |i, on| {
            b.cells[i].ques = if on { 8 } else { 0 }
        })
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_border(b),
            encode::encode_binary(b, |c| c.ques == 8, false)
        ))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_area_room(r, b, d)?;
        r.decode_cell(b, d, |c, s| {
            match s {
                "#" => {
                    c.ques = 0;
                    c.qans = 1;
                }
                "%" => {
                    c.ques = 0;
                    c.qsub = 1;
                }
                "o" => c.ques = 0,
                "+" => {
                    c.ques = 8;
                    c.qsub = 1;
                }
                _ => {}
            }
            Ok(())
        })
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_area_room(w, b);
        w.encode_cell(b, |c| {
            if c.ques == 0 {
                if c.qans == 1 {
                    "# "
                } else if c.qsub == 1 {
                    "% "
                } else {
                    "o "
                }
            } else if c.qsub == 1 {
                "+ "
            } else {
                ". "
            }
            .into()
        });
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
}

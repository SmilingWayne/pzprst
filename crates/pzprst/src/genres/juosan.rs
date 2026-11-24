//! Juosan room clues and vertical/horizontal bar answers.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::Transform;
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "juosan"
    }
    fn hasborder(&self) -> u8 {
        1
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn room_clues(&self) -> bool {
        true
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, s, d);
        encode::decode_room_number16(b, &rest, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_border(b),
            encode::encode_room_number16(b)
        ))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_area_room(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        r.decode_cell(b, d, |c, s| {
            c.qans = match s {
                "1" => 12,
                "2" => 13,
                _ => 0,
            };
            Ok(())
        })
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_area_room(w, b);
        fileio::encode_cell_qnum(w, b);
        w.encode_cell(b, |c| {
            match (c.ques, c.qans) {
                (1, _) => ". ",
                (_, 0) => "0 ",
                (_, 12) => "1 ",
                (_, 13) => "2 ",
                _ => ". ",
            }
            .into()
        });
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        if t.swaps_axes() {
            for c in &mut b.cells {
                c.qans = match c.qans {
                    12 => 13,
                    13 => 12,
                    x => x,
                };
            }
        }
    }
    fn adjust_board_data(&self, _: Transform, b: &mut Board) {
        crate::rooms::normalize_clues(b);
    }
}

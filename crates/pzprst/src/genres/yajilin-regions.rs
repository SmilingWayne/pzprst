//! Regional Yajilin: numbered rooms, shading and loop state.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "yajilin-regions"
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
        &[('o', "yajilin_out")]
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
        r.decode_config_flag(b, "o", "yajilin_out");
        fileio::decode_area_room(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "o", "yajilin_out");
        fileio::encode_area_room(w, b);
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data(&self, _: crate::Transform, b: &mut Board) {
        crate::rooms::normalize_clues(b);
    }
}

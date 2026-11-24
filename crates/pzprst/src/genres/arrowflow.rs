//! Arrow Flow: numeric clues and direction-valued answers.
use super::{g_roma as roma, g_yajikazu as arrows, Genre};
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str { "arrowflow" }
    fn hasborder(&self) -> u8 { 0 }
    fn hascross(&self) -> u8 { 2 }
    fn rule_flags(&self) -> &'static [(char, &'static str)] { &[] }
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, s, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        roma::decode_cell_answers(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        roma::encode_cell_answers(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data_before(&self, t: crate::Transform, b: &mut Board) {
        arrows::adjust_cell_answer_numbers(b, t);
    }
}

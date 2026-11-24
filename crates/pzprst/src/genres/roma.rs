//! Roma: region borders, destination clues, and direction-valued answers.
use super::{g_yajikazu as arrows, Genre};
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str { "roma" }
    fn hasborder(&self) -> u8 { 1 }
    fn hascross(&self) -> u8 { 2 }
    fn rule_flags(&self) -> &'static [(char, &'static str)] { &[] }
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, s, d);
        encode::decode_number10(b, &rest, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!("{}{}", encode::encode_border(b), encode::encode_number10(b)?))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_area_room(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        decode_cell_answers(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_area_room(w, b);
        fileio::encode_cell_qnum(w, b);
        encode_cell_answers(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data_before(&self, t: crate::Transform, b: &mut Board) {
        arrows::adjust_cell_numbers(b, t);
    }
}

/// Roma's `decodeCellAnumsub` uses `+N` for a directional submark.
pub(super) fn decode_cell_answers(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, token| {
        if let Some(n) = token.strip_prefix('+') {
            cell.qsub = fileio::int(n)?;
        } else if token != "." {
            cell.anum = fileio::int(token)?;
        }
        Ok(())
    })
}

/// Roma's `encodeCellAnumsub` writes the numeric directional submark.
pub(super) fn encode_cell_answers(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.anum != -1 {
            format!("{} ", cell.anum)
        } else if cell.qsub != 0 {
            format!("+{} ", cell.qsub)
        } else {
            ". ".into()
        }
    });
}

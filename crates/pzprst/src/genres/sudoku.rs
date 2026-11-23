//! Sudoku: number16 clues and answer numbers with pencil marks.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Sudoku;
struct Sudoku;

impl Genre for Sudoku {
    fn pid(&self) -> &'static str {
        "sudoku"
    }
    fn hasborder(&self) -> u8 {
        1
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn subnumbers(&self) -> bool {
        true
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn init_board(&self, b: &mut Board) {
        let side = (b.cols() as f64).sqrt() as i32 * 2;
        let width = if b.cols() == 6 { 6 } else { side };
        for i in 0..b.borders.len() {
            let (x, y) = b.geom.border_pos(i);
            if x % width == 0 || y % side == 0 {
                b.borders[i].ques = 1;
            }
        }
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, body, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }
    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let max = b.cols().max(b.rows()) as i32;
        if b.cells
            .iter()
            .any(|c| c.qnum != -1 && c.qnum != -2 && !(1..=max).contains(&c.qnum))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("sudoku clue outside 1-{max} or ?"),
            );
        }
    }
}

//! pzprjs nurikabe: URL and FileIO format.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "nurikabe"
    }
    fn hasborder(&self) -> u8 {
        0
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, body, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum_ans(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum_ans(w, b);
        Ok(())
    }
    fn validate(&self, _b: &Board, _d: &mut Diagnostics) {}
}

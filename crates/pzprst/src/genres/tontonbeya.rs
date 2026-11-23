//! pzprjs tontonbeya: URL and FileIO format.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "tontonbeya"
    }
    fn subnumbers(&self) -> bool {
        true
    }
    fn hasborder(&self) -> u8 {
        1
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        encode::decode_number10(b, &rest, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_border(b),
            encode::encode_number10(b)?
        ))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_area_room(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_area_room(w, b);
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }
    fn validate(&self, _b: &Board, _d: &mut Diagnostics) {}
}

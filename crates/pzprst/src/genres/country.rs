//! pzprjs country: URL and FileIO format.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "country"
    }
    fn room_clues(&self) -> bool {
        true
    }
    fn hasborder(&self) -> u8 {
        1
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('e', "country_empty")]
    }
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
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
        if r.filever >= 1 {
            let flags = r.read_line().unwrap_or("");
            b.rules.insert(
                "country_empty",
                flags.split(',').any(|f| f == "country_empty"),
            );
        }
        if r.filever >= 2 {
            fileio::decode_border_ques(r, b, d)?;
        } else {
            fileio::decode_area_room(r, b, d)?;
        }
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_border_line(r, b, d)?;
        fileio::decode_cell_qsub(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.write_line(if b.rules.get("country_empty").copied().unwrap_or(false) {
            "country_empty"
        } else {
            ""
        });
        w.filever = 2;
        fileio::encode_border_ques(w, b);
        fileio::encode_cell_qnum(w, b);
        fileio::encode_border_line(w, b);
        fileio::encode_cell_qsub(w, b);
        Ok(())
    }
    fn validate(&self, _b: &Board, _d: &mut Diagnostics) {}
    fn adjust_board_data(&self, _t: crate::Transform, b: &mut Board) {
        crate::rooms::normalize_clues(b);
    }
}

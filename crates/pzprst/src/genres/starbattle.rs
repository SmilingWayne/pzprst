//! Star Battle: room boundaries and a puzzle-wide star count.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str { "starbattle" }
    fn hasborder(&self) -> u8 { 1 }
    fn hascross(&self) -> u8 { 2 }
    fn rule_flags(&self) -> &'static [(char, &'static str)] { &[] }
    fn init_board(&self, b: &mut Board) { b.indicator = Some(1); }
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let (count, rest) = body.split_once('/').unwrap_or((body, ""));
        b.indicator = Some(jsnum::integer(count).unwrap_or(0));
        encode::decode_border(b, rest, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!("{}/{}", b.indicator.unwrap_or(1), encode::encode_border(b)))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        b.indicator = Some(r.read_line().and_then(jsnum::integer).unwrap_or(0));
        fileio::decode_area_room(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.write_line(&b.indicator.unwrap_or(1).to_string());
        fileio::encode_area_room(w, b);
        fileio::encode_cell_ans(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
}

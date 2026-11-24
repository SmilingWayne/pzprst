//! Remlen room clues, blocked cells, and line-arrow answers.
use super::{g_slalom as arrows, Genre};
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "remlen"
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
        let rest = if let Some(rest) = rest.strip_prefix('/') {
            rest.to_string()
        } else {
            encode::decode_empty(b, &rest, d)
        };
        let rest = rest.strip_prefix('/').unwrap_or(&rest).to_string();
        encode::decode_room_number16(b, &rest, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        let empty = encode::encode_empty(b);
        // encodeEmpty delegates to encodeBinary, which does not return the
        // generic encoder's boolean; pzprjs therefore always appends '/'.
        Ok(format!(
            "{}{}/{}",
            encode::encode_border(b),
            empty,
            encode::encode_room_number16(b)
        ))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        if r.filever >= 2 {
            fileio::decode_border_ques(r, b, d)?;
        } else {
            fileio::decode_area_room(r, b, d)?;
        }
        r.decode_cell(b, d, |c, s| {
            match s {
                "#" => c.ques = 7,
                "-" => c.qnum = -2,
                "." => {}
                _ => c.qnum = fileio::int(s)?,
            }
            Ok(())
        })?;
        arrows::decode_border_arrow_ans(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.filever = 2;
        fileio::encode_border_ques(w, b);
        w.encode_cell(b, |c| {
            let s = if c.ques == 7 {
                "#".into()
            } else if c.qnum >= 0 {
                c.qnum.to_string()
            } else if c.qnum == -2 {
                "-".into()
            } else {
                ".".into()
            };
            format!("{s} ")
        });
        arrows::encode_border_arrow_ans(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data(&self, _: crate::Transform, b: &mut Board) {
        crate::rooms::normalize_clues(b);
    }
}

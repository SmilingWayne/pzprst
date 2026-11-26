//! Sendai-Miyagi: numbered rooms and answer borders.
use super::Genre;
use crate::error::{Diagnostics, Result};
use crate::fileio::{FileReader, FileWriter};
use crate::{encode, fileio, Board, Transform};
pub(crate) static GENRE: &dyn Genre = &Sendai;
struct Sendai;
impl Genre for Sendai {
    fn pid(&self) -> &'static str {
        "sendai"
    }
    fn hasborder(&self) -> u8 {
        1
    }
    fn room_clues(&self) -> bool {
        true
    }
    fn room_border_is_wall(&self, edge: &crate::Obj) -> bool {
        edge.ques == 1
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
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
        fileio::decode_cell_qnum(r, b, d)?;
        r.decode_border(b, d, |edge, token| {
            let mut value = fileio::int(token)?;
            if value & 4 != 0 {
                edge.qsub = 1;
                value &= !4;
            }
            if value == 1 {
                edge.ques = 1;
            } else if value == 2 {
                edge.qans = 1;
            }
            Ok(())
        })
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        w.encode_border(b, |edge| {
            let mut value = if edge.ques != 0 {
                1
            } else if edge.qans != 0 {
                2
            } else {
                0
            };
            if edge.qsub != 0 {
                value |= 4;
            }
            format!("{value} ")
        });
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data(&self, _: Transform, b: &mut Board) {
        crate::rooms::normalize_clues(b);
    }
}

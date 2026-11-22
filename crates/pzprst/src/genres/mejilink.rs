//! Mejilink (pzprjs `variety/mejilink.js`, pid `mejilink`).
//!
//! `Border.ques` defaults to 1, but `Board.initBoardSize` calls
//! `BorderList.allclear`, whose true initial value is `border.inside ? 1 : 0`
//! (inner borders are walls, the outer frame is not). pzprst objects start at
//! their shared default 0, so both decoders set the inner borders to walls
//! before reading.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Mejilink;

struct Mejilink;

/// pzprjs `BorderList.allclear`: every inner border is a wall.
fn set_border_defaults(b: &mut Board) {
    let inner = 2 * b.cols() * b.rows() - b.cols() - b.rows();
    for id in 0..inner {
        if let Some(i) = b.border_by_id(id) {
            b.borders[i].ques = 1;
        }
    }
}

impl Genre for Mejilink {
    fn pid(&self) -> &'static str {
        "mejilink"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        set_border_defaults(b);
        encode::decode_outer_border(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_outer_border(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        set_border_defaults(b);
        let mut unknown = false;
        r.decode_border(b, d, |border, ca| {
            match ca {
                "2" => {
                    border.ques = 0;
                    border.line = 1;
                }
                "-1" => {
                    border.ques = 0;
                    border.qsub = 2;
                }
                "1" => border.ques = 0,
                _ => {
                    // `0` is the wall token; anything else is read as a wall too.
                    border.ques = 1;
                    unknown |= ca != "0";
                }
            }
            Ok(())
        })?;
        if unknown {
            d.push(
                DiagnosticCode::UnexpectedChar,
                "unknown mejilink border token is read as a wall",
            );
        }
        Ok(())
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_border(b, |border| {
            if border.line == 1 {
                "2 ".into()
            } else if border.qsub == 2 {
                "-1 ".into()
            } else if border.ques == 0 {
                "1 ".into()
            } else {
                "0 ".into()
            }
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.ques)
                || !(0..=1).contains(&e.line)
                || !(e.qsub == 0 || e.qsub == 2)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "mejilink border value outside ques 0/1, line 0/1, cross 0/2",
            );
        }
    }
}

//! Combi Block (pzprjs `variety/cbblock.js`, pid `cbblock`). The shared script
//! also serves `dbchoco`, `nikoji` and `mirrorbk`, which override
//! `Encode`/`FileIO` and are separate pids.
//!
//! pzprjs sets the `Border` prototype default `ques = 1` for this pid
//! (`"Border@cbblock#1": { ques: 1 }`), so borders that a URL body does not
//! cover are walls. pzprst objects start at their shared default 0, so both
//! decoders set every border to a wall before reading.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Cbblock;

struct Cbblock;

impl Genre for Cbblock {
    fn pid(&self) -> &'static str {
        "cbblock"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        for border in &mut b.borders {
            border.ques = 1;
        }
        encode::decode_outer_border(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_outer_border(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        for border in &mut b.borders {
            border.ques = 1;
        }
        let mut unknown = false;
        r.decode_border(b, d, |border, ca| {
            match ca {
                "3" => {
                    border.ques = 0;
                    border.qans = 1;
                    border.qsub = 1;
                }
                "1" => {
                    border.ques = 0;
                    border.qans = 1;
                }
                "-1" => {
                    border.ques = 1;
                    border.qsub = 1;
                }
                "-2" => {
                    border.ques = 0;
                    border.qsub = 1;
                }
                "2" => border.ques = 0,
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
                "unknown cbblock border token is read as a wall",
            );
        }
        Ok(())
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_border(b, |border| {
            if border.qans == 1 && border.qsub == 1 {
                "3 ".into()
            } else if border.qans == 1 {
                "1 ".into()
            } else if border.ques == 1 && border.qsub == 1 {
                "-1 ".into()
            } else if border.ques == 0 && border.qsub == 1 {
                "-2 ".into()
            } else if border.ques == 0 {
                "2 ".into()
            } else {
                "0 ".into()
            }
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.ques)
                || !(0..=1).contains(&e.qans)
                || !(0..=1).contains(&e.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "cbblock border value outside ques/qans/qsub 0 or 1",
            );
        }
    }
}

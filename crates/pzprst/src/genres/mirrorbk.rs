//! Mirror Block (pzprjs `variety/cbblock.js`, pid `mirrorbk`). Combi Block
//! (`cbblock`), Double Choco (`dbchoco`) and NIKOJI (`nikoji`) share that
//! script but override `Encode`/`FileIO`, so they are separate pids.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Mirrorbk;

struct Mirrorbk;

impl Genre for Mirrorbk {
    fn pid(&self) -> &'static str {
        "mirrorbk"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_number16(b, body, d);
        encode::decode_border(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_number16(b),
            encode::encode_border(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        let mut wall = false;
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
                    // `mirrorbk` FileIO: every other token is a wall.
                    border.ques = 1;
                    wall |= ca != "0";
                }
            }
            Ok(())
        })?;
        if wall {
            d.push(
                DiagnosticCode::UnexpectedChar,
                "unknown mirrorbk border token is read as a wall",
            );
        }
        Ok(())
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
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
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "mirrorbk cell value outside empty / ? / clue 1..cols*rows",
            );
        }
        if b.borders.iter().any(|e| {
            !matches!(e.ques, 0 | 1) || !matches!(e.qans, 0 | 1) || !matches!(e.qsub, 0 | 1)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "mirrorbk border value outside ques/qans/qsub 0 or 1",
            );
        }
    }
}

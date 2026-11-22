//! Tatamibari (pzprjs `variety/tatamibari.js`, pid `tatamibari`).
//!
//! A cell's `qnum` is 1 (horizontal bar), 2 (vertical bar), 3 (square) or -2
//! (`?`); the URL letters `3`/`1`/`2` stand for those three clues in that
//! order. FileIO stores `c`/`a`/`b`/`-` and then the answer border layer
//! (`decodeBorderAns`). pzprjs `BoardExec.adjustBoardData` swaps the horizontal
//! and vertical bar under every turn (`tques` 2↔3).

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Tatamibari;

struct Tatamibari;

impl Genre for Tatamibari {
    fn pid(&self) -> &'static str {
        "tatamibari"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let chars: Vec<char> = body.chars().collect();
        let n = b.cells.len();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() {
            let ca = chars[i];
            let Some(cell) = b.cells.get_mut(c) else {
                break;
            };
            match ca {
                '.' => cell.qnum = -2,
                '1' => cell.qnum = 2,
                '2' => cell.qnum = 3,
                '3' => cell.qnum = 1,
                'g'..='z' => {
                    c += jsnum::digit(ca, 36).unwrap() as usize - 16;
                }
                // pzprjs skips two cells for an unrecognized character.
                _ => {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("unexpected character {ca:?} skips two cells"),
                    );
                    c += 1;
                }
            }
            c += 1;
            if c >= n {
                break;
            }
            i += 1;
        }
        if c < n {
            d.push(
                DiagnosticCode::TruncatedBody,
                "body ends before the last cell",
            );
        }
        chars.iter().skip(i + 1).collect()
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut cm = String::new();
        let mut count = 0u32;
        for cell in &b.cells {
            let pstr = match cell.qnum {
                -2 => ".",
                1 => "3",
                2 => "1",
                3 => "2",
                _ => {
                    count += 1;
                    ""
                }
            };
            if count == 0 {
                cm.push_str(pstr);
            } else if !pstr.is_empty() || count == 20 {
                cm.push_str(&jsnum::to_radix(15 + count, 36));
                cm.push_str(pstr);
                count = 0;
            }
        }
        if count > 0 {
            cm.push_str(&jsnum::to_radix(15 + count, 36));
        }
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "a" {
                cell.qnum = 2;
            } else if ca == "b" {
                cell.qnum = 3;
            } else if ca == "c" {
                cell.qnum = 1;
            } else if ca == "-" {
                cell.qnum = -2;
            }
            Ok(())
        })?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| match cell.qnum {
            -2 => "- ".into(),
            1 => "c ".into(),
            2 => "a ".into(),
            3 => "b ".into(),
            _ => ". ".into(),
        });
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !matches!(c.qnum, -2 | -1 | 1 | 2 | 3))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tatamibari cell value outside horizontal 1 / vertical 2 / square 3 / ?",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tatamibari border value outside answer 0/1 or cross 0/1",
            );
        }
    }

    /// `BoardExec.adjustBoardData` under `key & TURN`: `tques` 2↔3.
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        for op in ops(t) {
            if !matches!(op, Op::TurnR | Op::TurnL) {
                continue;
            }
            for cell in &mut b.cells {
                cell.qnum = match cell.qnum {
                    2 => 3,
                    3 => 2,
                    q => q,
                };
            }
        }
    }
}

/// The pzprjs board operations a composed [`Transform`] stands for.
#[derive(Clone, Copy)]
enum Op {
    TurnR,
    TurnL,
    FlipX,
    FlipY,
}

fn ops(t: Transform) -> &'static [Op] {
    match t {
        Transform::Identity => &[],
        Transform::Rot90 => &[Op::TurnR],
        Transform::Rot180 => &[Op::TurnR, Op::TurnR],
        Transform::Rot270 => &[Op::TurnL],
        Transform::FlipX => &[Op::FlipX],
        Transform::FlipY => &[Op::FlipY],
        Transform::Transpose => &[Op::TurnL, Op::FlipY],
        Transform::AntiTranspose => &[Op::TurnR, Op::FlipY],
    }
}

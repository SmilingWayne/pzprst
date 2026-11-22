//! Nanameguri / Gokigen Naname (pzprjs `variety/nanameguri.js`, pid
//! `nanameguri`).
//!
//! Cells hold a diagonal slash in `ques` (0 none, 31 `\`, 32 `/`) and borders
//! hold the loop in `line`; `border.ques` 1 is a wall. The URL is the shared
//! border-wall layer (`decodeBorder`) followed by the base-27 triple layer
//! (`genericDecodeThree`) of the cell slashes; FileIO stores the walls, the
//! cell layer and then the answer lines. Every rotation/reflection swaps the
//! two slash directions (`adjustBoardData` in `BoardExec`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Nanameguri;

struct Nanameguri;

impl Genre for Nanameguri {
    fn pid(&self) -> &'static str {
        "nanameguri"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        let n = b.cells.len();
        let cells = &mut b.cells;
        encode::decode_three(n, &rest, d, |c, val| {
            if val > 0 {
                cells[c].ques = val + 30;
            }
        })
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_border(b);
        body.push_str(&encode::encode_three(b.cells.len(), |c| {
            (b.cells[c].ques - 30).max(0)
        })?);
        Ok(body)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_border_ques(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            if ca == "1" {
                cell.ques = 31;
            } else if ca == "2" {
                cell.ques = 32;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_border_ques(w, b);
        w.encode_cell(b, |cell| match cell.ques {
            31 => "1 ".into(),
            32 => "2 ".into(),
            _ => ". ".into(),
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| !matches!(c.ques, 0 | 31 | 32)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "nanameguri cell value outside empty / \\ (31) / / (32)",
            );
        }
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.ques)
                || !(0..=1).contains(&e.line)
                || !(e.qsub == 0 || e.qsub == 2)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "nanameguri border value outside wall 0/1, line 0/1 or cross -1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData` (nanameguri.js): every operation of
    /// the turn/flip group swaps the two diagonal slashes.
    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        for _ in turn_ops(t) {
            for cell in &mut b.cells {
                cell.ques = match cell.ques {
                    31 => 32,
                    32 => 31,
                    other => other,
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

fn turn_ops(t: Transform) -> &'static [Op] {
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

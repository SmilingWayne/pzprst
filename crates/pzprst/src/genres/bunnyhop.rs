//! Bunny Hop (pzprjs `variety/bunnyhop.js`, pid `bunnyhop`).
//!
//! The URL is the `decodeEmpty`/`encodeEmpty` question layer (`ques === 7`).
//! FileIO stores the same `#` layer, the directed border lines and the cell
//! `qsub` marks: the low four bits are peke directions, bits 4–6 half-line
//! directions. `adjustBoardData` turns both the line direction and the peke /
//! half bits for every rotation and reflection, so the genre overrides
//! `adjust_board_data_before`.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Bunnyhop;

struct Bunnyhop;

/// One pzprjs `BoardExec` rotation/reflection operation.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    TurnR,
    TurnL,
    FlipX,
    FlipY,
}

impl Op {
    fn turns(self) -> bool {
        matches!(self, Op::TurnR | Op::TurnL)
    }

    /// pzprjs `adjustBoardData`'s `flipvert` / `fliphorz` line test.
    fn flips_line(self, vertical: bool) -> bool {
        match self {
            Op::TurnL | Op::FlipX => vertical,
            Op::TurnR | Op::FlipY => !vertical,
        }
    }
}

/// The pzprjs operations `transform` is composed of, in order (tools/gen_fixtures.js).
fn operations(t: Transform) -> &'static [Op] {
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

/// `getTranslatePekes`: map table for the low four `qsub` bits.
fn translate_pekes(op: Op) -> &'static [(i32, i32)] {
    match op {
        Op::FlipY => &[
            (1, 2),
            (2, 1),
            (5, 6),
            (6, 5),
            (9, 10),
            (10, 9),
            (13, 14),
            (14, 13),
        ],
        Op::FlipX => &[
            (4, 8),
            (5, 9),
            (6, 10),
            (7, 11),
            (8, 4),
            (9, 5),
            (10, 6),
            (11, 7),
        ],
        Op::TurnR => &[
            (1, 8),
            (2, 4),
            (3, 12),
            (4, 1),
            (5, 9),
            (6, 5),
            (7, 13),
            (8, 2),
            (9, 10),
            (10, 6),
            (11, 14),
            (12, 3),
            (13, 11),
            (14, 7),
        ],
        Op::TurnL => &[
            (1, 4),
            (2, 8),
            (3, 12),
            (4, 2),
            (5, 6),
            (6, 10),
            (7, 14),
            (8, 1),
            (9, 5),
            (10, 9),
            (11, 13),
            (12, 3),
            (13, 7),
            (14, 11),
        ],
    }
}

/// `getTranslateHalves`: map table for `qsub >> 4 & 7`.
fn translate_halves(op: Op) -> &'static [(i32, i32)] {
    match op {
        Op::FlipY => &[(1, 3), (3, 1), (2, 4), (4, 2)],
        Op::FlipX => &[(1, 2), (2, 1), (3, 4), (4, 3)],
        Op::TurnR => &[(1, 2), (2, 4), (4, 3), (3, 1)],
        Op::TurnL => &[(1, 3), (3, 4), (4, 2), (2, 1)],
    }
}

fn lookup(table: &[(i32, i32)], value: i32) -> Option<i32> {
    table
        .iter()
        .find(|&&(from, _)| from == value)
        .map(|&(_, to)| to)
}

fn is_vertical(b: &Board, i: usize) -> bool {
    let (bx, _) = b.geom.border_pos(i);
    bx & 1 == 0
}

impl Genre for Bunnyhop {
    fn pid(&self) -> &'static str {
        "bunnyhop"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_empty(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_empty(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            cell.ques = if ca == "#" { 7 } else { 0 };
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)?;
        fileio::decode_cell_qsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 7 {
                "# ".into()
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_line(w, b);
        fileio::encode_cell_qsub(w, b);
        Ok(())
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // Reproduce the pzprjs call sequence one operation at a time. A turn
        // swaps the orientation of every border, so recover the orientation
        // the border had before the transform and replay the per-operation
        // `flipvert`/`fliphorz` tests; a line is inverted once per flip.
        let ops = operations(t);
        let turned = ops.iter().filter(|op| op.turns()).count() % 2 == 1;
        let vertical: Vec<bool> = (0..b.borders.len()).map(|i| is_vertical(b, i)).collect();
        for (i, border) in b.borders.iter_mut().enumerate() {
            if border.line == 0 {
                continue;
            }
            let mut v = vertical[i] ^ turned;
            let mut flipped = false;
            for &op in ops {
                if op.flips_line(v) {
                    flipped = !flipped;
                }
                if op.turns() {
                    v = !v;
                }
            }
            if flipped {
                border.line = 3 - border.line;
            }
        }
        for &op in ops {
            let pekes = translate_pekes(op);
            let halves = translate_halves(op);
            for cell in &mut b.cells {
                let peke = cell.qsub & 15;
                let half = (cell.qsub >> 4) & 7;
                let peke = lookup(pekes, peke).unwrap_or(peke);
                // `halfTrans[half]` is `undefined` for 0 and 5-7; JS shifts it to 0.
                let half = lookup(halves, half).unwrap_or(0);
                cell.qsub = peke | (half << 4);
            }
        }
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.ques == 0 || c.ques == 7)
                || c.qnum != -1
                || !(0..=127).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "bunnyhop cell value outside empty/shaded, peke bits 0-127",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=2).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "bunnyhop border value outside line 0-2, cross -1",
            );
        }
    }
}

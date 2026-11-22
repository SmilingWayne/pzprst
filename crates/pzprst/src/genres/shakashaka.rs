//! Shakashaka (pzprjs `variety/shakashaka.js`, pid `shakashaka`).
//!
//! Cells carry a clue number in `qnum` (0–4, `?` is -2); answers are triangles
//! in `qans` (2 bottom-left, 3 bottom-right, 4 top-right, 5 top-left), the
//! black dot in `qsub` and the "clue done" mark in `qcmp`. The URL is
//! `decode4Cell`/`encode4Cell`; FileIO writes the clue layer with
//! `decodeCellQnumb` and then the answer layer with `decodeCellQanssubcmp`.
//! pzprjs `adjustBoardData` rotates/reflection-maps the triangle `qans` values,
//! so the genre overrides `adjust_board_data_before`.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Shakashaka;

struct Shakashaka;

impl Genre for Shakashaka {
    fn pid(&self) -> &'static str {
        "shakashaka"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_4cell(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_4cell(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnumb(r, b, d)?;
        decode_cell_qanssubcmp(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnumb(w, b);
        encode_cell_qanssubcmp(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=4).contains(&c.qnum))
                || !matches!(c.qans, 0 | 2 | 3 | 4 | 5)
                || !(c.qsub == 0 || c.qsub == 1)
                || !(c.qcmp == 0 || c.qcmp == 1)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "shakashaka cell value outside clue 0-4 / ? / triangle 2-5 / background mark",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`: every quarter turn and mirror
    /// permutes the four triangle orientations. pzprjs runs it before the
    /// coordinates move, and the value-only permutation commutes with the move.
    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        for &op in ops(t) {
            for cell in &mut b.cells {
                cell.qans = rotate_qans(op, cell.qans);
            }
        }
    }
}

/// `decodeCellQanssubcmp`: triangle `qans`, black dot `qsub`, clue-done `qcmp`.
fn decode_cell_qanssubcmp(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "+" {
            cell.qsub = 1;
        } else if ca == "-" {
            cell.qcmp = 1;
        } else if ca != "." {
            cell.qans = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQanssubcmp`: triangle first, then the black dot, the clue-done
/// mark and `.`.
fn encode_cell_qanssubcmp(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qans != 0 {
            format!("{} ", cell.qans)
        } else if cell.qsub == 1 {
            "+ ".into()
        } else if cell.qcmp == 1 {
            "- ".into()
        } else {
            ". ".into()
        }
    });
}

/// One pzprjs `BoardExec` rotation/reflection operation.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    TurnR,
    TurnL,
    FlipX,
    FlipY,
}

/// The pzprjs operations a composed [`Transform`] stands for, in
/// `tools/gen_fixtures.js` order: `rot90` = `turnr`, `rot180` = `turnr`+
/// `turnr`, `transpose` = `turnl`+`flipy`, `antitranspose` = `turnr`+`flipy`.
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

/// `adjustBoardData`'s `trans` tables; like pzprjs `trans[cell.qans]`, a value
/// outside the table is left alone.
fn rotate_qans(op: Op, qans: i32) -> i32 {
    let trans: &[i32; 6] = match op {
        Op::FlipY => &[0, 1, 5, 4, 3, 2],
        Op::FlipX => &[0, 1, 3, 2, 5, 4],
        Op::TurnR => &[0, 1, 5, 2, 3, 4],
        Op::TurnL => &[0, 1, 3, 4, 5, 2],
    };
    trans.get(qans as usize).copied().unwrap_or(qans)
}

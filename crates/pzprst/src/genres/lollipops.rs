//! Lollipops (pzprjs `variety/lollipops.js`, pid `lollipops`).
//!
//! A cell holds an object number in `qnum` (1 circle, 2/3 the two bar
//! orientations, -2 `?`) and the same in `anum` for the answer. FileIO also
//! stores the `qcmp` mark and the `qsub` background. The board has no borders
//! and the URL only stores `qnum`.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Lollipops;

struct Lollipops;

impl Genre for Lollipops {
    fn pid(&self) -> &'static str {
        "lollipops"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number10(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        encode::encode_number10(b)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=3).contains(&c.qnum))
                || !(c.anum == -1 || (1..=3).contains(&c.anum))
                || !(0..=4).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "lollipops cell value outside object 1-3/? or answer mark",
            );
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData`: on every quarter turn the two bar
        // orientations swap (qnum 2/3, else anum 2/3).
        for _ in 0..turns(t) {
            for cell in &mut b.cells {
                if cell.qnum > 1 {
                    cell.qnum = 5 - cell.qnum;
                } else if cell.anum > 1 {
                    cell.anum = 5 - cell.anum;
                }
            }
        }
    }
}

/// Number of quarter turns in the board operations of a transform (`flipy`
/// adds no turn). pzprjs calls `adjustBoardData` once per operation.
fn turns(t: Transform) -> usize {
    match t {
        Transform::Rot90 | Transform::Rot270 | Transform::Transpose | Transform::AntiTranspose => {
            1
        }
        Transform::Rot180 => 2,
        Transform::Identity | Transform::FlipX | Transform::FlipY => 0,
    }
}

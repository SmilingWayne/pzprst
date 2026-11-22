//! Slash Pack (pzprjs `variety/slashpack.js`, pid `slashpack`).
//!
//! Cells carry a room number in `qnum` and the answer slashes in `qans`
//! (31 or 32) plus the four corner circles in `qsub` (three bits). The URL is
//! the shared `decodeNumber16`/`encodeNumber16`; FileIO adds the slash layer.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Slashpack;

struct Slashpack;

impl Genre for Slashpack {
    fn pid(&self) -> &'static str {
        "slashpack"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            let mut rest = ca;
            if let Some(first) = ca.chars().next() {
                let code = first as u32;
                if code > 110 && code <= 117 {
                    cell.qsub = code as i32 - 110;
                    rest = &ca[first.len_utf8()..];
                }
            }
            if let Some(qans) = jsnum::integer(rest) {
                if qans != 0 {
                    cell.qans = qans + 30;
                }
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        w.encode_cell(b, |cell| {
            let mut s = String::new();
            if cell.qsub > 0 {
                s.push(char::from_u32(110 + cell.qsub as u32).unwrap_or('?'));
            }
            if cell.qans > 30 {
                s.push_str(&(cell.qans - 30).to_string());
            } else if s.is_empty() {
                s.push('.');
            }
            format!("{s} ")
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(c.qans == 0 || c.qans == 31 || c.qans == 32)
                || !(0..=7).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("slashpack cell value outside clue 1-{maxnum} / ? / slash 31-32 or corner bits"),
            );
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        apply_ops(t, |op| {
            for cell in &mut b.cells {
                cell.qans = flip_slash(op, cell.qans);
            }
        });
    }
}

/// The pzprjs board operations behind a composed `Transform`, in `BoardExec`
/// order: `rot90` = `turnr`, `rot180` = `turnr`+`turnr`, `transpose` =
/// `turnl`+`flipy`, `antitranspose` = `turnr`+`flipy`.
fn apply_ops(t: Transform, mut op: impl FnMut(Transform)) {
    match t {
        Transform::Identity => {}
        Transform::Rot90 => op(Transform::Rot90),
        Transform::Rot180 => {
            op(Transform::Rot90);
            op(Transform::Rot90);
        }
        Transform::Rot270 => op(Transform::Rot270),
        Transform::FlipX => op(Transform::FlipX),
        Transform::FlipY => op(Transform::FlipY),
        Transform::Transpose => {
            op(Transform::Rot270);
            op(Transform::FlipY);
        }
        Transform::AntiTranspose => {
            op(Transform::Rot90);
            op(Transform::FlipY);
        }
    }
}

/// `adjustBoardData`'s `{0: 0, 31: 32, 32: 31}` map. pzprjs assigns `undefined`
/// to any other value; an all-zero value encodes the same, so keep 0 here.
fn flip_slash(_op: Transform, qans: i32) -> i32 {
    match qans {
        31 => 32,
        32 => 31,
        _ => 0,
    }
}

//! Fire Walk (pzprjs `variety/icewalk.js`, pid `firewalk`).
//!
//! Shared walk format: `encodeIce` (a base-32 layer of `ques = 6` fire cells)
//! followed by `encodeNumber16` (clues 1…`cols * rows`, `.` for `?`), with URL
//! flag `f` (`loop_full`). FileIO stores fire as `#` (or `A`/`B`/`C` together
//! with the answer arc `qans` 1–3), `?` as `-`, clues as their decimal number,
//! the answer border lines through `decodeBorderLine`, and — this is the only
//! walk pid that does — a cross layer of `qsub` dot marks. `BoardExec@firewalk`
//! also swaps the two arc orientations (`qans` 1 ↔ 2) under `turnl`/`turnr`/
//! `flipx`/`flipy`, i.e. once per board operation, so `rot180` and both diagonal
//! transforms leave them alone.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Firewalk;

struct Firewalk;

impl Genre for Firewalk {
    fn pid(&self) -> &'static str {
        "firewalk"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    /// pzprjs `Board.hascross` (the inherited default 2: crosses exist on the
    /// even/even points of the whole board, outer frame included).
    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('f', "loop_full")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_ice(b, body, d);
        encode::decode_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_ice(b),
            encode::encode_number16(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "f", "loop_full");
        r.decode_cell(b, d, |cell, ca| {
            if ca == "A" {
                cell.ques = 6;
                cell.qans = 1;
            } else if ca == "B" {
                cell.ques = 6;
                cell.qans = 2;
            } else if ca == "C" {
                cell.ques = 6;
                cell.qans = 3;
            } else if ca == "#" {
                cell.ques = 6;
            } else if ca == "-" {
                cell.qnum = -2;
            } else if ca != "." {
                cell.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)?;
        r.decode_cross(b, d, false, |cross, ca| {
            cross.qsub = fileio::int(ca)?;
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "f", "loop_full");
        w.encode_cell(b, |cell| {
            if cell.qans > 0 {
                // pzprjs `String.fromCharCode(64 + cell.qans)`.
                let ca = char::from_u32(64 + cell.qans as u32).unwrap_or('\u{fffd}');
                format!("{ca} ")
            } else if cell.ques == 6 {
                "# ".into()
            } else if cell.qnum == -2 {
                "- ".into()
            } else if cell.qnum >= 0 {
                format!("{} ", cell.qnum)
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_line(w, b);
        if b.crosses.iter().any(|cross| cross.qsub != 0) {
            w.encode_cross(b, false, |cross| format!("{} ", cross.qsub));
        }
        Ok(())
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec@firewalk.adjustBoardData`: an arc whose `qans` is 1
        // curves the other way once the board is turned or flipped. pzprjs runs
        // it once per board operation, so it is applied per operation here too.
        apply_ops(t, |_op| {
            for cell in &mut b.cells {
                cell.qans = match cell.qans {
                    1 => 2,
                    2 => 1,
                    qans => qans,
                };
            }
        });
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cell.maxnum`: cols * rows (minnum is the default 1).
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.ques == 0 || c.ques == 6)
                || !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(0..=3).contains(&c.qans)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!(
                    "firewalk cell value outside empty/fire, clue 1-{maxnum}/?, or answer arc 0-3"
                ),
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "firewalk border value outside line 0/1, cross -1",
            );
        }
        if b.crosses.iter().any(|c| !(0..=2).contains(&c.qsub)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "firewalk cross value outside empty / dot / cross mark",
            );
        }
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

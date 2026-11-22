//! Myopia (pzprjs `variety/myopia.js`, pid `myopia`).
//!
//! `qnum` is a 4-bit arrow mask (`1` up, `2` down, `4` left, `8` right, bit
//! order `1 << (dir - 1)`), -2 is `?` and -1 is no arrow. The URL is
//! `decodeNumber16`; the `f`
//! flag is the `slither_full` rule. FileIO stores the `f` line, the clue layer,
//! the background-mark (`qsub`) layer and the border line layer. Rotations and
//! reflections map the arrow mask (`adjustCellArrow` in
//! `BoardExec.adjustBoardData`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Myopia;

struct Myopia;

impl Genre for Myopia {
    fn pid(&self) -> &'static str {
        "myopia"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('f', "slither_full")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "f", "slither_full");
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_qsub(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "f", "slither_full");
        w.encode_cell(b, |cell| match cell.qnum {
            n if n >= 0 => format!("{n} "),
            -2 => "- ".into(),
            _ => ". ".into(),
        });
        fileio::encode_cell_qsub(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` → `adjustCellArrow`. Because
        // `Cell.numberAsObject` is set, both `qnum` and `anum` are mapped;
        // the translation covers the 1-bit and combined arrow masks.
        for op in turn_ops(t) {
            for cell in &mut b.cells {
                if let Some(v) = translate_arrow(*op, cell.qnum) {
                    cell.qnum = v;
                }
                if let Some(v) = translate_arrow(*op, cell.anum) {
                    cell.anum = v;
                }
            }
        }
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum` 15; the URL and FileIO can also keep 0 and `?` (-2).
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -2 || (0..=15).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "myopia cell value outside arrow mask 1-15/?",
            );
        }
        if b.cells.iter().any(|c| !(0..=4).contains(&c.qsub)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "myopia cell background mark outside 0-4",
            );
        }
        if b.borders
            .iter()
            .any(|e| e.line < 0 || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "myopia border value outside line >=0, cross -1",
            );
        }
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Right,
    Left,
    FlipX,
    FlipY,
}

/// pzprjs `BoardExec.getTranslateDir` (myopia's arrow-mask override).
fn translate_arrow(op: Turn, val: i32) -> Option<i32> {
    Some(match (op, val) {
        (Turn::FlipY, 1) => 2,
        (Turn::FlipY, 2) => 1,
        (Turn::FlipY, 5) => 6,
        (Turn::FlipY, 6) => 5,
        (Turn::FlipY, 9) => 10,
        (Turn::FlipY, 10) => 9,
        (Turn::FlipY, 13) => 14,
        (Turn::FlipY, 14) => 13,
        (Turn::FlipX, 4) => 8,
        (Turn::FlipX, 5) => 9,
        (Turn::FlipX, 6) => 10,
        (Turn::FlipX, 7) => 11,
        (Turn::FlipX, 8) => 4,
        (Turn::FlipX, 9) => 5,
        (Turn::FlipX, 10) => 6,
        (Turn::FlipX, 11) => 7,
        (Turn::Right, 1) => 8,
        (Turn::Right, 2) => 4,
        (Turn::Right, 3) => 12,
        (Turn::Right, 4) => 1,
        (Turn::Right, 5) => 9,
        (Turn::Right, 6) => 5,
        (Turn::Right, 7) => 13,
        (Turn::Right, 8) => 2,
        (Turn::Right, 9) => 10,
        (Turn::Right, 10) => 6,
        (Turn::Right, 11) => 14,
        (Turn::Right, 12) => 3,
        (Turn::Right, 13) => 11,
        (Turn::Right, 14) => 7,
        (Turn::Left, 1) => 4,
        (Turn::Left, 2) => 8,
        (Turn::Left, 3) => 12,
        (Turn::Left, 4) => 2,
        (Turn::Left, 5) => 6,
        (Turn::Left, 6) => 10,
        (Turn::Left, 7) => 14,
        (Turn::Left, 8) => 1,
        (Turn::Left, 9) => 5,
        (Turn::Left, 10) => 9,
        (Turn::Left, 11) => 13,
        (Turn::Left, 12) => 3,
        (Turn::Left, 13) => 7,
        (Turn::Left, 14) => 11,
        _ => return None,
    })
}

/// The pzprjs board operations a pzprst transform is composed of, in order
/// (`rot90` = turnr, `transpose` = turnl + flipy, …).
fn turn_ops(t: Transform) -> &'static [Turn] {
    match t {
        Transform::Identity => &[],
        Transform::Rot90 => &[Turn::Right],
        Transform::Rot180 => &[Turn::Right, Turn::Right],
        Transform::Rot270 => &[Turn::Left],
        Transform::FlipX => &[Turn::FlipX],
        Transform::FlipY => &[Turn::FlipY],
        Transform::Transpose => &[Turn::Left, Turn::FlipY],
        Transform::AntiTranspose => &[Turn::Right, Turn::FlipY],
    }
}

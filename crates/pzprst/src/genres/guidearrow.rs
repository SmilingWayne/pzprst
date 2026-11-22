//! Guide Arrow (pzprjs `variety/guidearrow.js`, pid `guidearrow`).
//!
//! Every cell holds an arrow direction in `qnum` (1 up, 2 down, 3 left,
//! 4 right; -2 is `?`), shading in `qans` and a background mark in `qsub`.
//! The goal is pzprjs's extra `board.goalpos` object; pzprst's board model has
//! no extra objects, so the goal lives on the marker cell `qdir = 1` with the
//! doubled `bx`/`by` in `qnum3`/`qnum4`. `(1, 1)` (pzprjs `initExtraObject`) is
//! left implicit, like the empty URL body a fresh pzprjs board starts from.
//!
//! The URL is two `writeNumber16` goal coordinates followed by `decode4Cell`;
//! FileIO stores the goal line, `decodeCellQnum`, `decodeCellAns` and the border
//! line layer (only when it has content).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &GuideArrow;

struct GuideArrow;

impl Genre for GuideArrow {
    fn pid(&self) -> &'static str {
        "guidearrow"
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
        let mut i = 0usize;
        let bx = decode_one_number16(&chars, &mut i, d);
        let by = decode_one_number16(&chars, &mut i, d);
        set_goal(b, (2 * bx - 1, 2 * by - 1));
        let rest: String = chars[i.min(chars.len())..].iter().collect();
        encode::decode_4cell(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let (bx, by) = goal(b);
        let mut cm = encode::write_number16((i64::from(bx) + 1) / 2);
        cm += &encode::write_number16((i64::from(by) + 1) / 2);
        cm += &encode::encode_4cell(b);
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_g(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_g(b, w);
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line_if_present(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum` 4; the URL codecs also produce the legacy clue 0.
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=4).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "guidearrow cell value outside empty/?/arrow 0-4, shade 0/1 or sub 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| e.line < 0 || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "guidearrow border value outside line >= 0 or cross -1",
            );
        }
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` → `adjustCellArrow`; guidearrow has
        // `numberAsObject`, so `qnum` and `anum` hold the direction values.
        for op in ops(t) {
            for cell in &mut b.cells {
                if let Some(dir) = translate_dir(*op, cell.qnum) {
                    cell.qnum = dir;
                }
                if let Some(dir) = translate_dir(*op, cell.anum) {
                    cell.anum = dir;
                }
            }
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData2`: the goal follows the operation.
        // The two pzprjs hooks only record the mapped position before the move
        // and restore it after; computing the same map on the transformed board
        // is equivalent. The operations run in `tools/gen_fixtures.js` order.
        let ops = ops(t);
        if ops.is_empty() {
            return;
        }
        let mut cols = if t.swaps_axes() { b.rows() } else { b.cols() } as i32;
        let mut rows = if t.swaps_axes() { b.cols() } else { b.rows() } as i32;
        let mut pos = stored_goal(b).unwrap_or((1, 1));
        for op in ops {
            pos = map_pos(*op, pos, cols, rows);
            if matches!(op, Op::TurnR | Op::TurnL) {
                std::mem::swap(&mut cols, &mut rows);
            }
        }
        set_goal(b, pos);
    }
}

/// The pzprjs board operations a composed [`Transform`] stands for.
#[derive(Clone, Copy, PartialEq, Eq)]
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

/// pzprjs `getAfterPos` for the turn/flip operations.
fn map_pos(op: Op, pos: (i32, i32), cols: i32, rows: i32) -> (i32, i32) {
    let (bx, by) = pos;
    match op {
        Op::TurnR => (2 * rows - by, bx),
        Op::TurnL => (by, 2 * cols - bx),
        Op::FlipX => (2 * cols - bx, by),
        Op::FlipY => (bx, 2 * rows - by),
    }
}

/// The explicit goal position if the board carries a marker.
fn stored_goal(b: &Board) -> Option<(i32, i32)> {
    b.cells
        .iter()
        .find(|cell| cell.qdir == 1)
        .map(|cell| (cell.qnum3, cell.qnum4))
}

/// The goal position, explicit marker or pzprjs `initExtraObject` default.
fn goal(b: &Board) -> (i32, i32) {
    stored_goal(b).unwrap_or((1, 1))
}

/// Store the goal on cell 0, normalizing the default to "no marker".
fn set_goal(b: &mut Board, pos: (i32, i32)) {
    for cell in &mut b.cells {
        if cell.qdir == 1 {
            cell.qdir = 0;
            cell.qnum3 = -1;
            cell.qnum4 = -1;
        }
    }
    if pos == (1, 1) {
        return;
    }
    let cell = &mut b.cells[0];
    cell.qdir = 1;
    cell.qnum3 = pos.0;
    cell.qnum4 = pos.1;
}

/// `decodeOneNumber16`: one URL number, leaving the rest for the caller.
fn decode_one_number16(chars: &[char], i: &mut usize, d: &mut Diagnostics) -> i32 {
    let (value, used) = encode::read_number16(chars, *i);
    *i += used;
    match value {
        Some(v) => v,
        None => {
            d.push(
                DiagnosticCode::UnexpectedChar,
                "incomplete hex escape leaves the goal coordinate at -1",
            );
            -1
        }
    }
}

/// `decodeG`: the goal position line.
fn decode_g(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let line = r.read_line().unwrap_or("").to_string();
    let mut parts = line.split(' ');
    let mut coord = |what: &str| -> i32 {
        let token = parts.next().unwrap_or("");
        match jsnum::parse_int(token, 10) {
            Some(n) => n as i32,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("no base-10 goal {what} in the goal line"),
                );
                0
            }
        }
    };
    let pos = (coord("x"), coord("y"));
    set_goal(b, pos);
    Ok(())
}

/// `encodeG`.
fn encode_g(b: &Board, w: &mut FileWriter) {
    let (bx, by) = goal(b);
    w.write_line(&format!("{bx} {by}"));
}

/// pzprjs `BoardExec.getTranslateDir`, as one direction value mapping.
fn translate_dir(op: Op, dir: i32) -> Option<i32> {
    match (op, dir) {
        (Op::TurnR, 1) => Some(4),
        (Op::TurnR, 2) => Some(3),
        (Op::TurnR, 3) => Some(1),
        (Op::TurnR, 4) => Some(2),
        (Op::TurnL, 1) => Some(3),
        (Op::TurnL, 2) => Some(4),
        (Op::TurnL, 3) => Some(2),
        (Op::TurnL, 4) => Some(1),
        (Op::FlipX, 3) => Some(4),
        (Op::FlipX, 4) => Some(3),
        (Op::FlipY, 1) => Some(2),
        (Op::FlipY, 2) => Some(1),
        _ => None,
    }
}

//! Building Walk (pzprjs `variety/haisu.js`, pid `bdwalk`): a walk from a
//! start cell to a goal cell with up/down elevator clues.
//!
//! pzprjs keeps the start and goal in the extra `board.startpos`/`goalpos`
//! objects; pzprst's board model has no extra objects, so one marker cell holds
//! both positions: `qdir = 1`, `qnum3`/`qnum4` are the start's doubled `bx`/`by`
//! and `qnum2`/`qsub` the goal's. Positions equal to pzprjs `initExtraObject`
//! are left implicit (no marker), exactly like the empty URL body a fresh
//! pzprjs board starts from.
//!
//! The URL is four `writeNumber16` coordinates (`startpos.bx`, `by`, then
//! `goalpos.bx`, `by`, divided by two) followed by the clue layer; FileIO stores
//! the four doubled coordinates as a decimal line. The `m` URL flag / FileIO
//! line is pzprjs's `bdwalk_height` config, which pzprjs writes when the config
//! is *false*; `Board.rules` follows the pzprst convention (the flag letter is
//! present), so the stored boolean is the negation of the pzprjs config value.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &BdWalk;

struct BdWalk;

impl Genre for BdWalk {
    fn pid(&self) -> &'static str {
        "bdwalk"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        // `outpflag = !getConfig("bdwalk_height") ? "m" : null`.
        &[('m', "bdwalk_height")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let chars: Vec<char> = body.chars().collect();
        let (start, goal, used) = decode_sg_url(&chars, d);
        set_positions(b, start, goal);
        let rest: String = chars[used.min(chars.len())..].iter().collect();
        let n = b.cells.len();
        let cells = &mut b.cells;
        encode::generic_decode_number16(n, &rest, d, |c, val| {
            cells[c].qnum = match val {
                0 => -3,
                1 => -4,
                v if v > 1 => v - 1,
                v => v,
            };
        })
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let (start, goal) = positions(b);
        let mut cm = String::new();
        for (bx, by) in [start, goal] {
            cm += &encode::write_number16((i64::from(bx) + 1) / 2);
            cm += &encode::write_number16((i64::from(by) + 1) / 2);
        }
        cm += &encode::generic_encode_number16(b.cells.len(), |c| {
            let v = b.cells[c].qnum;
            if v == -3 {
                0
            } else if v == -4 {
                1
            } else if v < 0 {
                v
            } else {
                v + 1
            }
        });
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "m", "bdwalk_height");
        decode_sg_fileio(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            if ca == "-" {
                cell.qnum = -2;
            } else if ca == "U" {
                cell.qnum = -3;
            } else if ca == "D" {
                cell.qnum = -4;
            } else if ca != "." {
                cell.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        decode_border_arrow_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "m", "bdwalk_height");
        encode_sg_fileio(b, w);
        w.encode_cell(b, |cell| {
            if cell.qnum >= 0 {
                format!("{} ", cell.qnum)
            } else if cell.qnum == -2 {
                "- ".into()
            } else if cell.qnum == -3 {
                "U ".into()
            } else if cell.qnum == -4 {
                "D ".into()
            } else {
                ". ".into()
            }
        });
        encode_border_arrow_ans(b, w);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !(matches!(c.qnum, -4..=-1) || (1..=99).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "bdwalk cell value outside empty / ? / up / down / clue 1-99",
            );
        }
        if b.borders
            .iter()
            .any(|e| e.line < 0 || !matches!(e.qsub, 0 | 2 | 11..=14))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "bdwalk border value outside line >= 0 or aux mark 0/cross/u/d/l/r",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData` + `adjustBoardData2`: both positions
    /// follow the board operation.
    ///
    /// The two pzprjs hooks record the mapped position before the move and
    /// restore it after; computing the same map on the transformed board is
    /// equivalent. `adjustBoardData2` also resets a start that would coincide
    /// with the goal to the board corners, and pzprjs runs it after every
    /// operation, so the operations run here in `tools/gen_fixtures.js` order.
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        let ops = ops(t);
        if ops.is_empty() {
            return;
        }
        // pzprjs maps with the dimensions the board has *before* each operation.
        let mut cols = if t.swaps_axes() { b.rows() } else { b.cols() } as i32;
        let mut rows = if t.swaps_axes() { b.cols() } else { b.rows() } as i32;
        let (mut start, mut goal) = positions_on(b, cols as usize, rows as usize);
        for op in ops {
            start = map_pos(*op, start, cols, rows);
            goal = map_pos(*op, goal, cols, rows);
            if matches!(op, Op::TurnR | Op::TurnL) {
                std::mem::swap(&mut cols, &mut rows);
            }
            if start == goal {
                // `startpos.init(1, 1); goalpos.init(maxbx - 1, maxby - 1)`.
                start = (1, 1);
                goal = (2 * cols - 1, 2 * rows - 1);
            }
        }
        set_positions(b, start, goal);
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

/// `initExtraObject`: the start/goal of a fresh pzprjs board.
fn default_positions(cols: usize, rows: usize) -> ((i32, i32), (i32, i32)) {
    let rows = rows as i32;
    let cols = cols as i32;
    let start = if cols % 2 == 1 || rows % 2 == 1 {
        (1, 2 * rows - 1)
    } else {
        (1, 1)
    };
    (start, (2 * cols - 1, 1))
}

/// The marker cell's explicit positions, if any.
fn stored_positions(b: &Board) -> Option<((i32, i32), (i32, i32))> {
    b.cells
        .iter()
        .find(|cell| cell.qdir == 1)
        .map(|cell| ((cell.qnum3, cell.qnum4), (cell.qnum2, cell.qsub)))
}

/// The start/goal positions under this board's own dimensions.
fn positions(b: &Board) -> ((i32, i32), (i32, i32)) {
    positions_on(b, b.cols(), b.rows())
}

/// The start/goal positions, explicit marker or `initExtraObject` default for
/// the given dimensions.
fn positions_on(b: &Board, cols: usize, rows: usize) -> ((i32, i32), (i32, i32)) {
    stored_positions(b).unwrap_or_else(|| default_positions(cols, rows))
}

/// Store the positions on cell 0, normalizing the default to "no marker".
fn set_positions(b: &mut Board, start: (i32, i32), goal: (i32, i32)) {
    for cell in &mut b.cells {
        if cell.qdir == 1 {
            cell.qdir = 0;
            cell.qnum2 = -1;
            cell.qnum3 = -1;
            cell.qnum4 = -1;
            cell.qsub = 0;
        }
    }
    if (start, goal) == default_positions(b.cols(), b.rows()) {
        return;
    }
    let cell = &mut b.cells[0];
    cell.qdir = 1;
    cell.qnum2 = goal.0;
    cell.qnum3 = start.0;
    cell.qnum4 = start.1;
    cell.qsub = goal.1;
}

/// `decodeOneNumber16`: one URL number, leaving the rest for the caller.
fn decode_one_number16(chars: &[char], i: usize, d: &mut Diagnostics) -> (i32, usize) {
    match encode::read_number16(chars, i) {
        (Some(v), used) => (v, used),
        (None, used) => {
            d.push(
                DiagnosticCode::UnexpectedChar,
                "incomplete hex escape leaves a start/goal coordinate at -1",
            );
            (-1, used)
        }
    }
}

/// `decodeSG` on the URL body: four `decodeOneNumber16` coordinates, and the
/// number of characters they consumed.
fn decode_sg_url(chars: &[char], d: &mut Diagnostics) -> ((i32, i32), (i32, i32), usize) {
    let mut i = 0usize;
    let mut coord = |i: &mut usize| {
        let (v, used) = decode_one_number16(chars, *i, d);
        *i += used;
        2 * v - 1
    };
    let start = (coord(&mut i), coord(&mut i));
    let goal = (coord(&mut i), coord(&mut i));
    (start, goal, i)
}

/// `decodeSG` on a FileIO line: four base-10 doubled coordinates.
fn decode_sg_fileio(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let line = r.read_line().unwrap_or("").to_string();
    let mut parts = line.split(' ');
    let mut coord = |what: &str| -> i32 {
        let token = parts.next().unwrap_or("");
        match jsnum::parse_int(token, 10) {
            Some(n) => n as i32,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("no base-10 {what} in the start/goal line"),
                );
                0
            }
        }
    };
    let start = (coord("start x"), coord("start y"));
    let goal = (coord("goal x"), coord("goal y"));
    set_positions(b, start, goal);
    Ok(())
}

/// `encodeSG` on a FileIO line.
fn encode_sg_fileio(b: &Board, w: &mut FileWriter) {
    let (start, goal) = positions(b);
    w.write_line(&format!(
        "{} {} {} {}",
        start.0, start.1, goal.0, goal.1
    ));
}

/// `decodeBorderArrowAns`: border line, or `-n` for a line plus cross
/// (`qsub` 2), with an optional trailing `u`/`d`/`l`/`r` aux mark (`qsub`
/// 11–14). Any other lowercase letter is stripped without setting a mark.
fn decode_border_arrow_ans(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_border(b, d, |border, ca| {
        let mut ca = ca.to_string();
        if let Some(lca) = ca.chars().last() {
            if lca.is_ascii_lowercase() {
                border.qsub = match lca {
                    'u' => 11,
                    'd' => 12,
                    'l' => 13,
                    'r' => 14,
                    _ => border.qsub,
                };
                ca.pop();
            }
        }
        if !ca.is_empty() && ca != "0" {
            if ca.starts_with('-') {
                border.line = -fileio::int(&ca)? - 1;
                border.qsub = 2;
            } else {
                border.line = fileio::int(&ca)?;
            }
        }
        Ok(())
    })
}

/// `encodeBorderArrowAns`: `-1-line` for a line plus cross, the line itself
/// otherwise, then the `u`/`d`/`l`/`r` aux mark; `0` when all are empty.
fn encode_border_arrow_ans(b: &Board, w: &mut FileWriter) {
    w.encode_border(b, |border| {
        let mut ca = String::new();
        if border.qsub == 2 {
            ca.push_str(&(-1 - border.line).to_string());
        } else if border.line > 0 {
            ca.push_str(&border.line.to_string());
        }
        if let Some(dir) = match border.qsub {
            11 => Some('u'),
            12 => Some('d'),
            13 => Some('l'),
            14 => Some('r'),
            _ => None,
        } {
            ca.push(dir);
        }
        if ca.is_empty() {
            "0 ".into()
        } else {
            format!("{ca} ")
        }
    });
}

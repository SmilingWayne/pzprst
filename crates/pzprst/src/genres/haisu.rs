//! Haisu (pzprjs `variety/haisu.js`, pid `haisu`).
//!
//! `hasborder` 1. The URL starts with the two marker positions (`decodeSG`:
//! four numbers, doubled coordinates `2n-1`), then the border walls
//! (`decodeBorder`) and the cell numbers (`decodeNumber16`). FileIO stores the
//! marker line, border walls, numbers and a border line / aux-direction layer
//! (`decodeBorderArrowAns` for `filever >= 1`, `decodeBorderLine` before).
//!
//! pzprjs keeps the start and goal in two extra address objects; pzprst has no
//! address objects, so each marker lives on its own cell as `qdir` bit 1
//! (start) / 2 (goal), or `qdir` 3 when both are on the same cell. A position
//! equal to the pzprjs default stays implicit, exactly as an empty URL body does.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Haisu;

struct Haisu;

impl Genre for Haisu {
    fn pid(&self) -> &'static str {
        "haisu"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = decode_sg(b, body, d);
        let rest = encode::decode_border(b, &rest, d);
        encode::decode_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}{}",
            encode_sg(b),
            encode::encode_border(b),
            encode::encode_number16(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_sg_line(r, b, d)?;
        fileio::decode_border_ques(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        if r.filever >= 1 {
            decode_border_arrow_ans(r, b, d)
        } else {
            fileio::decode_border_line(r, b, d)
        }
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.filever = 1;
        encode_sg_line(b, w);
        fileio::encode_border_ques(w, b);
        fileio::encode_cell_qnum(w, b);
        encode_border_arrow_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell@haisu.maxnum` is the size of the cell's room; `minnum` 1.
        let mut maxnum = vec![0i32; b.cells.len()];
        for room in rooms(b) {
            for &c in &room {
                maxnum[c] = room.len() as i32;
            }
        }
        if b.cells.iter().enumerate().any(|(i, c)| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum[i]).contains(&c.qnum))
                || !(0..=3).contains(&c.qdir)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "haisu cell value outside clue 0..room size / ? or start/goal marker",
            );
        }
        let mut starts = 0;
        let mut goals = 0;
        for c in &b.cells {
            if c.qdir & 1 != 0 {
                starts += 1;
            }
            if c.qdir & 2 != 0 {
                goals += 1;
            }
        }
        if starts > 1 || goals > 1 {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "haisu board has more than one start or goal marker",
            );
        }
        if b.borders.iter().any(|e| {
            !(e.ques == 0 || e.ques == 1)
                || !(0..=1).contains(&e.line)
                || !matches!(e.qsub, 0 | 2 | 11..=14)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "haisu border value outside wall 0/1, line 0/1 or aux mark 0/cross/u/d/l/r",
            );
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` records the positions mapped by
        // `getAfterPos`, and `adjustBoardData2` moves the markers there.
        // pzprst's markers travel with their cells, so only two cases need
        // work here: an implicit default position has to be materialized, and
        // `adjustBoardData2`'s "start == goal after an operation" reset must
        // run once per pzprjs operation.
        if t == Transform::Identity {
            return;
        }
        let (cols, rows) = if t.swaps_axes() {
            (b.rows(), b.cols())
        } else {
            (b.cols(), b.rows())
        };
        // A marker has already moved with its cell; an implicit position is
        // still the pre-transform default, so it has to be mapped explicitly.
        let (dstart, dgoal) = defaults(cols, rows);
        let marked_start = marker_pos(b, 1);
        let marked_goal = marker_pos(b, 2);
        let mut start = marked_start.unwrap_or(dstart);
        let mut goal = marked_goal.unwrap_or(dgoal);
        if marked_start.is_none() {
            start = map_positions(start, t, (cols, rows));
        }
        if marked_goal.is_none() {
            goal = map_positions(goal, t, (cols, rows));
        }
        if start == goal {
            // `adjustBoardData2` resets an equal pair to the corners.
            (start, goal) = map_sg(start, start, t, (cols, rows));
        }
        store_sg(b, start, goal);
    }
}

/// pzprjs `Cell@haisu` / `Cell@kaisu` rooms: `AreaRoomGraph` joins cells over
/// borders that are not separators (`ques`/`qans` zero) and drops `ques == 7`.
fn rooms(b: &Board) -> Vec<Vec<usize>> {
    use crate::board::Group;
    let (cols, rows) = (b.cols(), b.rows());
    let n = cols * rows;
    let mut seen = vec![false; n];
    let mut out = Vec::new();
    for start in 0..n {
        if seen[start] || b.cells[start].ques == 7 {
            continue;
        }
        seen[start] = true;
        let mut component = vec![start];
        let mut stack = vec![start];
        while let Some(c) = stack.pop() {
            let (col, row) = (c % cols, c / cols);
            let bx = 2 * col as i32 + 1;
            let by = 2 * row as i32 + 1;
            let mut neighbours = [None; 4];
            if col > 0 {
                neighbours[0] = Some((bx - 1, by));
            }
            if col + 1 < cols {
                neighbours[1] = Some((bx + 1, by));
            }
            if row > 0 {
                neighbours[2] = Some((bx, by - 1));
            }
            if row + 1 < rows {
                neighbours[3] = Some((bx, by + 1));
            }
            for (dir, pos) in neighbours.iter().enumerate() {
                let Some((nbx, nby)) = *pos else { continue };
                let Some(border) = b.obj(Group::Border, nbx, nby) else {
                    continue;
                };
                if border.ques > 0 || border.qans > 0 {
                    continue;
                }
                let next = match dir {
                    0 => c - 1,
                    1 => c + 1,
                    2 => c - cols,
                    _ => c + cols,
                };
                if !seen[next] {
                    seen[next] = true;
                    component.push(next);
                    stack.push(next);
                }
            }
        }
        out.push(component);
    }
    out
}

/// pzprjs `Board.initExtraObject` for haisu: the default start and goal.
fn defaults(cols: usize, rows: usize) -> ((i32, i32), (i32, i32)) {
    let (c, r) = (cols as i32, rows as i32);
    let start = if cols % 2 == 1 || rows % 2 == 1 {
        (1, 2 * r - 1)
    } else {
        (1, 1)
    };
    (start, (2 * c - 1, 1))
}

/// The stored start and goal positions, or the defaults of the current board.
fn sg_positions(b: &Board) -> ((i32, i32), (i32, i32)) {
    let (dstart, dgoal) = defaults(b.cols(), b.rows());
    (
        marker_pos(b, 1).unwrap_or(dstart),
        marker_pos(b, 2).unwrap_or(dgoal),
    )
}

/// The position of the cell carrying a start (bit 1) or goal (bit 2) marker.
fn marker_pos(b: &Board, bit: i32) -> Option<(i32, i32)> {
    b.cells
        .iter()
        .enumerate()
        .find(|(_, cell)| cell.qdir & bit != 0)
        .map(|(i, _)| b.geom.cell_pos(i))
}

/// Store the positions as markers, normalizing the defaults to "no marker".
/// Off-board positions cannot be stored; returns false when one is dropped.
fn store_sg(b: &mut Board, start: (i32, i32), goal: (i32, i32)) -> bool {
    let (dstart, dgoal) = defaults(b.cols(), b.rows());
    for cell in &mut b.cells {
        cell.qdir &= !3;
    }
    let mut ok = true;
    for (pos, default, bit) in [(start, dstart, 1), (goal, dgoal, 2)] {
        if pos == default {
            continue;
        }
        match b.geom.cell_index(pos.0, pos.1) {
            Some(i) => b.cells[i].qdir |= bit,
            None => ok = false,
        }
    }
    ok
}

/// `decodeSG`: four `decodeOneNumber16` values, then the doubled coordinates.
fn decode_sg(b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = body.chars().collect();
    let mut i = 0usize;
    let mut nums = [0i32; 4];
    for num in &mut nums {
        let (val, used) = decode_one_number16(&chars[i.min(chars.len())..], d);
        *num = 2 * val - 1;
        i += used;
    }
    if !store_sg(b, (nums[0], nums[1]), (nums[2], nums[3])) {
        d.push(
            DiagnosticCode::ValueOutOfRange,
            "haisu start/goal position outside the board is dropped",
        );
    }
    chars[i.min(chars.len())..].iter().collect()
}

/// `encodeSG`.
fn encode_sg(b: &Board) -> String {
    let (start, goal) = sg_positions(b);
    let mut cm = String::new();
    for v in [start.0, start.1, goal.0, goal.1] {
        cm += &encode::write_number16((i64::from(v) + 1) / 2);
    }
    cm
}

/// `decodeOneNumber16`: one number, leaving the rest for the caller.
fn decode_one_number16(chars: &[char], d: &mut Diagnostics) -> (i32, usize) {
    match encode::read_number16(chars, 0) {
        (Some(v), used) => (v, used),
        (None, used) => {
            d.push(
                DiagnosticCode::UnexpectedChar,
                "incomplete hex escape leaves a marker position at -1",
            );
            (-1, used)
        }
    }
}

/// FileIO `decodeSG`: the marker line (`bx by goalbx goalby`).
fn decode_sg_line(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let line = r.read_line().unwrap_or("").to_string();
    let mut parts = line.split(' ');
    let mut nums = [0i32; 4];
    for num in &mut nums {
        let token = parts.next().unwrap_or("");
        match jsnum::parse_int(token, 10) {
            Some(n) => *num = n as i32,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("no base-10 marker position in {token:?}"),
                );
                *num = 0;
            }
        }
    }
    if !store_sg(b, (nums[0], nums[1]), (nums[2], nums[3])) {
        d.push(
            DiagnosticCode::ValueOutOfRange,
            "haisu start/goal position outside the board is dropped",
        );
    }
    Ok(())
}

/// FileIO `encodeSG`.
fn encode_sg_line(b: &Board, w: &mut FileWriter) {
    let (start, goal) = sg_positions(b);
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
fn encode_border_arrow_ans(w: &mut FileWriter, b: &Board) {
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

/// The pzprjs board operations a composed [`Transform`] stands for.
#[derive(Clone, Copy)]
enum Op {
    TurnR,
    TurnL,
    FlipX,
    FlipY,
}

/// The operations in `tools/gen_fixtures.js` order.
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

/// pzprjs `BoardExec.getAfterPos` for one operation.
fn map_pos(op: Op, (bx, by): (i32, i32), (cols, rows): (usize, usize)) -> (i32, i32) {
    let (c, r) = (cols as i32, rows as i32);
    match op {
        Op::TurnR => (2 * r - by, bx),
        Op::TurnL => (by, 2 * c - bx),
        Op::FlipX => (2 * c - bx, by),
        Op::FlipY => (bx, 2 * r - by),
    }
}

/// One position through every operation of the composed transform.
fn map_positions(pos: (i32, i32), t: Transform, dims: (usize, usize)) -> (i32, i32) {
    let mut pos = pos;
    let mut dims = dims;
    for op in ops(t) {
        pos = map_pos(*op, pos, dims);
        if matches!(op, Op::TurnR | Op::TurnL) {
            dims = (dims.1, dims.0);
        }
    }
    pos
}

/// The positions after every operation, including `adjustBoardData2`'s reset
/// to the corners when both markers are on the same cell.
fn map_sg(
    start: (i32, i32),
    goal: (i32, i32),
    t: Transform,
    dims: (usize, usize),
) -> ((i32, i32), (i32, i32)) {
    let (mut s, mut g) = (start, goal);
    let mut dims = dims;
    for (i, op) in ops(t).iter().enumerate() {
        s = map_pos(*op, s, dims);
        g = map_pos(*op, g, dims);
        if matches!(op, Op::TurnR | Op::TurnL) {
            dims = (dims.1, dims.0);
        }
        if i == 0 && start == goal {
            s = (1, 1);
            g = (2 * dims.0 as i32 - 1, 2 * dims.1 as i32 - 1);
        }
    }
    (s, g)
}

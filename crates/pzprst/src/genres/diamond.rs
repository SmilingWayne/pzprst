//! Diamond (pzprjs `variety/diamond.js`, pid `diamond`).
//!
//! The URL stores only the cell numbers (`decode4Cell`: 0–4, `.` for `?`).
//! FileIO adds the answer shading/`?` layer (`decodeCellQnumAns`), the diamond
//! layer on the inner crosses (`decodeCross` with `isInset`, one `0`/`1`/`2`
//! per cross) and — only when some dot holds a value of 2 or more — the dot
//! grid `decodeDotFile`. pzprjs's `Dot.getDot`/`Dot.setDot` are overridden for
//! this pid: a dot is the piece's `qsub`, and setting one is refused while
//! `Dot.isHidden` holds (a numbered cell, a cell or border touching a diamond,
//! or a diamond itself).

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Diamond;

struct Diamond;

impl Genre for Diamond {
    fn pid(&self) -> &'static str {
        "diamond"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        1
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
        fileio::decode_cell_qnum_ans(r, b, d)?;
        r.decode_cross(b, d, true, |cross, ca| {
            // pzprjs `cross.setDiamond(+ca, true)`: `1` marks a diamond, `2` an
            // auxiliary mark, and every other token (a `.` becomes `NaN`)
            // clears both. `isInset` keeps `setDiamond`'s frame guard quiet.
            let val = fileio::int(ca).ok();
            cross.qans = i32::from(val == Some(1));
            cross.qsub = i32::from(val == Some(2));
            Ok(())
        })?;
        decode_dot_file(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum_ans(w, b);
        w.encode_cross(b, true, |cross| {
            // pzprjs `cross.getDiamond()`.
            let ca = if cross.qans != 0 {
                1
            } else if cross.qsub == 1 {
                2
            } else {
                0
            };
            format!("{ca} ")
        });
        if any_dot_with_value(b) {
            encode_dot_file(w, b);
        }
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let legal_dot = |piece: &crate::Obj| {
            (piece.qsub == -2 || (0..=9).contains(&piece.qsub)) && (piece.ques == 0 || piece.ques == 7)
        };
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=4).contains(&c.qnum))
                || !(c.qans == 0 || c.qans == 1)
                || !legal_dot(c)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "diamond cell value outside clue 0-4/?, shade 0/1, or dot -2/0-9 with X 0/7",
            );
        }
        if b.crosses.iter().any(|c| {
            !(c.qans == 0 || c.qans == 1) || !(c.qsub == 0 || c.qsub == 1) || !legal_dot(c)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "diamond cross value outside diamond 0/1, mark 0/1, or dot -2/0-9",
            );
        }
        if b.borders.iter().any(|e| !legal_dot(e)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "diamond border dot outside -2/0-9 with X 0/7",
            );
        }
        if b.cells
            .iter()
            .chain(&b.crosses)
            .chain(&b.borders)
            .any(|piece| piece.ques == 7)
        {
            d.push(
                DiagnosticCode::LossyUrl,
                "diamond X dots (ques 7) are dropped from the URL",
            );
        }
    }
}

/// pzprjs `Dot.isHidden` for diamond: a numbered cell, a cell or border that
/// touches a diamond (its corner crosses / its two side crosses), or a piece
/// that is itself a diamond.
fn is_hidden(b: &Board, bx: i32, by: i32, group: Group) -> bool {
    let diamond = |x: i32, y: i32| {
        b.obj(Group::Cross, x, y)
            .is_some_and(|cross| cross.qans == 1)
    };
    match group {
        Group::Cell => {
            let cell = b.obj(Group::Cell, bx, by);
            // `Cell.allowUnshade` (a numbered cell never takes a dot) plus the
            // four corner crosses of `Cell.dotCrosses`.
            cell.is_some_and(|cell| cell.qnum != -1 || cell.qans == 1)
                || diamond(bx - 1, by - 1)
                || diamond(bx + 1, by - 1)
                || diamond(bx - 1, by + 1)
                || diamond(bx + 1, by + 1)
        }
        Group::Border => {
            let (c1, c2) = if bx & 1 == 0 {
                ((bx, by - 1), (bx, by + 1))
            } else {
                ((bx - 1, by), (bx + 1, by))
            };
            b.obj(Group::Border, bx, by)
                .is_some_and(|border| border.qans == 1)
                || diamond(c1.0, c1.1)
                || diamond(c2.0, c2.1)
        }
        Group::Cross => b
            .obj(Group::Cross, bx, by)
            .is_some_and(|cross| cross.qans == 1),
        // diamond has no excells; `Dot` only ever wraps cells, borders and crosses.
        Group::ExCell => false,
    }
}

/// `Dot.setDot` for diamond (the piece's `qsub`), refused while `isHidden`.
fn set_dot(b: &mut Board, bx: i32, by: i32, group: Group, val: i32) {
    if val != 0 && is_hidden(b, bx, by, group) {
        return;
    }
    if let Some(dot) = b.obj_mut(group, bx, by) {
        dot.qsub = val;
    }
}

/// `decodeDotFile` with diamond's `Dot.setDot`; `2*rows-1` lines of
/// `2*cols-1` dot characters.
fn decode_dot_file(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let (cols, rows) = (b.cols(), b.rows());
    let dotsmax = (2 * cols - 1) * (2 * rows - 1);
    let mut data = String::new();
    for _ in 0..2 * rows - 1 {
        match r.read_line() {
            Some(line) => data.push_str(line),
            None => d.push(
                DiagnosticCode::TruncatedBody,
                "FileIO ends before the last dot line",
            ),
        }
    }
    let chars: Vec<char> = data.chars().collect();
    if chars.len() > dotsmax {
        d.push(
            DiagnosticCode::TrailingData,
            "dot data has more characters than dots",
        );
    }
    for (s, &ca) in chars.iter().take(dotsmax).enumerate() {
        let (bx, by) = encode::dot_point(b, s);
        let group = encode::dot_group(bx, by);
        if let Some(num) = ca.to_digit(10) {
            if (1..=9).contains(&num) {
                set_dot(b, bx, by, group, num as i32);
            }
        } else if ca == '-' {
            set_dot(b, bx, by, group, -2);
        } else if ca == 'X' {
            if let Some(dot) = b.obj_mut(group, bx, by) {
                dot.ques = 7;
            }
        }
    }
    Ok(())
}

/// Whether any dot on the grid has `getDot() >= 2`, i.e. whether FileIO writes
/// the dot layer at all.
fn any_dot_with_value(b: &Board) -> bool {
    let (cols, rows) = (b.cols(), b.rows());
    (1..=2 * rows - 1).any(|by| {
        (1..=2 * cols - 1)
            .any(|bx| b.dot(bx as i32, by as i32).is_some_and(|dot| dot.qsub >= 2))
    })
}

/// `encodeDotFile` with diamond's `Dot.getDot` (the piece's `qsub`).
fn encode_dot_file(w: &mut FileWriter, b: &Board) {
    let (cols, rows) = (b.cols(), b.rows());
    for by in 1..=2 * rows - 1 {
        let mut data = String::new();
        for bx in 1..=2 * cols - 1 {
            let ca = match b.dot(bx as i32, by as i32) {
                Some(dot) if (1..=9).contains(&dot.qsub) => {
                    char::from_digit(dot.qsub as u32, 10).unwrap()
                }
                Some(dot) if dot.ques == 7 => 'X',
                Some(dot) if dot.qsub == -2 => '-',
                _ => '.',
            };
            data.push(ca);
        }
        w.write_line(&data);
    }
}

//! Pencils (pzprjs `variety/pencils.js`, pid `pencils`).
//!
//! A cell is either a clue (`qnum` 0–max(cols, rows)-1, `?` is -2) or an
//! arrow pointing at a pencil tip (`qdir` 1 up, 2 down, 3 left, 4 right); the
//! border behind the arrow carries `ques` 1 (the pencil outline, derived by
//! `Cell.setArrow` and not stored in either format). FileIO also stores the
//! border answers (`qans`, `qsub`) and the loop (`line`), plus `qans`/`qsub`
//! marks on cells. Rotations/reflections rotate `qdir` and `qans`
//! (`BoardExec.adjustBoardData`).

use super::Genre;
use crate::board::{Board, Group};
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Pencils;

struct Pencils;

impl Genre for Pencils {
    fn pid(&self) -> &'static str {
        "pencils"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    /// pzprjs `Board.hascross` (the inherited default 2); the URL and FileIO
    /// store no cross data.
    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_pencils(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_pencils(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        let mut index = 0usize;
        let mut arrows = Vec::new();
        r.decode_cell(b, d, |cell, ca| {
            let c = index;
            index += 1;
            if let Some(rest) = ca.strip_prefix('o') {
                // pzprjs stores `qdir` 5 as "this is a number cell"; pzprst
                // models it as a plain clue so FileIO round-trips.
                if !rest.is_empty() {
                    cell.qnum = fileio::int(rest)?;
                }
            } else if ca != "." {
                match jsnum::integer(ca) {
                    Some(dir) if (1..=4).contains(&dir) => {
                        cell.qnum = -1;
                        cell.qdir = dir;
                        arrows.push((c, dir));
                    }
                    Some(_) => {}
                    None => return Err(format!("expected an arrow direction, found {ca:?}")),
                }
            }
            Ok(())
        })?;
        for (c, dir) in arrows {
            pencil_outline(b, c, dir);
        }
        fileio::decode_border_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            let rest = if let Some(rest) = ca.strip_prefix('+') {
                cell.qsub = 1;
                rest
            } else if let Some(rest) = ca.strip_prefix('-') {
                cell.qsub = 2;
                rest
            } else {
                ca
            };
            if rest != "." {
                if let Some(qans) = jsnum::integer(rest) {
                    if qans <= 4 {
                        cell.qans = qans;
                    }
                }
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.qnum != -1 {
                format!("o{} ", cell.qnum)
            } else if cell.qdir != 0 {
                format!("{} ", cell.qdir)
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_ans(w, b);
        fileio::encode_border_line(w, b);
        w.encode_cell(b, |cell| {
            let mut ca = String::new();
            if cell.qsub == 1 {
                ca.push('+');
            } else if cell.qsub == 2 {
                ca.push('-');
            }
            if cell.qans != 0 {
                ca.push_str(&cell.qans.to_string());
            }
            if ca.is_empty() {
                ca.push('.');
            }
            format!("{ca} ")
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cell.maxnum` is max(cols, rows) - 1, `minnum` 1.
        let maxnum = b.cols().max(b.rows()) as i32 - 1;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
                || !(0..=4).contains(&c.qdir)
                || !(0..=4).contains(&c.qans)
                || !(0..=2).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "pencils cell value outside clue 0..maxnum / ? / arrow 1-4 or answer mark",
            );
        }
        if b.cells.iter().any(|c| {
            (c.qdir != 0 && c.qnum != -1) || c.qnum < -2 || c.qnum >= 256
        }) {
            d.push(
                DiagnosticCode::LossyUrl,
                "pencils URL stores a cell as either a number below 256 or an arrow, so the other half is dropped",
            );
        }
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.ques)
                || !(0..=1).contains(&e.qans)
                || !(0..=1).contains(&e.line)
                || !(0..=2).contains(&e.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "pencils border value outside wall 0/1, answer 0/1, line 0/1 or mark 0-2",
            );
        }
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` (pencils.js): rotate `qdir` and
        // `qans` with the same `getTranslateDir` tables, before the move.
        for op in turn_ops(t) {
            for cell in &mut b.cells {
                if let Some(dir) = translate_dir(*op, cell.qdir) {
                    cell.qdir = dir;
                }
                if let Some(ans) = translate_dir(*op, cell.qans) {
                    cell.qans = ans;
                }
            }
        }
    }
}

/// `decodePzpr`: one character per cell — one hex digit or `-` plus two hex
/// digits for a clue, `.` for `?`, `g`–`j` for an arrow direction 1–4, and
/// `k`–`z` for blank runs of `digit - 20` cells.
fn decode_pencils(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        if c >= n {
            break;
        }
        if ca.is_ascii_digit() || ('a'..='f').contains(&ca) {
            b.cells[c].qnum = jsnum::digit(ca, 16).unwrap() as i32;
        } else if ca == '-' {
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qn) => b.cells[c].qnum = qn as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal number after '-'",
                ),
            }
            i += 2;
        } else if ca == '.' {
            b.cells[c].qnum = -2;
        } else if ('g'..='j').contains(&ca) {
            let dir = jsnum::digit(ca, 20).unwrap() as i32 - 15;
            set_arrow_question(b, c, dir);
        } else if ('k'..='z').contains(&ca) {
            c += jsnum::digit(ca, 36).unwrap() as usize - 20;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} is skipped"),
            );
        }
        c += 1;
        if c >= n {
            break;
        }
        i += 1;
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    // pzprjs consumed the character at `i` before breaking; the rest starts after it.
    chars[(i + 1).min(chars.len())..].iter().collect()
}

/// `encodePzpr`: clues as hex, arrows as `g`–`j`, blanks as base-36 runs.
fn encode_pencils(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let (dir, qn) = (cell.qdir, cell.qnum);
        let pstr = if qn != -1 {
            if (0..16).contains(&qn) {
                Some(jsnum::to_radix(qn as u32, 16))
            } else if (16..256).contains(&qn) {
                Some(format!("-{}", jsnum::to_radix(qn as u32, 16)))
            } else {
                Some(".".to_string())
            }
        } else if dir != 0 {
            Some(jsnum::to_radix((dir + 15) as u32, 20))
        } else {
            None
        };
        match pstr {
            Some(pstr) => {
                if count == 0 {
                    cm.push_str(&pstr);
                } else {
                    cm.push_str(&jsnum::to_radix(count + 19, 36));
                    cm.push_str(&pstr);
                    count = 0;
                }
            }
            None => {
                count += 1;
                if count == 16 {
                    cm.push_str(&jsnum::to_radix(count + 19, 36));
                    count = 0;
                }
            }
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(count + 19, 36));
    }
    cm
}

/// `Cell.setArrow(dir, true)` on a cell that has no arrow and no answer mark,
/// which is all the URL decoder ever sees: clear the clue, set the direction
/// and mark the border behind the arrow as the pencil outline.
fn set_arrow_question(b: &mut Board, c: usize, dir: i32) {
    if !(1..=4).contains(&dir) {
        return;
    }
    b.cells[c].qnum = -1;
    b.cells[c].qdir = dir;
    pencil_outline(b, c, dir);
}

/// The border half of `Cell.setArrow`: the pencil outline sits *behind* the
/// arrow (up → bottom border, down → top, left → right, right → left).
fn pencil_outline(b: &mut Board, c: usize, dir: i32) {
    let (bx, by) = b.geom.cell_pos(c);
    let (ex, ey) = match dir {
        1 => (bx, by + 1),
        2 => (bx, by - 1),
        3 => (bx + 1, by),
        _ => (bx - 1, by),
    };
    if let Some(border) = b.obj_mut(Group::Border, ex, ey) {
        border.ques = 1;
    }
}

/// pzprjs `BoardExec.getTranslateDir`, as one direction value mapping.
fn translate_dir(op: Turn, dir: i32) -> Option<i32> {
    match (op, dir) {
        (Turn::Right, 1) => Some(4),
        (Turn::Right, 2) => Some(3),
        (Turn::Right, 3) => Some(1),
        (Turn::Right, 4) => Some(2),
        (Turn::Left, 1) => Some(3),
        (Turn::Left, 2) => Some(4),
        (Turn::Left, 3) => Some(2),
        (Turn::Left, 4) => Some(1),
        (Turn::FlipX, 3) => Some(4),
        (Turn::FlipX, 4) => Some(3),
        (Turn::FlipY, 1) => Some(2),
        (Turn::FlipY, 2) => Some(1),
        _ => None,
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Right,
    Left,
    FlipX,
    FlipY,
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

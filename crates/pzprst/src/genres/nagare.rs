//! Nagareru Loop (pzprjs `variety/nagare.js`, pid `nagare`).
//!
//! `ques` 1 marks a wall cell; `qdir` is a wind direction (1 up, 2 down,
//! 3 left, 4 right). The URL is `decodeNagare`: decimal `qu * 5 + dir`
//! (1–9), `a`–`z` runs of empty cells. FileIO stores `u`/`d`/`l`/`r` for
//! arrows, `N`/`U`/`D`/`L`/`R` for walls (with an optional arrow) and then the
//! border line/aux-direction layer (`decodeBorderArrowAns`). Rotations and
//! reflections turn the cell directions and the border aux marks
//! (`adjustBoardData` in pzprjs `BoardExec`).

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Nagare;

struct Nagare;

impl Genre for Nagare {
    fn pid(&self) -> &'static str {
        "nagare"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_nagare(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_nagare(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca != "." {
                let val = match ca {
                    "u" => 1,
                    "d" => 2,
                    "l" => 3,
                    "r" => 4,
                    "N" => 5,
                    "U" => 6,
                    "D" => 7,
                    "L" => 8,
                    "R" => 9,
                    _ => return Err(format!("expected a nagare cell token, found {ca:?}")),
                };
                cell.ques = val / 5;
                cell.qdir = val % 5;
            }
            Ok(())
        })?;
        decode_border_arrow_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 1 || (1..=4).contains(&cell.qdir) {
                let val = cell.ques * 5 + cell.qdir;
                ["u ", "d ", "l ", "r ", "N ", "U ", "D ", "L ", "R "]
                    .get((val - 1) as usize)
                    .copied()
                    .unwrap_or(". ")
                    .into()
            } else {
                ". ".into()
            }
        });
        encode_border_arrow_ans(w, b);
        Ok(())
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData`: `adjustCellArrow` turns the cell
        // directions, then the border aux marks (`qsub` 11–14) turn the same
        // way (`trans[border.qsub - 10]`).
        for op in turn_ops(t) {
            for cell in &mut b.cells {
                if let Some(dir) = translate_dir(*op, cell.qdir) {
                    cell.qdir = dir;
                }
            }
            for border in &mut b.borders {
                if let Some(dir) = translate_dir(*op, border.qsub - 10) {
                    border.qsub = dir + 10;
                }
            }
        }
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !(c.ques == 0 || c.ques == 1) || !(0..=4).contains(&c.qdir))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "nagare cell value outside wall 0/1 or direction 0-4",
            );
        }
        if b.borders.iter().any(|e| {
            e.line < 0 || !matches!(e.qsub, 0 | 2 | 11 | 12 | 13 | 14)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "nagare border value outside line >= 0 or aux mark 0/cross/u/d/l/r",
            );
        }
    }
}

/// `decodeNagare`: `1`–`9` are `ques * 5 + qdir`, `a`–`z` run over
/// `parseInt(ca, 36) - 10` empty cells; every other character consumes one
/// empty cell.
fn decode_nagare(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    let mut a = 0usize;
    while i < chars.len() {
        let ca = chars[i];
        if ('1'..='9').contains(&ca) {
            let val = ca.to_digit(10).unwrap() as i32;
            b.cells[c].ques = val / 5;
            b.cells[c].qdir = val % 5;
        } else if ca.is_ascii_lowercase() {
            c += jsnum::digit(ca, 36).unwrap() as usize - 10;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} is skipped"),
            );
        }
        c += 1;
        a = i + 1;
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
    chars[a.min(chars.len())..].iter().collect()
}

/// `encodeNagare`: `ques * 5 + qdir`, `9 + count` in base 36 for empty-cell
/// runs (flushed after 26 like pzprjs).
fn encode_nagare(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let pstr = if cell.ques == 1 || (1..=4).contains(&cell.qdir) {
            (cell.ques * 5 + cell.qdir).to_string()
        } else {
            count += 1;
            if count == 26 {
                cm.push_str(&jsnum::to_radix(9 + count, 36));
                count = 0;
            }
            continue;
        };
        if count == 0 {
            cm.push_str(&pstr);
        } else {
            cm.push_str(&jsnum::to_radix(9 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(9 + count, 36));
    }
    cm
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

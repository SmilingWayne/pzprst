//! Minarism (pzprjs `variety/minarism.js`, pid `minarism`).
//!
//! Clues live on **borders**: a circle number (`qnum`, `?` is -2) or an
//! inequality arrow (`qdir` UP/DN/LT/RT, the direction depends on whether the
//! border is horizontal or vertical). The URL is `decodeMinarism`
//! (`0`–`f` numbers, `-` plus two hex digits for 16–255, `.` for `?`, `g`/`h`
//! for the two inequality directions, `i`–`z` blank runs). FileIO stores the
//! border marks and then the answer number/cmp/sub layer. Rotations and
//! reflections rotate the border arrows (`adjustBorderArrow` in
//! `BoardExec.adjustBoardData`).

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Minarism;

struct Minarism;

impl Genre for Minarism {
    fn pid(&self) -> &'static str {
        "minarism"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_minarism(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_minarism(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        // The `a`/`b` inequality token means different directions on vertical
        // and horizontal borders, and `decode_border`'s closure does not see
        // the position: feed it the orientations in FileIO order (which is
        // `b.borders` order for `hasborder` 1).
        let mut orientations = (0..b.borders.len())
            .map(|i| b.geom.border_pos(i).1 & 1 == 0)
            .collect::<Vec<bool>>()
            .into_iter();
        r.decode_border(b, d, |border, ca| {
            let is_horz = orientations.next().unwrap_or(false);
            if ca == "a" {
                border.qdir = if is_horz { UP } else { LT };
            } else if ca == "b" {
                border.qdir = if is_horz { DN } else { RT };
            } else if ca == "." {
                border.qnum = -2;
            } else if ca != "0" {
                border.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        // The token depends on the border's orientation, which the closure
        // cannot see; encode border by border in FileIO order.
        w.encode_border(b, |border| {
            let dir = border.qdir;
            if dir == UP || dir == LT {
                "a ".into()
            } else if dir == DN || dir == RT {
                "b ".into()
            } else if border.qnum == -2 {
                ". ".into()
            } else if border.qnum != -1 {
                format!("{} ", border.qnum)
            } else {
                "0 ".into()
            }
        });
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` → `adjustBorderArrow`:
        // `trans[border.qdir]` rewrites the arrow directions of every inner
        // border, once per board operation, before the coordinates move.
        for op in turn_ops(t) {
            for border in &mut b.borders {
                if let Some(dir) = translate_dir(*op, border.qdir) {
                    border.qdir = dir;
                }
            }
        }
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cell.maxnum` is `max(cols, rows)`; border clues are read as
        // 0–255 by the URL decoder, and a `?` is -2.
        let maxnum = b.cols().max(b.rows()) as i32;
        if b.borders
            .iter()
            .any(|e| !(e.qnum == -2 || e.qnum == -1 || (0..=maxnum).contains(&e.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("minarism border clue outside 0-{maxnum}/?"),
            );
        }
        if b.borders.iter().any(|e| !(0..=4).contains(&e.qdir)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "minarism border direction outside 0-4",
            );
        }
        if b.borders.iter().any(|e| e.qnum >= 256) {
            d.push(
                DiagnosticCode::LossyUrl,
                "minarism numbers of 256 or more are dropped from the URL",
            );
        }
        if b.cells
            .iter()
            .any(|c| !(0..=4).contains(&c.qsub) || !(0..=1).contains(&c.qcmp))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "minarism cell answer mark outside sub 0-4 / cmp 0-1",
            );
        }
    }
}

const UP: i32 = 1;
const DN: i32 = 2;
const LT: i32 = 3;
const RT: i32 = 4;

/// `decodeMinarism`: one mark per border id, with `i`–`z` skipping
/// `parseInt(ca, 36) - 18` borders and `-` reading two hex digits.
fn decode_minarism(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let bdmax = b.borders.len();
    let (mut id, mut a) = (0usize, 0usize);
    let mut i = 0usize;
    while i < chars.len() {
        if id >= bdmax {
            break;
        }
        let ca = chars[i];
        let is_horz = b.geom.border_pos(id).1 & 1 == 0;
        let mut tmp = 0;
        if ca.is_ascii_digit() || ('a'..='f').contains(&ca) {
            b.borders[id].qnum = ca.to_digit(16).unwrap() as i32;
        } else if ca == '-' {
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qn) => b.borders[id].qnum = qn as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal digits after '-' leave the border empty",
                ),
            }
            i += 2;
        } else if ca == '.' {
            b.borders[id].qnum = -2;
        } else if ca == 'g' {
            tmp = 1;
        } else if ca == 'h' {
            tmp = 2;
        } else if ('i'..='z').contains(&ca) {
            id += jsnum::digit(ca, 36).unwrap() as usize - 18;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} is skipped"),
            );
        }
        if tmp == 1 {
            b.borders[id].qdir = if is_horz { UP } else { LT };
        } else if tmp == 2 {
            b.borders[id].qdir = if is_horz { DN } else { RT };
        }
        id += 1;
        i += 1;
        if id >= bdmax {
            a = i;
            break;
        }
    }
    chars[a.min(chars.len())..].iter().collect()
}

/// `encodeMinarism`: the inverse of [`decode_minarism`], with `17 + count` in
/// base 36 for blank runs, flushed after 18 blanks like pzprjs.
fn encode_minarism(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for border in &b.borders {
        let dir = border.qdir;
        let qnum = border.qnum;
        let pstr = if dir == UP || dir == LT {
            Some("g".to_string())
        } else if dir == DN || dir == RT {
            Some("h".to_string())
        } else if qnum == -2 {
            Some(".".to_string())
        } else if (0..16).contains(&qnum) {
            Some(jsnum::to_radix(qnum as u32, 16))
        } else if (16..256).contains(&qnum) {
            Some(format!("-{}", jsnum::to_radix(qnum as u32, 16)))
        } else {
            None
        };
        match pstr {
            Some(pstr) => {
                if count == 0 {
                    cm.push_str(&pstr);
                } else {
                    cm.push_str(&jsnum::to_radix(17 + count, 36));
                    cm.push_str(&pstr);
                    count = 0;
                }
            }
            None => {
                count += 1;
                if count == 18 {
                    cm.push_str(&jsnum::to_radix(17 + count, 36));
                    count = 0;
                }
            }
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(17 + count, 36));
    }
    cm
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

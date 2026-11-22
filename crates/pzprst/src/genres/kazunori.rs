//! Kazunori (pzprjs `variety/kazunori.js`, pid `kazunori`).
//!
//! `hasborder` 1. The URL is the border wall layer (`decodeBorder`) followed by
//! one number per inner border (`decodeBorderNumber`, `.` is `?`). FileIO stores
//! the border number/wall layer (`|` wall, `!` a number without a wall, `-` a
//! wall plus `?`) and the cell answer numbers (`decodeCellAnumsub`). Rotations
//! turn border `qdir` (`BoardExec.adjustBorderArrow`), which no codec sets.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Kazunori;

struct Kazunori;

impl Genre for Kazunori {
    fn pid(&self) -> &'static str {
        "kazunori"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        decode_border_number(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_border(b),
            encode_border_number(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_border(b, d, |border, ca| {
            if ca == "|" {
                border.ques = 1;
            } else if ca == "-" {
                border.ques = 1;
                border.qnum = -2;
            } else if ca == "!" {
                border.qnum = -2;
            } else if ca != "." {
                let rest = match ca.strip_prefix('!') {
                    Some(rest) => rest,
                    None => {
                        border.ques = 1;
                        ca
                    }
                };
                // pzprjs `border.qnum = +ca`; garbage would store NaN.
                if let Some(n) = jsnum::integer(rest) {
                    border.qnum = n;
                }
            }
            Ok(())
        })?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_border(b, |border| {
            if border.qnum >= 0 {
                let prefix = if border.ques == 1 { "" } else { "!" };
                format!("{prefix}{} ", border.qnum)
            } else if border.qnum == -2 {
                if border.ques == 1 {
                    "- ".into()
                } else {
                    "! ".into()
                }
            } else if border.ques == 1 {
                "| ".into()
            } else {
                ". ".into()
            }
        });
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum` is `(room size + 1) >> 1`; `Border.maxnum` is the sum
        // of its two cells' maxnums.
        let mut maxnum = vec![0i32; b.cells.len()];
        for room in rooms(b) {
            let n = ((room.len() as i32) + 1) >> 1;
            for &c in &room {
                maxnum[c] = n;
            }
        }
        if b.cells.iter().enumerate().any(|(i, c)| {
            c.qnum != -1
                || c.ques != 0
                || !(c.anum == -1 || (0..=maxnum[i]).contains(&c.anum))
                || !(c.qcmp == 0 || c.qcmp == 1)
                || !(0..=4).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kazunori cell value outside answer number / done mark 0-1 / background mark 0-4",
            );
        }
        let border_ok = b.borders.iter().enumerate().all(|(i, e)| {
            let (bx, by) = b.geom.border_pos(i);
            let (c1, c2) = border_sides(b, bx, by);
            let border_max = c1.map_or(0, |c| maxnum[c]) + c2.map_or(0, |c| maxnum[c]);
            (e.ques == 0 || e.ques == 1)
                && (e.qnum == -1 || e.qnum == -2 || (0..=border_max).contains(&e.qnum))
                && (0..=4).contains(&e.qdir)
        });
        if !border_ok {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kazunori border value outside wall 0/1, number 0..adjacent room maxima or ?",
            );
        }
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` → `adjustBorderArrow`, which runs
        // before the board moves and turns border `qdir`.
        for op in turn_ops(t) {
            for border in &mut b.borders {
                if let Some(dir) = translate_dir(*op, border.qdir) {
                    border.qdir = dir;
                }
            }
        }
    }
}

/// The two cells on either side of the border at `(bx, by)`.
fn border_sides(b: &Board, bx: i32, by: i32) -> (Option<usize>, Option<usize>) {
    if bx & 1 == 0 && by & 1 == 1 {
        (
            b.geom.cell_index(bx - 1, by),
            b.geom.cell_index(bx + 1, by),
        )
    } else if bx & 1 == 1 && by & 1 == 0 {
        (
            b.geom.cell_index(bx, by - 1),
            b.geom.cell_index(bx, by + 1),
        )
    } else {
        (None, None)
    }
}

/// pzprjs `AreaRoomGraph` components (`border.ques`/`border.qans` separate,
/// `ques == 7` cells are not nodes).
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

/// `decodeBorderNumber`: one value per inner border, `.` is `?`, `-` plus two
/// hex digits is 16–255 and `g`–`z` skip borders.
fn decode_border_number(b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = body.chars().collect();
    let n = b.borders.len();
    let (mut id, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let mut used = 1usize;
        if matches!(ca, '0'..='9' | 'a'..='f') || ca == '-' || ca == '.' {
            if id >= n {
                // pzprjs reads `bd.border[id].qnum` and throws here.
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    "border number after the last border is skipped",
                );
                return chars[i..].iter().collect();
            }
            if ca == '-' {
                used = 3;
                match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                    Some(v) => b.borders[id].qnum = v as i32,
                    None => d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("border number at character {i} is not hexadecimal"),
                    ),
                }
            } else if ca == '.' {
                b.borders[id].qnum = -2;
            } else {
                b.borders[id].qnum = ca.to_digit(16).unwrap() as i32;
            }
        } else if ('g'..='z').contains(&ca) {
            id += jsnum::digit(ca, 36).unwrap() as usize - 16;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} advances one border"),
            );
        }
        id += 1;
        i += used;
        if id >= n {
            break;
        }
    }
    if id < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last border",
        );
    }
    chars[i.min(chars.len())..].iter().collect()
}

/// `encodeBorderNumber`.
fn encode_border_number(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for border in &b.borders {
        let qnum = border.qnum;
        let pstr = if qnum == -2 {
            Some(".".to_string())
        } else if (0..16).contains(&qnum) {
            Some(jsnum::to_radix(qnum as u32, 16))
        } else if (16..256).contains(&qnum) {
            Some(format!("-{}", jsnum::to_radix(qnum as u32, 16)))
        } else {
            None
        };
        if let Some(pstr) = pstr {
            if count == 0 {
                cm.push_str(&pstr);
            } else {
                cm.push_str(&jsnum::to_radix(15 + count, 36));
                cm.push_str(&pstr);
                count = 0;
            }
        } else {
            count += 1;
            if count == 20 {
                cm.push_str(&jsnum::to_radix(15 + count, 36));
                count = 0;
            }
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(15 + count, 36));
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

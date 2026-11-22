//! Heyajirimisaki (pzprjs `variety/yajilin.js`, pid `heyajirimisaki`).
//!
//! `hasborder` 1. The URL is the border wall layer (`decodeBorder`), the corner
//! numbers in `qnum2` (`decodeCornerNumber`) and the arrow direction + number in
//! `qdir`/`qnum` (`decodeArrowNumber16`). FileIO stores border walls, one
//! `dir,num` token per cell (`5,num` for a corner number, `.` empty), the
//! answer shading (`decodeCellAns`) and the border lines. Rotations turn the
//! cell arrows (`BoardExec@yajilin,lixloop,heyajirimisaki` → `adjustNumberArrow`).

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Heyajirimisaki;

struct Heyajirimisaki;

impl Genre for Heyajirimisaki {
    fn pid(&self) -> &'static str {
        "heyajirimisaki"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        let rest = decode_corner_number(b, &rest, d);
        decode_arrow_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}{}",
            encode::encode_border(b),
            encode_corner_number(b),
            encode_arrow_number16(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_border_ques(r, b, d)?;
        decode_heyajirimisaki(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_border_ques(w, b);
        encode_heyajirimisaki(w, b);
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell@heyajirimisaki#2`: a positive `qnum` allows `max(cols, rows)`;
        // a corner number allows `max(room size, cols, rows)`.
        let maxnum = b.cols().max(b.rows()) as i32;
        let mut roomnum = vec![maxnum; b.cells.len()];
        for room in rooms(b) {
            for &c in &room {
                roomnum[c] = roomnum[c].max(room.len() as i32);
            }
        }
        if b.cells.iter().enumerate().any(|(i, c)| {
            !(c.qnum == -1
                || c.qnum == -2
                || c.qnum == -3
                || (0..=maxnum).contains(&c.qnum))
                || !(c.qnum2 == -1 || c.qnum2 == -2 || (0..=roomnum[i]).contains(&c.qnum2))
                || !(0..=4).contains(&c.qdir)
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "heyajirimisaki cell value outside number/arrow, corner number, shade or mark range",
            );
        }
        if b.borders.iter().any(|e| {
            !(e.ques == 0 || e.ques == 1)
                || !(0..=1).contains(&e.line)
                || !(e.qsub == 0 || e.qsub == 2)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "heyajirimisaki border value outside wall 0/1, line 0/1 or cross -1",
            );
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` → `adjustNumberArrow`
        // → `adjustCellArrowField(key, d, "qdir")`; the map only rewrites the
        // value, so applying it to the transformed board is equivalent.
        for op in turn_ops(t) {
            for cell in &mut b.cells {
                if let Some(dir) = translate_dir(*op, cell.qdir) {
                    cell.qdir = dir;
                }
            }
        }
    }
}

/// pzprjs `AreaRoomGraph` components (`border.ques`/`border.qans` separate,
/// `ques == 7` cells are not nodes).
fn rooms(b: &Board) -> Vec<Vec<usize>> {
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

/// `decodeCornerNumber`: the `qnum2` layer.
fn decode_corner_number(b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
    encode::generic_decode_number16(b.cells.len(), body, d, |c, val| {
        b.cells[c].qnum2 = val;
    })
}

/// `encodeCornerNumber`.
fn encode_corner_number(b: &Board) -> String {
    encode::generic_encode_number16(b.cells.len(), |c| b.cells[c].qnum2)
}

/// `decodeArrowNumber16`.
fn decode_arrow_number16(b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = body.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let mut used = 1usize;
        if ca == '+' {
            let cell = &mut b.cells[c];
            cell.qdir = 0;
            cell.qnum = -3;
        } else if ('0'..='4').contains(&ca) {
            // `dir` + one hex digit, or `.` for `?`.
            let cell = &mut b.cells[c];
            cell.qdir = ca.to_digit(16).unwrap() as i32;
            used = 2;
            let next = chars.get(i + 1).copied();
            if next == Some('.') {
                cell.qnum = -2;
            } else if let Some(v) = next.and_then(|ch| jsnum::digit(ch, 16)) {
                cell.qnum = v as i32;
            } else {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("arrow number at character {i} is not hexadecimal"),
                );
            }
        } else if ('5'..='9').contains(&ca) {
            // `5` + `dir` + two hex digits (`dir` 0-4).
            let cell = &mut b.cells[c];
            cell.qdir = ca.to_digit(16).unwrap() as i32 - 5;
            used = 3;
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(v) => cell.qnum = v as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("arrow number at character {i} is not hexadecimal"),
                ),
            }
        } else if ca == '-' {
            // `-` + `dir` + three hex digits.
            let cell = &mut b.cells[c];
            used = 5;
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 1), 16) {
                Some(v) => cell.qdir = v as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("arrow direction at character {i} is not hexadecimal"),
                ),
            }
            match jsnum::parse_int(&jsnum::substr(&chars, i + 2, 3), 16) {
                Some(v) => cell.qnum = v as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("arrow number at character {i} is not hexadecimal"),
                ),
            }
        } else if ca.is_ascii_lowercase() {
            c += jsnum::digit(ca, 36).unwrap() as usize - 10;
        }
        c += 1;
        i += used;
        if c >= n {
            break;
        }
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    chars.iter().skip(i).collect()
}

/// `encodeArrowNumber16`: the inverse of [`decode_arrow_number16`].
fn encode_arrow_number16(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let dir = cell.qdir;
        let qn = cell.qnum;
        let pstr = if qn == -2 {
            Some(format!("{dir}."))
        } else if qn == -3 {
            Some("+".into())
        } else if (0..16).contains(&qn) {
            Some(format!("{dir}{}", jsnum::to_radix(qn as u32, 16)))
        } else if (16..256).contains(&qn) {
            // pzprjs adds 5 to the direction, then appends the two hex digits.
            Some(format!("{}{}", dir + 5, jsnum::to_radix(qn as u32, 16)))
        } else if (256..4096).contains(&qn) {
            Some(format!("-{dir}{}", jsnum::to_radix(qn as u32, 16)))
        } else {
            None
        };
        match pstr {
            Some(pstr) => {
                if count == 0 {
                    cm.push_str(&pstr);
                } else {
                    cm.push_str(&jsnum::to_radix(count + 9, 36));
                    cm.push_str(&pstr);
                    count = 0;
                }
            }
            None => {
                count += 1;
                if count == 26 {
                    cm.push_str(&jsnum::to_radix(count + 9, 36));
                    count = 0;
                }
            }
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(count + 9, 36));
    }
    cm
}

/// `decodeHeyajirimisaki`: `5,num` is a corner number in `qnum2`, otherwise
/// `dir,num` fills `qdir` and `qnum` (`-` is `?`).
fn decode_heyajirimisaki(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "." {
            return Ok(());
        }
        let mut inp = ca.split(',');
        let dir = inp.next().unwrap_or("");
        let num = inp
            .next()
            .ok_or_else(|| format!("expected \"dir,num\", found {ca:?}"))?;
        let dir = if dir != "0" { fileio::int(dir)? } else { 0 };
        let num = if num != "-" { fileio::int(num)? } else { -2 };
        if dir == 5 {
            cell.qnum2 = num;
        } else {
            cell.qdir = dir;
            cell.qnum = num;
        }
        Ok(())
    })
}

/// `encodeHeyajirimisaki`.
fn encode_heyajirimisaki(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qnum2 >= 0 {
            format!("5,{} ", cell.qnum2)
        } else if cell.qnum != -1 {
            let dir = if cell.qdir != 0 {
                cell.qdir.to_string()
            } else {
                "0".into()
            };
            let num = if cell.qnum != -2 {
                cell.qnum.to_string()
            } else {
                "-".into()
            };
            format!("{dir},{num} ")
        } else {
            ". ".into()
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

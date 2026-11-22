//! Mukkonn Enn (pzprjs `variety/compass.js`, pid `mukkonn`).
//!
//! `ques` 51 is a compass cell with up to four numbers, `ques` 7 a black cell.
//! The four clue slots are pzprjs `qnum` = right, `qnum2` = down, `qnum3` =
//! left and `qnum4` = up (see `Piece.getQnumDir`). The URL is
//! `decodeCompass` (four numbers per compass cell, `_` black cells, `g`–`z`
//! runs of empty cells). FileIO stores the four slots as `qnum,qnum2,qnum3,
//! qnum4` and then the border line layer. Rotations/reflections permute the
//! four slots (`adjustBoardData2` in pzprjs `BoardExec`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Mukkonn;

struct Mukkonn;

impl Genre for Mukkonn {
    fn pid(&self) -> &'static str {
        "mukkonn"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_compass(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_compass(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "#" {
                cell.ques = 7;
            } else if ca != "." {
                let mut inp = ca.split(',');
                let qnum = fileio::int(inp.next().unwrap_or(""))?;
                let qnum2 = fileio::int(inp.next().unwrap_or(""))?;
                let qnum3 = fileio::int(inp.next().unwrap_or(""))?;
                let qnum4 = fileio::int(inp.next().unwrap_or(""))?;
                cell.ques = 51;
                cell.qnum = qnum;
                cell.qnum2 = qnum2;
                cell.qnum3 = qnum3;
                cell.qnum4 = qnum4;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 7 {
                "# ".into()
            } else if cell.ques == 51 {
                format!(
                    "{},{},{},{} ",
                    cell.qnum, cell.qnum2, cell.qnum3, cell.qnum4
                )
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData2`: permute the four compass slots
        // of every `ques` 51 cell, once per board operation after the move.
        for op in turn_ops(t) {
            for cell in &mut b.cells {
                if cell.ques != 51 {
                    continue;
                }
                let (q1, q2, q3, q4) = (cell.qnum, cell.qnum2, cell.qnum3, cell.qnum4);
                match op {
                    Turn::FlipY => {
                        cell.qnum2 = q4;
                        cell.qnum4 = q2;
                    }
                    Turn::FlipX => {
                        cell.qnum = q3;
                        cell.qnum3 = q1;
                    }
                    Turn::Right => {
                        cell.qnum = q4;
                        cell.qnum2 = q1;
                        cell.qnum3 = q2;
                        cell.qnum4 = q3;
                    }
                    Turn::Left => {
                        cell.qnum = q2;
                        cell.qnum2 = q3;
                        cell.qnum3 = q4;
                        cell.qnum4 = q1;
                    }
                }
            }
        }
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell@mukkonn.maxnum`: `max(cols, rows) - 1`; `minnum` 0.
        let maxnum = b.cols().max(b.rows()) as i32 - 1;
        if b.cells.iter().any(|c| {
            !matches!(c.ques, 0 | 7 | 51)
                || [c.qnum, c.qnum2, c.qnum3, c.qnum4]
                    .iter()
                    .any(|&q| !(q == -1 || q == -2 || (0..=maxnum).contains(&q)))
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("mukkonn cell value outside empty/black/compass or clue 0-{maxnum}/?"),
            );
        }
        if b.cells.iter().any(|c| {
            c.ques == 51 && [c.qnum, c.qnum2, c.qnum3, c.qnum4].contains(&-2)
        }) {
            d.push(
                DiagnosticCode::LossyUrl,
                "mukkonn ? clues are dropped to empty slots by the URL",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "mukkonn border value outside line 0/1, cross -1",
            );
        }
    }
}

/// `decodeCompass`: `_` is a black cell, `g`–`z` skips `parseInt(ca, 36) - 15`
/// empty cells, and any other cell is a compass with four `readNumber16`
/// values (a missing value ends the cell after one character).
fn decode_compass(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() && c < n {
        let ca = chars[i];
        if ca == '_' {
            b.cells[c].ques = 7;
            c += 1;
            i += 1;
            continue;
        }
        if ('g'..='z').contains(&ca) {
            c += jsnum::digit(ca, 36).unwrap() as usize - 15;
            i += 1;
            continue;
        }
        b.cells[c].ques = 51;
        for dir in 1..=4 {
            let (val, used) = encode::read_number16(&chars, i);
            match val {
                Some(-1) => {
                    i += 1;
                    break;
                }
                Some(v) => {
                    set_qnum_dir(&mut b.cells[c], dir, if v == -2 { -1 } else { v });
                    i += used;
                }
                None => {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("incomplete hex escape at offset {i} leaves the slot empty"),
                    );
                    i += used;
                }
            }
        }
        c += 1;
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    chars[i.min(chars.len())..].iter().collect()
}

/// `encodeCompass`: `_` black cells, four numbers per compass cell, base-36
/// runs for empty cells (`15 + count`, flushed after 20).
fn encode_compass(b: &Board) -> String {
    let mut count = 0u32;
    let mut cm = String::new();
    for cell in &b.cells {
        let pstr = if cell.ques == 7 {
            "_".to_string()
        } else if cell.ques == 51 {
            let mut pstr = String::new();
            for dir in 1..=4 {
                let mut qn = get_qnum_dir(cell, dir);
                if qn == -1 {
                    qn = -2;
                }
                pstr.push_str(&encode::write_number16(i64::from(qn)));
            }
            pstr
        } else {
            count += 1;
            if count == 20 {
                cm.push_str(&jsnum::to_radix(15 + count, 36));
                count = 0;
            }
            continue;
        };
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 20 {
            cm.push_str(&jsnum::to_radix(15 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(15 + count, 36));
    }
    cm
}

/// pzprjs `Cell.getQnumDir`: UP=`qnum4`, DN=`qnum2`, LT=`qnum3`, RT=`qnum`.
fn get_qnum_dir(cell: &crate::Obj, dir: i32) -> i32 {
    match dir {
        1 => cell.qnum4,
        2 => cell.qnum2,
        3 => cell.qnum3,
        4 => cell.qnum,
        _ => -1,
    }
}

/// pzprjs `Cell.setQnumDir`.
fn set_qnum_dir(cell: &mut crate::Obj, dir: i32, val: i32) {
    match dir {
        1 => cell.qnum4 = val,
        2 => cell.qnum2 = val,
        3 => cell.qnum3 = val,
        4 => cell.qnum = val,
        _ => {}
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

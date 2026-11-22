//! Loop Special (pzprjs `variety/pipelink.js`, pid `loopsp`).
//!
//! Cells hold a clue number in `qnum` (0–4, `?` is -2) or one of the six line
//! parts in `ques` 12–17 (`ques` 11 is a direction-less part; `ques` 6 is the
//! inherited pipelink circle and -2 the `?` mark the FileIO layer can also
//! carry). The URL is `decodeLoopsp`; FileIO stores the cell layer and then a
//! single loop on the borders (`decodeBorderLine`). Rotations/reflections
//! permute the line parts (`adjustBoardData` in `BoardExec`).

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Loopsp;

struct Loopsp;

impl Genre for Loopsp {
    fn pid(&self) -> &'static str {
        "loopsp"
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
        decode_loopsp(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_loopsp(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        // `"loopsp" !== this.pid` is false, so no obsolete dispType line is read.
        decode_pipelink_fileio(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_pipelink_fileio(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows() / 4) as i32;
        if b.cells.iter().any(|c| {
            !matches!(c.ques, -2 | 0 | 6 | 11..=17)
                || !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "loopsp cell value outside empty / ? / line part 11-17 / clue 0..maxnum",
            );
        }
        if b.cells.iter().any(|c| {
            c.qnum >= 4096
                || (c.qnum >= 0 && (11..=17).contains(&c.ques))
                || (c.qnum == -1 && matches!(c.ques, -2 | 6))
        }) {
            d.push(
                DiagnosticCode::LossyUrl,
                "loopsp question data dropped from a URL: a direction-less cell cannot carry the `?` mark or circle, a line part loses to a clue, and numbers of 4096 or more are not stored",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "loopsp border value outside line 0/1, cross -1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData` (pipelink.js, `loopsp` shares it):
    /// rotate/flip the line parts stored in `cell.ques`, before the coordinates
    /// move, once per board operation.
    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        for op in ops(t) {
            for cell in &mut b.cells {
                if let Some(qu) = translate_ques(*op, cell.ques) {
                    cell.ques = qu;
                }
            }
        }
    }
}

/// `decodeLoopsp`: `.` is `?`, `0`–`9`/`a`–`f` one hex clue, `-` plus two hex
/// digits a clue 16–255, `+` plus three a clue 256–4095, `g`–`m` the line
/// parts 11–17, and `n`–`z` blank runs that skip `digit - 23` cells.
fn decode_loopsp(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let Some(cell) = b.cells.get_mut(c) else {
            break;
        };
        // pzprjs sets `qnum` for `.` first and then still runs the hex checks.
        if ca == '.' {
            cell.qnum = -2;
        }
        if ca.is_ascii_digit() || ('a'..='f').contains(&ca) {
            cell.qnum = jsnum::digit(ca, 16).unwrap() as i32;
        } else if ca == '-' {
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qn) => cell.qnum = qn as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal number after '-'",
                ),
            }
            i += 2;
        } else if ca == '+' {
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 3), 16) {
                Some(qn) => cell.qnum = qn as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal number after '+'",
                ),
            }
            i += 3;
        } else if ('g'..='m').contains(&ca) {
            cell.ques = jsnum::digit(ca, 36).unwrap() as i32 - 5;
        } else if ('n'..='z').contains(&ca) {
            c += jsnum::digit(ca, 36).unwrap() as usize - 23;
        } else if ca != '.' {
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

/// `encodeLoopsp`: the inverse of [`decode_loopsp`].
fn encode_loopsp(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let (qn, qu) = (cell.qnum, cell.ques);
        let pstr = if qn == -2 {
            Some(".".into())
        } else if (0..16).contains(&qn) {
            Some(jsnum::to_radix(qn as u32, 16))
        } else if (16..256).contains(&qn) {
            Some(format!("-{}", jsnum::to_radix(qn as u32, 16)))
        } else if (256..4096).contains(&qn) {
            Some(format!("+{}", jsnum::to_radix(qn as u32, 16)))
        } else if (11..=17).contains(&qu) {
            Some(jsnum::to_radix((qu + 5) as u32, 36))
        } else {
            None
        };
        match pstr {
            Some(pstr) => {
                if count == 0 {
                    cm.push_str(&pstr);
                } else {
                    cm.push_str(&jsnum::to_radix(22 + count, 36));
                    cm.push_str(&pstr);
                    count = 0;
                }
            }
            None => {
                count += 1;
                if count == 13 {
                    cm.push_str(&jsnum::to_radix(22 + count, 36));
                    count = 0;
                }
            }
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(22 + count, 36));
    }
    cm
}

/// `decodePipelink` (FileIO, shared with `pipelink`): `o` ice, `-` unknown,
/// `a`–`g` line parts, `.` empty, anything else a clue number.
fn decode_pipelink_fileio(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "o" {
            cell.ques = 6;
        } else if ca == "-" {
            cell.ques = -2;
        } else if ca.len() == 1 && ('a'..='g').contains(&ca.chars().next().unwrap()) {
            cell.ques = jsnum::digit(ca.chars().next().unwrap(), 36).unwrap() as i32 + 1;
        } else if ca != "." {
            cell.qnum = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodePipelink` (FileIO, shared with `pipelink`).
fn encode_pipelink_fileio(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.ques == 6 {
            "o ".into()
        } else if cell.ques == -2 {
            "- ".into()
        } else if (11..=17).contains(&cell.ques) {
            format!("{} ", jsnum::to_radix((cell.ques - 1) as u32, 36))
        } else if cell.qnum != -1 {
            format!("{} ", cell.qnum)
        } else {
            ". ".into()
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

/// `adjustBoardData`'s `tques` tables (pipelink.js, `loopsp` shares them): the
/// line parts 12–17 change with the operation, other values stay put.
fn translate_ques(op: Op, ques: i32) -> Option<i32> {
    let map: &[(i32, i32)] = match op {
        Op::FlipY => &[(14, 17), (15, 16), (16, 15), (17, 14)],
        Op::FlipX => &[(14, 15), (15, 14), (16, 17), (17, 16)],
        Op::TurnR => &[(12, 13), (13, 12), (14, 17), (15, 14), (16, 15), (17, 16)],
        Op::TurnL => &[(12, 13), (13, 12), (14, 15), (15, 16), (16, 17), (17, 14)],
    };
    map.iter().find(|(from, _)| *from == ques).map(|&(_, to)| to)
}

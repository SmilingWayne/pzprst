//! Recoil (pzprjs `variety/recoil.js`, pid `recoil`).
//!
//! pzprjs keeps the start position in an extra `board.startpos` object; pzprst's
//! board model has no extra objects, so the marker lives on cell 0: `qdir = 1`
//! means "explicit start", with the doubled coordinates in `qnum3`/`qnum4`.
//! The default (pzprjs `initExtraObject`) is the largest odd coordinate that
//! fits the board, which equals the URL body of an empty start marker.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Recoil;

struct Recoil;

impl Genre for Recoil {
    fn pid(&self) -> &'static str {
        "recoil"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let chars: Vec<char> = body.chars().collect();
        let (bx, used) = decode_one_number16(&chars, d);
        let rest: Vec<char> = chars[used.min(chars.len())..].to_vec();
        let (by, used) = decode_one_number16(&rest, d);
        let rest: String = rest[used.min(rest.len())..].iter().collect();
        set_startpos(b, (2 * bx - 1, 2 * by - 1));
        encode::decode_empty(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let (bx, by) = startpos(b);
        let mut cm = encode::write_number16((i64::from(bx) + 1) / 2);
        cm += &encode::write_number16((i64::from(by) + 1) / 2);
        cm += &encode::encode_empty(b);
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_s(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            cell.ques = if ca == "#" { 7 } else { 0 };
            Ok(())
        })?;
        decode_border_arrow_ans(r, b, d)?;
        fileio::decode_cell_qsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_s(b, w);
        w.encode_cell(b, |cell| {
            if cell.ques == 7 {
                "# ".into()
            } else {
                ". ".into()
            }
        });
        encode_border_arrow_ans(b, w);
        fileio::encode_cell_qsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.ques == 0 || c.ques == 7) || c.qnum != -1 || c.qsub < 0
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "recoil cell value outside empty 0 / shaded 7 or background mark",
            );
        }
        if b.borders.iter().any(|e| {
            e.line < 0 || !matches!(e.qsub, 0 | 2 | 11 | 12 | 13 | 14)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "recoil border value outside line >=0, cross -1, direction mark u/d/l/r",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData` + `adjustBoardData2`: the start
    /// position follows the board operation.
    ///
    /// The two pzprjs hooks only record the mapped position before the move and
    /// restore it after; computing the same map on the transformed board is
    /// equivalent. Operations run in `tools/gen_fixtures.js` order.
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        let (mut cols, mut rows) = if t.swaps_axes() {
            (b.rows(), b.cols())
        } else {
            (b.cols(), b.rows())
        };
        let mut pos = stored_startpos(b).unwrap_or_else(|| default_pos(cols, rows));
        for op in ops(t) {
            let (bx, by) = pos;
            let (c, r) = (cols as i32, rows as i32);
            pos = match op {
                // pzprjs `getAfterPos`: TURNR maps (bx, by) to (2R - by, bx).
                Op::TurnR => (2 * r - by, bx),
                Op::TurnL => (by, 2 * c - bx),
                Op::FlipX => (2 * c - bx, by),
                Op::FlipY => (bx, 2 * r - by),
            };
            if matches!(op, Op::TurnR | Op::TurnL) {
                std::mem::swap(&mut cols, &mut rows);
            }
        }
        set_startpos(b, pos);
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

/// pzprjs `initExtraObject`: the largest odd doubled coordinate per axis.
fn default_pos(cols: usize, rows: usize) -> (i32, i32) {
    let odd = |n: usize| if n % 2 == 1 { n as i32 } else { n as i32 - 1 };
    (odd(cols), odd(rows))
}

/// The explicit start marker if a board carries one.
fn stored_startpos(b: &Board) -> Option<(i32, i32)> {
    b.cells
        .iter()
        .find(|cell| cell.qdir == 1)
        .map(|cell| (cell.qnum3, cell.qnum4))
}

/// The start position, explicit marker or pzprjs default.
fn startpos(b: &Board) -> (i32, i32) {
    stored_startpos(b).unwrap_or_else(|| default_pos(b.cols(), b.rows()))
}

/// Store the start position on cell 0, normalizing the default to "no marker".
fn set_startpos(b: &mut Board, pos: (i32, i32)) {
    for cell in &mut b.cells {
        if cell.qdir == 1 {
            cell.qdir = 0;
            cell.qnum3 = -1;
            cell.qnum4 = -1;
        }
    }
    if pos == default_pos(b.cols(), b.rows()) {
        return;
    }
    b.cells[0].qdir = 1;
    b.cells[0].qnum3 = pos.0;
    b.cells[0].qnum4 = pos.1;
}

/// `decodeOneNumber16`: one number, leaving the rest for the caller.
fn decode_one_number16(chars: &[char], d: &mut Diagnostics) -> (i32, usize) {
    match encode::read_number16(chars, 0) {
        (Some(v), used) => (v, used),
        (None, used) => {
            d.push(
                DiagnosticCode::UnexpectedChar,
                "incomplete hex escape leaves the start position at -1",
            );
            (-1, used)
        }
    }
}

/// `decodeS`: the start position line.
fn decode_s(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let line = r.read_line().unwrap_or("").to_string();
    let mut parts = line.split(' ');
    let mut coord = |what: &str| -> i32 {
        let token = parts.next().unwrap_or("");
        match jsnum::parse_int(token, 10) {
            Some(n) => n as i32,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("no base-10 start {what} in {token:?}"),
                );
                0
            }
        }
    };
    let bx = coord("x");
    let by = coord("y");
    set_startpos(b, (bx, by));
    Ok(())
}

/// `encodeS`.
fn encode_s(b: &Board, w: &mut FileWriter) {
    let (bx, by) = startpos(b);
    w.write_line(&format!("{bx} {by}"));
}

/// `decodeBorderArrowAns`: lines, crosses (`-n`) and u/d/l/r aux marks.
fn decode_border_arrow_ans(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_border(b, d, |border, ca| {
        let mut ca = ca;
        if let Some(last) = ca.chars().last() {
            if last.is_ascii_lowercase() {
                border.qsub = match last {
                    'u' => 11,
                    'd' => 12,
                    'l' => 13,
                    'r' => 14,
                    _ => border.qsub,
                };
                ca = &ca[..ca.len() - last.len_utf8()];
            }
        }
        if !ca.is_empty() && ca != "0" {
            if ca.starts_with('-') {
                border.line = -fileio::int(ca)? - 1;
                border.qsub = 2;
            } else {
                border.line = fileio::int(ca)?;
            }
        }
        Ok(())
    })
}

/// `encodeBorderArrowAns`.
fn encode_border_arrow_ans(b: &Board, w: &mut FileWriter) {
    w.encode_border(b, |border| {
        let mut ca = String::new();
        if border.qsub == 2 {
            ca.push_str(&format!("{}", -1 - border.line));
        } else if border.line > 0 {
            ca.push_str(&format!("{}", border.line));
        }
        if border.qsub >= 11 {
            ca.push_str(match border.qsub {
                11 => "u",
                12 => "d",
                13 => "l",
                14 => "r",
                _ => "",
            });
        }
        if ca.is_empty() {
            "0 ".into()
        } else {
            format!("{ca} ")
        }
    });
}

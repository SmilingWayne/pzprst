//! Rule of Three (pzprjs `variety/amibo.js`, pid `ruleofthree`).
//!
//! The URL stores only the shaded cells (`ques == 7`); the FileIO stores the
//! numbers and the three direction bars (`qans` 11/12/13).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Ruleofthree;

struct Ruleofthree;

impl Genre for Ruleofthree {
    fn pid(&self) -> &'static str {
        "ruleofthree"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_empty(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_empty(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            let mut ca = ca;
            if let Some(rest) = ca.strip_prefix('o') {
                cell.qsub = 1;
                ca = rest;
            }
            match ca {
                "l" => cell.qans = 12,
                "-" => cell.qans = 13,
                "+" => cell.qans = 11,
                "x" => cell.ques = 7,
                "#" => cell.qnum = -2,
                "." => {}
                _ => cell.qnum = fileio::int(ca)?,
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            let mut ca = if cell.qans == 12 {
                "l".to_string()
            } else if cell.qans == 13 {
                "-".to_string()
            } else if cell.qans == 11 {
                "+".to_string()
            } else if cell.ques == 7 {
                "x".to_string()
            } else if cell.qnum >= 0 {
                cell.qnum.to_string()
            } else if cell.qnum == -2 {
                "#".to_string()
            } else {
                ".".to_string()
            };
            if cell.qsub == 1 {
                ca.insert(0, 'o');
            }
            format!("{ca} ")
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let (maxbx, maxby) = (2 * cols, 2 * rows);
        let mut bad = false;
        let mut lossy = false;
        for (i, cell) in b.cells.iter().enumerate() {
            let (bx, by) = b.geom.cell_pos(i);
            let col = (if bx < maxbx >> 1 { maxbx - bx } else { bx }) >> 1;
            let row = (if by < maxby >> 1 { maxby - by } else { by }) >> 1;
            let maxnum = col.max(row);
            let qnum_ok = cell.qnum == -1
                || cell.qnum == -2
                || (maxnum >= 2 && (2..=maxnum).contains(&cell.qnum));
            if !qnum_ok
                || !(cell.ques == 0 || cell.ques == 7)
                || !matches!(cell.qans, 0 | 11 | 12 | 13)
                || !(cell.qsub == 0 || cell.qsub == 1)
            {
                bad = true;
            }
            if cell.qnum != -1 {
                lossy = true;
            }
        }
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "ruleofthree cell value outside clue 2-max / ? / shaded / direction bar",
            );
        }
        if lossy {
            d.push(
                DiagnosticCode::LossyUrl,
                "ruleofthree clues are dropped from a URL: it stores shaded cells only",
            );
        }
        if b.borders
            .iter()
            .any(|e| e.line < 0 || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "ruleofthree border value outside line >=0, cross -1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`: quarter turns swap the two diagonal
    /// bars; mirrors leave them alone.
    ///
    /// pzprjs runs this before the coordinates move; the swap commutes with the
    /// move and the operations run in `tools/gen_fixtures.js` order.
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        for op in ops(t) {
            if !matches!(op, Op::TurnR | Op::TurnL) {
                continue;
            }
            for cell in &mut b.cells {
                cell.qans = match cell.qans {
                    12 => 13,
                    13 => 12,
                    other => other,
                };
            }
        }
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

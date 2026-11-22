//! Amibo (pzprjs `variety/amibo.js`, pid `amibo`). `ruleofthree` shares the
//! script but overrides `Board.hasborder` and `Encode`, so it is a separate pid
//! with its own genre file.
//!
//! A cell has a clue number in `qnum` (-1 empty, -2 `?`, at least 2 up to the
//! cell's positional maximum), a blocked-cell mark (`ques = 7`, FileIO `x`), a
//! direction bar (`qans` 11/12/13) and a circle mark (`qsub = 1`). The URL
//! stores only the numbers (`decodeNumber10or16`); FileIO adds the answer layer
//! (bars, circles, blocked cells) and a border line layer. Quarter turns swap
//! the two diagonal bars (`BoardExec.adjustBoardData`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Amibo;

struct Amibo;

impl Genre for Amibo {
    fn pid(&self) -> &'static str {
        "amibo"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number10or16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number10or16(b))
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
            if cell.ques == 7 {
                lossy = true;
            }
        }
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "amibo cell value outside clue 2-max / ? / blocked cell or answer bar",
            );
        }
        if lossy {
            d.push(
                DiagnosticCode::LossyUrl,
                "amibo blocked cells are dropped from a URL",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "amibo border value outside line 0/1 or cross -1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`: a turn swaps the two diagonal bars;
    /// mirrors leave them alone. pzprjs calls the hook once per board operation,
    /// so two turns cancel, as they do in `turnr turnr`.
    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        for op in turn_ops(t) {
            if !matches!(op, Turn::Right | Turn::Left) {
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

/// The pzprjs board operations a pzprst transform is composed of.
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

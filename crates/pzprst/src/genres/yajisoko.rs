//! Yajisan-Sokoban (pzprjs `variety/yosenabe.js`, pid `yajisoko`).
//! `yosenabe` and `brownies` share the script but override `Encode`/`FileIO`,
//! so they are separate pids with their own genre files.
//!
//! A cell has a dark-cell mark (`qnum = -2`, FileIO `#`), an arrow direction
//! `qdir` (0-4) and a filling number in `qnum2` (-1 empty, -2 `?`, -4 `∞`,
//! -5 no-line mark, 0..max(cols,rows)-1 clues). The URL packs the number and
//! direction in `genericDecodeNumber16` and stores the -5 marks in a
//! `decodeBinary` layer; FileIO stores `dir,num` per cell, the answer line
//! layer and a packed `qsub` / `qcmp` mark. Rotations/reflections rotate
//! `qdir` (`adjustNumberArrow` in `BoardExec.adjustBoardData`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Yajisoko;

struct Yajisoko;

/// `decodeCellDirecQnum`, the yajisoko variant: `#` is the dark cell, otherwise
/// `dir,num` fills `qdir` / `qnum2`.
fn decode_cell_direc_qnum(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "#" {
            cell.qdir = 0;
            cell.qnum = -2;
        } else if ca != "." {
            let mut inp = ca.split(',');
            let dir = inp
                .next()
                .ok_or_else(|| format!("expected \"dir,num\", found {ca:?}"))?;
            let num = inp
                .next()
                .ok_or_else(|| format!("expected \"dir,num\", found {ca:?}"))?;
            cell.qdir = fileio::int(dir)?;
            cell.qnum2 = fileio::int(num)?;
        }
        Ok(())
    })
}

/// `encodeCellDirecQnum`, the yajisoko variant.
fn encode_cell_direc_qnum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qnum == -2 {
            "# ".into()
        } else if cell.qnum2 != -1 {
            format!("{},{} ", cell.qdir, cell.qnum2)
        } else {
            ". ".into()
        }
    });
}

/// `decodeCellQsubQcmp`: one token packs `qsub` in the low nibble and `qcmp`
/// above it; `0` means both are zero.
fn decode_cell_qsub_qcmp(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca != "0" {
            let val = fileio::int(ca)?;
            cell.qsub = val & 0x0f;
            cell.qcmp = val >> 4;
        }
        Ok(())
    })
}

/// `encodeCellQsubQcmp`.
fn encode_cell_qsub_qcmp(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        format!("{} ", cell.qsub + (cell.qcmp << 4))
    });
}

impl Genre for Yajisoko {
    fn pid(&self) -> &'static str {
        "yajisoko"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let n = b.cells.len();
        let rest = encode::generic_decode_number16(n, body, d, |c, val| {
            let cell = &mut b.cells[c];
            if val == -2 {
                cell.qnum = -2;
            } else if val != -1 {
                cell.qdir = val % 5;
                let num = val / 5;
                cell.qnum2 = match num {
                    0 => -4,
                    1 => -2,
                    _ => num - 2,
                };
            }
        });
        encode::decode_binary(b, &rest, d, |cell| cell.qnum2 = -5)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut cm = encode::generic_encode_number16(b.cells.len(), |c| {
            let cell = &b.cells[c];
            if cell.qnum2 == -1 {
                cell.qnum
            } else {
                let factor = if cell.qnum2 == -4 {
                    0
                } else if cell.qnum2 == -2 {
                    1
                } else {
                    cell.qnum2 + 2
                };
                factor * 5 + cell.qdir
            }
        });
        cm.push_str(&encode::encode_binary(b, |cell| cell.qnum2 == -5, true));
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_cell_direc_qnum(r, b, d)?;
        fileio::decode_border_line(r, b, d)?;
        decode_cell_qsub_qcmp(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_direc_qnum(w, b);
        fileio::encode_border_line(w, b);
        encode_cell_qsub_qcmp(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = b.cols().max(b.rows()) as i32 - 1;
        let mut bad = false;
        let mut lossy = false;
        for cell in &b.cells {
            let qnum2_ok = cell.qnum2 == -1
                || cell.qnum2 == -2
                || cell.qnum2 == -4
                || cell.qnum2 == -5
                || (maxnum >= 0 && (0..=maxnum).contains(&cell.qnum2));
            if !(cell.qnum == -1 || cell.qnum == -2)
                || !qnum2_ok
                || !(0..=4).contains(&cell.qdir)
                || !(0..=15).contains(&cell.qsub)
            {
                bad = true;
            }
            // Only `qnum2` is written when it is set; `qdir` travels with it
            // except on the -5 layer, and a dark cell hides whatever else it
            // holds.
            if (cell.qnum2 != -1 && cell.qnum != -1)
                || (cell.qnum2 == -1 && cell.qdir != 0)
                || (cell.qnum2 == -5 && cell.qdir != 0)
            {
                lossy = true;
            }
        }
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "yajisoko cell value outside empty / dark cell, number / ? / infinity / no-line or direction 0..4",
            );
        }
        if lossy {
            d.push(
                DiagnosticCode::LossyUrl,
                "yajisoko numbers without a direction slot, or a direction on an empty/-5 cell, are dropped from the URL",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "yajisoko border value outside line 0/1 or cross -1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`: `adjustNumberArrow` rotates the
    /// arrow direction before the coordinates move.
    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        adjust_cell_qdirs(b, t);
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

/// `adjustCellArrowField(key, d, "qdir")`: each board operation turns the
/// arrow directions before the coordinates move.
fn adjust_cell_qdirs(b: &mut Board, t: Transform) {
    for op in turn_ops(t) {
        for cell in &mut b.cells {
            if let Some(dir) = translate_dir(*op, cell.qdir) {
                cell.qdir = dir;
            }
        }
    }
}

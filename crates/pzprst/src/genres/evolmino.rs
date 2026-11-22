//! Evolmino (pzprjs `variety/evolmino.js`, pid `evolmino`).
//!
//! The URL stores one base-3 cell value (0 empty, 1 black, 2 square) and, after
//! it, the border arrows as two base-36 runs. FileIO stores a `#`/`0`/`.` cell
//! layer, a `1`/`2` arrow layer and an answer layer with `0` (anum) and `+`
//! (qsub). `adjustBoardData` turns every border qdir.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Evolmino;

struct Evolmino;

/// pzprjs directions: `UP`, `DN`, `LT`, `RT`.
const UP: i32 = 1;
const DN: i32 = 2;
const LT: i32 = 3;
const RT: i32 = 4;

/// One pzprjs `BoardExec` rotation/reflection operation.
#[derive(Clone, Copy)]
enum Op {
    TurnR,
    TurnL,
    FlipX,
    FlipY,
}

/// The pzprjs operations `transform` is composed of, in order (tools/gen_fixtures.js).
fn operations(t: Transform) -> &'static [Op] {
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

/// `getTranslateDir`: map each direction to its image under one operation.
fn translate_dir(op: Op) -> &'static [(i32, i32)] {
    match op {
        Op::FlipY => &[(1, 2), (2, 1)],
        Op::FlipX => &[(3, 4), (4, 3)],
        Op::TurnR => &[(1, 4), (2, 3), (3, 1), (4, 2)],
        Op::TurnL => &[(1, 3), (2, 4), (3, 2), (4, 1)],
    }
}

fn is_vertical(b: &Board, i: usize) -> bool {
    let (bx, _) = b.geom.border_pos(i);
    bx & 1 == 0
}

/// Number of inner borders: `2 * cols * rows - cols - rows`.
fn inner_borders(b: &Board) -> usize {
    let (c, r) = (b.cols() as i32, b.rows() as i32);
    (2 * c * r - c - r).max(0) as usize
}

/// `decodeBorderArrow`: one run per direction pair, pzprjs border ids.
fn decode_border_arrow(b: &mut Board, bstr: &str) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let bdinside = inner_borders(b);
    let vertical: Vec<bool> = (0..bdinside)
        .map(|id| b.border_by_id(id).is_some_and(|i| is_vertical(b, i)))
        .collect();
    let mut a = 0;
    a = arrow_pass(b, &chars, a, bdinside, &vertical, true);
    a = arrow_pass(b, &chars, a, bdinside, &vertical, false);
    chars[a.min(chars.len())..].iter().collect()
}

/// One pzprjs run-length pass of `decodeBorderArrow`; returns the new start.
fn arrow_pass(
    b: &mut Board,
    chars: &[char],
    start: usize,
    bdinside: usize,
    vertical: &[bool],
    first: bool,
) -> usize {
    let mut id: i32 = 0;
    let mut a = start;
    let mut nan = false;
    let mut i = start;
    while i < chars.len() {
        let ca = chars[i];
        if ca == 'z' {
            if !nan {
                id += 35;
            }
        } else {
            match crate::jsnum::digit(ca, 36) {
                Some(v) if !nan => {
                    id += v as i32;
                    if id >= 0 && (id as usize) < bdinside {
                        if let Some(idx) = b.border_by_id(id as usize) {
                            b.borders[idx].qdir = if vertical[id as usize] {
                                if first {
                                    LT
                                } else {
                                    RT
                                }
                            } else if first {
                                UP
                            } else {
                                DN
                            };
                        }
                    }
                    id += 1;
                }
                _ => nan = true,
            }
        }
        if !nan && id >= bdinside as i32 {
            a = i + 1;
            break;
        }
        i += 1;
    }
    a
}

/// `encodeBorderArrow`: two run-length passes over the pzprjs border ids.
fn encode_border_arrow(b: &Board) -> String {
    let bdinside = inner_borders(b);
    let mut cm = String::new();
    for first in [true, false] {
        let mut num = 0u32;
        for id in 0..bdinside {
            let dir = b.border_by_id(id).map_or(0, |i| b.borders[i].qdir);
            let set = if first {
                dir == UP || dir == LT
            } else {
                dir == DN || dir == RT
            };
            if set {
                cm.push_str(&crate::jsnum::to_radix(num, 36));
                num = 0;
            } else {
                num += 1;
                if num >= 35 {
                    cm.push('z');
                    num = 0;
                }
            }
        }
        if num > 0 {
            cm.push_str(&crate::jsnum::to_radix(num, 36));
        }
    }
    cm
}

impl Genre for Evolmino {
    fn pid(&self) -> &'static str {
        "evolmino"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_three(b.cells.len(), body, d, |c, val| {
            if val == 1 {
                // 1: black cell
                b.cells[c].ques = 1;
                b.cells[c].qnum = -1;
            } else if val == 2 {
                // 2: square
                b.cells[c].ques = 0;
                b.cells[c].qnum = 1;
            }
        });
        decode_border_arrow(b, &rest)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_three(b.cells.len(), |c| {
            if b.cells[c].ques == 1 {
                1
            } else if b.cells[c].qnum == 1 {
                2
            } else {
                0
            }
        })?;
        body.push_str(&encode_border_arrow(b));
        Ok(body)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "#" {
                cell.ques = 1;
            } else if ca == "0" {
                cell.qnum = 1;
            }
            Ok(())
        })?;
        // The arrow token means different directions on vertical and horizontal
        // borders, and `decode_border`'s closure does not see the position:
        // collect the tokens, then apply them in FileIO order.
        let mut values = Vec::new();
        r.decode_border(b, d, |_border, ca| {
            values.push(fileio::int(ca)?);
            Ok(())
        })?;
        for (i, &val) in values.iter().enumerate() {
            let vertical = is_vertical(b, i);
            if val == 1 && !vertical {
                b.borders[i].qdir = UP;
            } else if val == 2 && !vertical {
                b.borders[i].qdir = DN;
            } else if val == 1 && vertical {
                b.borders[i].qdir = LT;
            } else if val == 2 && vertical {
                b.borders[i].qdir = RT;
            }
        }
        r.decode_cell(b, d, |cell, ca| {
            if ca == "0" {
                cell.anum = 1;
            } else if ca == "+" {
                cell.qsub = 1;
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 1 {
                "# ".into()
            } else if cell.qnum == 1 {
                "0 ".into()
            } else {
                ". ".into()
            }
        });
        let mut i = 0;
        w.encode_border(b, |_border| {
            let dir = b.borders[i].qdir;
            i += 1;
            if dir == UP || dir == LT {
                "1 ".into()
            } else if dir == DN || dir == RT {
                "2 ".into()
            } else {
                "0 ".into()
            }
        });
        w.encode_cell(b, |cell| {
            if cell.anum == 1 {
                "0 ".into()
            } else if cell.qsub == 1 {
                "+ ".into()
            } else {
                ". ".into()
            }
        });
        Ok(())
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // `adjustBoardData` only remaps qdir; map values are position
        // independent, so replaying the composed operations after the
        // coordinate move gives the pzprjs result.
        for &op in operations(t) {
            let trans = translate_dir(op);
            for border in &mut b.borders {
                if let Some(&(_, to)) = trans.iter().find(|&&(from, _)| from == border.qdir) {
                    border.qdir = to;
                }
            }
        }
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == 1)
                || !(c.ques == 0 || c.ques == 1)
                || !(c.anum == -1 || c.anum == 1)
                || !(c.qsub == 0 || c.qsub == 1)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "evolmino cell value outside empty/black/square, answer 0/+",
            );
        }
        if b.borders.iter().any(|e| !(0..=4).contains(&e.qdir)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "evolmino border value outside arrow 0-4",
            );
        }
    }
}

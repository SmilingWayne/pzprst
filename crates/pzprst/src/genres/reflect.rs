//! Reflect Link (pzprjs `variety/reflect.js`, pid `reflect`).
//!
//! Cells carry a direction triangle in `ques` (2–5), the special mark 11 and an
//! optional clue number in `qnum`; FileIO adds the line borders. The URL is the
//! genre's own `decodeReflectlink`/`encodeReflectlink`.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Reflect;

struct Reflect;

impl Genre for Reflect {
    fn pid(&self) -> &'static str {
        "reflect"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_reflectlink(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_reflectlink(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "+" {
                cell.ques = 11;
            } else if ca != "." {
                let mut chars = ca.chars();
                let first = chars.next().expect("FileIO token is not empty");
                cell.ques = fileio::int(&first.to_string())? + 1;
                let rest: String = chars.collect();
                if !rest.is_empty() {
                    cell.qnum = fileio::int(&rest)?;
                }
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 11 {
                "+ ".into()
            } else if (2..=5).contains(&cell.ques) {
                let number = if cell.qnum != -1 {
                    cell.qnum.to_string()
                } else {
                    String::new()
                };
                format!("{}{} ", cell.ques - 1, number)
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let mut out_of_range = false;
        let mut lossy = false;
        for (i, cell) in b.cells.iter().enumerate() {
            let (bx, by) = b.geom.cell_pos(i);
            let qu = cell.ques;
            if !matches!(qu, 0 | 2 | 3 | 4 | 5 | 11) {
                out_of_range = true;
                continue;
            }
            if (2..=5).contains(&qu) {
                // `Cell.maxnum`: the triangle points at the line that must be
                // that long, so the maximum depends on position and direction.
                let cx = bx >> 1;
                let cy = by >> 1;
                let mut maxnum = -1;
                if qu == 2 || qu == 3 {
                    maxnum += cy + 1;
                } else {
                    maxnum += rows - cy;
                }
                if qu == 3 || qu == 4 {
                    maxnum += cx + 1;
                } else {
                    maxnum += cols - cx;
                }
                if !(cell.qnum == -1 || (3..=maxnum).contains(&cell.qnum)) {
                    out_of_range = true;
                }
                if cell.qnum >= 256 {
                    lossy = true;
                }
            } else if cell.qnum != -1 {
                out_of_range = true;
            }
        }
        if out_of_range {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "reflect cell value outside triangle 2-5 / mark 11 / clue 3-maxnum",
            );
        }
        if lossy {
            d.push(
                DiagnosticCode::LossyUrl,
                "reflect clue numbers 256 or more are dropped from the URL",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "reflect border value outside line 0/1, cross -1",
            );
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        apply_ops(t, |op| {
            for cell in &mut b.cells {
                cell.ques = rotate_ques(op, cell.ques);
            }
        });
    }
}

/// The pzprjs board operations behind a composed `Transform`, in `BoardExec`
/// order: `rot90` = `turnr`, `rot180` = `turnr`+`turnr`, `transpose` =
/// `turnl`+`flipy`, `antitranspose` = `turnr`+`flipy`.
fn apply_ops(t: Transform, mut op: impl FnMut(Transform)) {
    match t {
        Transform::Identity => {}
        Transform::Rot90 => op(Transform::Rot90),
        Transform::Rot180 => {
            op(Transform::Rot90);
            op(Transform::Rot90);
        }
        Transform::Rot270 => op(Transform::Rot270),
        Transform::FlipX => op(Transform::FlipX),
        Transform::FlipY => op(Transform::FlipY),
        Transform::Transpose => {
            op(Transform::Rot270);
            op(Transform::FlipY);
        }
        Transform::AntiTranspose => {
            op(Transform::Rot90);
            op(Transform::FlipY);
        }
    }
}

/// `adjustBoardData`'s `tques` maps; every other value is left alone.
fn rotate_ques(op: Transform, ques: i32) -> i32 {
    match op {
        Transform::FlipY => match ques {
            2 => 5,
            3 => 4,
            4 => 3,
            5 => 2,
            _ => ques,
        },
        Transform::FlipX => match ques {
            2 => 3,
            3 => 2,
            4 => 5,
            5 => 4,
            _ => ques,
        },
        Transform::Rot90 => match ques {
            2 => 5,
            3 => 2,
            4 => 3,
            5 => 4,
            _ => ques,
        },
        Transform::Rot270 => match ques {
            2 => 3,
            3 => 4,
            4 => 5,
            5 => 2,
            _ => ques,
        },
        _ => ques,
    }
}

/// `decodeReflectlink`: `5` mark, `1`–`4` + one hex digit, `6`–`9` + two hex
/// digits for clues 16–255, `a`–`z` run lengths of empty cells.
fn decode_reflectlink(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let Some(cell) = b.cells.get_mut(c) else {
            break;
        };
        if ca == '5' {
            cell.ques = 11;
        } else if ('1'..='4').contains(&ca) {
            cell.ques = ca.to_digit(10).unwrap() as i32 + 1;
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 1), 16) {
                Some(qnum) => cell.qnum = qnum as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hex digit after a triangle leaves the clue empty",
                ),
            }
            i += 1;
        } else if ('6'..='9').contains(&ca) {
            cell.ques = ca.to_digit(10).unwrap() as i32 - 4;
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qnum) => cell.qnum = qnum as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no two hex digits after a large triangle leave the clue empty",
                ),
            }
            i += 2;
        } else if ca.is_ascii_lowercase() {
            c += jsnum::digit(ca, 36).unwrap() as usize - 10;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} skips one cell"),
            );
        }
        if cell.qnum == 0 {
            cell.qnum = -1;
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
    // pzprjs leaves the overflowing character in `outbstr` (`bstr.substr(i)`);
    // consume it like `decodeNumber10` so a full body is not trailing data.
    chars.iter().skip(i + 1).collect()
}

/// `encodeReflectlink`: triangle digit plus 0–2 hex digits, `5` for the mark and
/// base-36 run lengths for empty cells.
fn encode_reflectlink(b: &Board) -> String {
    let mut cm = String::new();
    let mut pstr = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let qu = cell.ques;
        if qu == 11 {
            pstr = "5".into();
        } else if (2..=5).contains(&qu) {
            let val = cell.qnum;
            if val <= 0 {
                pstr = format!("{}0", qu - 1);
            } else if (1..16).contains(&val) {
                pstr = format!("{}{}", qu - 1, jsnum::to_radix(val as u32, 16));
            } else if (16..256).contains(&val) {
                pstr = format!("{}{}", qu + 4, jsnum::to_radix(val as u32, 16));
            }
            // pzprjs keeps the previous `pstr` for a clue of 256 or more, so
            // the cell is silently dropped; the port reproduces that.
        } else {
            pstr.clear();
            count += 1;
        }

        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 26 {
            cm.push_str(&jsnum::to_radix(9 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(9 + count, 36));
    }
    cm
}

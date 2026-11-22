//! Tatebo-Yokobo (pzprjs `variety/tateyoko.js`, pid `tateyoko`).
//!
//! A cell is a number (`qnum`, `?` is -2) or a shaded block (`ques` 1) with a
//! length 0–4; the answer layer marks each cell as part of a vertical (`qans`
//! 12) or horizontal (`qans` 13) bar.
//!
//! The URL is `decodeTateyoko`/`encodeTateyoko`: one hex digit per number, `.`
//! for `?`, `x` for an unnumbered block, `o`–`s` (base 29 - 24) for a numbered
//! block, `-` + two hex digits for 16–255, and `n` / `i` + one hex digit for
//! empty-cell runs. FileIO stores the question layer (`a`–`f` blocks, `?`,
//! numbers, `.`) and the answer layer (`0`/`1`/`2`). pzprjs
//! `BoardExec.adjustBoardData` swaps the vertical and horizontal bar direction
//! under turns (`tans` 12↔13).

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Tateyoko;

struct Tateyoko;

impl Genre for Tateyoko {
    fn pid(&self) -> &'static str {
        "tateyoko"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let chars: Vec<char> = body.chars().collect();
        let n = b.cells.len() as i64;
        let (mut c, mut i) = (0i64, 0usize);
        while i < chars.len() {
            let ca = chars[i];
            let Some(cell) = usize::try_from(c).ok().and_then(|c| b.cells.get_mut(c)) else {
                break;
            };
            match ca {
                '.' => cell.qnum = -2,
                'x' => cell.ques = 1,
                'o'..='s' => {
                    cell.ques = 1;
                    cell.qnum = jsnum::digit(ca, 29).unwrap() as i32 - 24;
                }
                '0'..='9' | 'a'..='f' => cell.qnum = jsnum::digit(ca, 16).unwrap() as i32,
                '-' => {
                    match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                        Some(v) => cell.qnum = v as i32,
                        None => d.push(
                            DiagnosticCode::UnexpectedChar,
                            "no two hexadecimal digits after '-' leave a clue empty",
                        ),
                    }
                    i += 2;
                }
                'i' => {
                    match chars.get(i + 1).and_then(|ca| jsnum::digit(*ca, 16)) {
                        Some(v) => c += v as i64 - 1,
                        None => {
                            // pzprjs would store NaN as the cell index and stop.
                            d.push(
                                DiagnosticCode::UnexpectedChar,
                                "no hexadecimal run length after 'i'",
                            );
                            i = chars.len();
                            break;
                        }
                    }
                    i += 1;
                }
                _ => {}
            }
            c += 1;
            if c < 0 || c >= n {
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
        chars.iter().skip(i + 1).collect()
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut cm = String::new();
        let mut count = 0u32;
        for cell in &b.cells {
            let (qu, qn) = (cell.ques, cell.qnum);
            let mut pstr = String::new();
            if qu == 0 {
                if qn == -1 {
                    count += 1;
                } else if qn == -2 {
                    pstr = ".".into();
                } else if qn < 16 {
                    pstr = to_radix_signed(qn, 16);
                } else if qn < 256 {
                    pstr = format!("-{}", jsnum::to_radix(qn as u32, 16));
                } else {
                    count += 1;
                }
            } else if qu == 1 {
                pstr = if qn >= 0 {
                    jsnum::to_radix(qn as u32 + 24, 29)
                } else {
                    "x".into()
                };
            }

            if count == 0 {
                cm.push_str(&pstr);
            } else if !pstr.is_empty() || count == 15 {
                if count == 1 {
                    cm.push('n');
                } else {
                    cm.push('i');
                    cm.push_str(&jsnum::to_radix(count, 16));
                }
                cm.push_str(&pstr);
                count = 0;
            }
        }
        match count {
            1 => cm.push('n'),
            n if n >= 2 => {
                cm.push('i');
                cm.push_str(&jsnum::to_radix(n, 16));
            }
            _ => {}
        }
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca.len() == 1 && ('a'..='f').contains(&ca.chars().next().unwrap()) {
                cell.ques = 1;
                cell.qnum = match ca {
                    "a" => 1,
                    "b" => 2,
                    "c" => 3,
                    "d" => 4,
                    "e" => 0,
                    _ => -1,
                };
            } else if ca == "?" {
                cell.qnum = -2;
            } else if ca != "." {
                cell.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        r.decode_cell(b, d, |cell, ca| {
            if ca == "1" {
                cell.qans = 12;
            } else if ca == "2" {
                cell.qans = 13;
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 1 {
                if cell.qnum == -1 || cell.qnum == -2 {
                    "f ".into()
                } else {
                    match cell.qnum {
                        0 => "e ".into(),
                        1 => "a ".into(),
                        2 => "b ".into(),
                        3 => "c ".into(),
                        4 => "d ".into(),
                        // pzprjs indexes its table with the raw value and would
                        // write `undefined`; validate reports the value instead.
                        _ => "f ".into(),
                    }
                }
            } else if cell.qnum == -2 {
                "? ".into()
            } else if cell.qnum >= 0 {
                format!("{} ", cell.qnum)
            } else {
                ". ".into()
            }
        });
        w.encode_cell(b, |cell| {
            if cell.ques != 1 {
                if cell.qans == 0 {
                    return "0 ".into();
                } else if cell.qans == 12 {
                    return "1 ".into();
                } else if cell.qans == 13 {
                    return "2 ".into();
                }
            }
            ". ".into()
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum`: 4 for a shaded block, `max(cols, rows)` otherwise;
        // `Cell.minnum` 0.
        let maxnum = b.cols().max(b.rows()) as i32;
        let mut out_of_range = false;
        let mut lossy = false;
        for cell in &b.cells {
            let cell_max = if cell.ques == 1 { 4 } else { maxnum };
            if !matches!(cell.ques, 0 | 1)
                || !(cell.qnum == -2
                    || cell.qnum == -1
                    || (0..=cell_max).contains(&cell.qnum))
                || !matches!(cell.qans, 0 | 12 | 13)
            {
                out_of_range = true;
            }
            if cell.ques == 0 && cell.qnum >= 256 {
                lossy = true;
            }
            if cell.ques == 1 && cell.qnum == -2 {
                lossy = true;
            }
        }
        if out_of_range {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tateyoko cell value outside number 0-maxnum / ? / block 0-4 or bar 0/12/13",
            );
        }
        if lossy {
            d.push(
                DiagnosticCode::LossyUrl,
                "tateyoko numbers of 256 or more, or a `?` block, are dropped from the URL",
            );
        }
    }

    /// `BoardExec.adjustBoardData` under `key & TURN`: `tans` 12↔13.
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        for op in ops(t) {
            if !matches!(op, Op::TurnR | Op::TurnL) {
                continue;
            }
            for cell in &mut b.cells {
                cell.qans = match cell.qans {
                    12 => 13,
                    13 => 12,
                    q => q,
                };
            }
        }
    }
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

/// `n.toString(radix)` for a possibly negative JavaScript number.
fn to_radix_signed(n: i32, radix: u32) -> String {
    if n < 0 {
        format!("-{}", jsnum::to_radix(n.unsigned_abs(), radix))
    } else {
        jsnum::to_radix(n as u32, radix)
    }
}

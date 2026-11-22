//! School Trip (pzprjs `variety/shugaku.js`, pid `shugaku`).
//!
//! Cells carry a futon number in `qnum` (`?` is -2); the answers are futons in
//! `qans` (1 shaded, 41–45 with pillow, 46–50 without) and aisle marks in
//! `qsub`. The URL is `decodeShugaku`/`encodeShugaku`; FileIO merges every
//! layer into one cell token.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Shugaku;

struct Shugaku;

impl Genre for Shugaku {
    fn pid(&self) -> &'static str {
        "shugaku"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_shugaku(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_shugaku(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "5" {
                cell.qnum = -2;
            } else if ca == "#" {
                cell.qans = 1;
            } else if ca == "-" {
                cell.qsub = 1;
            } else if ("a"..="j").contains(&ca) {
                // pzprjs compares the token as a string (`ca >= "a" && ca <= "j"`).
                cell.qans = jsnum::parse_int(ca, 20).unwrap() as i32 + 31;
            } else if ca != "." {
                cell.qnum = fileio::int(ca)?;
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.qnum >= 0 {
                format!("{} ", cell.qnum)
            } else if cell.qnum == -2 {
                "5 ".into()
            } else if cell.qans == 1 {
                "# ".into()
            } else if cell.qans >= 41 {
                format!("{} ", jsnum::to_radix((cell.qans - 31) as u32, 20))
            } else if cell.qsub == 1 {
                "- ".into()
            } else {
                ". ".into()
            }
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=4).contains(&c.qnum))
                || !(c.qans == 0 || c.qans == 1 || (41..=50).contains(&c.qans))
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "shugaku cell value outside number 0-4 / ? / futon 41-50 or aisle mark",
            );
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        apply_ops(t, |op| {
            for cell in &mut b.cells {
                cell.qans = rotate_qans(op, cell.qans);
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

/// `adjustBoardData`'s qans maps: the missing-border side of a pillow/cushion
/// follows the board operation; 41/46 (full futons) are unchanged.
fn rotate_qans(op: Transform, qans: i32) -> i32 {
    match op {
        Transform::FlipY => match qans {
            42 => 43,
            43 => 42,
            47 => 48,
            48 => 47,
            _ => qans,
        },
        Transform::FlipX => match qans {
            44 => 45,
            45 => 44,
            49 => 50,
            50 => 49,
            _ => qans,
        },
        Transform::Rot90 => match qans {
            42 => 45,
            45 => 43,
            43 => 44,
            44 => 42,
            47 => 50,
            50 => 48,
            48 => 49,
            49 => 47,
            _ => qans,
        },
        Transform::Rot270 => match qans {
            42 => 44,
            44 => 43,
            43 => 45,
            45 => 42,
            47 => 49,
            49 => 48,
            48 => 50,
            50 => 47,
            _ => qans,
        },
        _ => qans,
    }
}

/// `decodeShugaku`: numbers `0`–`4`, `5` for `?`, and `6`–`z` run lengths.
fn decode_shugaku(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let Some(cell) = b.cells.get_mut(c) else {
            break;
        };
        if ('0'..='4').contains(&ca) {
            cell.qnum = jsnum::digit(ca, 36).unwrap() as i32;
        } else if ca == '5' {
            cell.qnum = -2;
        } else {
            match jsnum::digit(ca, 36) {
                // pzprjs makes `c` NaN and leaves the loop for unknown characters.
                None => {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("unexpected character {ca:?} leaves the rest unread"),
                    );
                    break;
                }
                Some(skip) => c += skip as usize - 6,
            }
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
    chars.iter().skip(i + 1).collect()
}

/// `encodeShugaku`: one base-36 character per clue and run lengths from `6`.
fn encode_shugaku(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let val = cell.qnum;
        let pstr = if val == -2 {
            "5".into()
        } else if val != -1 {
            to_radix_signed(val, 36)
        } else {
            count += 1;
            String::new()
        };

        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 30 {
            cm.push_str(&jsnum::to_radix(5 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(5 + count, 36));
    }
    cm
}

/// `n.toString(radix)` for a possibly negative JavaScript number.
fn to_radix_signed(n: i32, radix: u32) -> String {
    if n < 0 {
        format!("-{}", jsnum::to_radix(n.unsigned_abs(), radix))
    } else {
        jsnum::to_radix(n as u32, radix)
    }
}

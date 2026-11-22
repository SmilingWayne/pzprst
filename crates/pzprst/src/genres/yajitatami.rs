//! Yajitatami (pzprjs `variety/yajitatami.js`, pid `yajitatami`).
//!
//! A clue is a direction (`qdir` 0–4) plus a number (`qnum`), or `+` for a
//! direction-less number (`qnum` -3, URL `+` / FileIO `#`), or `?` (`qnum` -2).
//! The URL is `decodeArrowNumber16` followed by the inner border-wall layer
//! (`encodeBorder_if_exist` writes that layer only when a wall exists). FileIO
//! stores `qdir,qnum` per cell, then border walls (`decodeBorderQues`) and
//! answer borders (`decodeBorderAns`). Rotations/reflections rotate `qdir`
//! (`adjustNumberArrow` in `BoardExec.adjustBoardData`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Yajitatami;

struct Yajitatami;

impl Genre for Yajitatami {
    fn pid(&self) -> &'static str {
        "yajitatami"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = decode_arrow_number16(b, body, d);
        if rest.is_empty() {
            // pzprjs skips `decodeBorder` when `outbstr` is empty; the shared
            // codec would report a TruncatedBody for the omitted wall layer.
            String::new()
        } else {
            encode::decode_border(b, &rest, d)
        }
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode_arrow_number16(b),
            encode_border_if_exist(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_cell_direc_qnum(r, b, d)?;
        fileio::decode_border_ques(r, b, d)?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_direc_qnum(w, b);
        fileio::encode_border_ques(w, b);
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // `adjustNumberArrow`: `trans[cell.qdir]` only rewrites a direction.
        // pzprjs runs it in `adjustBoardData`, before the coordinates move; the
        // map acts on the object's own value, so applying it to the transformed
        // board gives the same result (and the before-hook's writes are
        // discarded by `Board::transform`'s copy loop in this crate revision).
        for cell in &mut b.cells {
            cell.qdir = translate_dir(t, cell.qdir);
        }
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -3
                || c.qnum == -2
                || c.qnum == -1
                || (0..=999).contains(&c.qnum))
                || !(0..=4).contains(&c.qdir)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "yajitatami cell value outside clue 0-999/+/? or direction 0-4",
            );
        }
        if b.cells.iter().any(|c| {
            (c.qnum >= 4096) || (c.qdir != 0 && !(c.qnum >= 0 || c.qnum == -2))
        }) {
            d.push(
                DiagnosticCode::LossyUrl,
                "yajitatami numbers of 4096 or more, or a direction without a number, are dropped from the URL",
            );
        }
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.ques)
                || !(0..=1).contains(&e.qans)
                || !(0..=1).contains(&e.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "yajitatami border value outside wall/answer mark 0/1",
            );
        }
    }
}

/// `decodeArrowNumber16`: `+` is a direction-less number, `0`–`4` plus one hex
/// digit a direction (`.`) and number 0–15, `5`–`9` plus two hex digits numbers
/// 16–255, `-` plus one hex direction and three hex digits numbers 256–4095,
/// and `a`–`z` blank runs that skip `digit - 10` cells.
fn decode_arrow_number16(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let Some(cell) = b.cells.get_mut(c) else {
            break;
        };
        if ca == '+' {
            cell.qdir = 0;
            cell.qnum = -3;
        } else if ('0'..='4').contains(&ca) {
            cell.qdir = ca.to_digit(16).unwrap() as i32;
            match chars.get(i + 1).copied() {
                Some('.') => cell.qnum = -2,
                Some(ca1) if ca1.is_ascii_hexdigit() => {
                    cell.qnum = ca1.to_digit(16).unwrap() as i32;
                }
                _ => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "direction character is not followed by a hexadecimal number",
                ),
            }
            i += 1;
        } else if ('5'..='9').contains(&ca) {
            cell.qdir = ca.to_digit(16).unwrap() as i32 - 5;
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qn) => cell.qnum = qn as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal digits after a direction character",
                ),
            }
            i += 2;
        } else if ca == '-' {
            if let Some(dir) = jsnum::parse_int(&jsnum::substr(&chars, i + 1, 1), 16) {
                cell.qdir = dir as i32;
            } else {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal direction after '-'",
                );
            }
            if let Some(qn) = jsnum::parse_int(&jsnum::substr(&chars, i + 2, 3), 16) {
                cell.qnum = qn as i32;
            } else {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal number after a direction",
                );
            }
            i += 4;
        } else if ca.is_ascii_lowercase() {
            c += jsnum::digit(ca, 36).unwrap() as usize - 10;
        } else {
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

/// `encodeArrowNumber16`.
fn encode_arrow_number16(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let (dir, qn) = (cell.qdir, cell.qnum);
        let mut pstr = String::new();
        if qn == -2 {
            pstr = format!("{dir}.");
        } else if qn == -3 {
            pstr = "+".into();
        } else if (0..16).contains(&qn) {
            pstr = format!("{dir}{}", jsnum::to_radix(qn as u32, 16));
        } else if (16..256).contains(&qn) {
            pstr = format!("{}{}", dir + 5, jsnum::to_radix(qn as u32, 16));
        } else if (256..4096).contains(&qn) {
            pstr = format!("-{dir}{}", jsnum::to_radix(qn as u32, 16));
        } else {
            count += 1;
        }
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 26 {
            cm.push_str(&jsnum::to_radix(count + 9, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(count + 9, 36));
    }
    cm
}

/// `encodeBorder_if_exist`: write the wall layer only when some border is a wall.
fn encode_border_if_exist(b: &Board) -> String {
    if b.borders.iter().any(|e| e.ques == 1) {
        encode::encode_border(b)
    } else {
        String::new()
    }
}

/// `decodeCellDirecQnum`: `#` is a direction-less number, otherwise `dir,num`
/// with `-` for `?`.
fn decode_cell_direc_qnum(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "#" {
            cell.qdir = 0;
            cell.qnum = -3;
        } else if ca != "." {
            let mut inp = ca.split(',');
            let dir = inp.next().ok_or("empty direction")?;
            let num = inp.next().ok_or("missing ',' in a direction,number token")?;
            cell.qdir = if dir != "0" { fileio::int(dir)? } else { 0 };
            cell.qnum = if num == "-" { -2 } else { fileio::int(num)? };
        }
        Ok(())
    })
}

/// `encodeCellDirecQnum`.
fn encode_cell_direc_qnum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qnum == -3 {
            "# ".into()
        } else if cell.qnum != -1 {
            let ca1 = if cell.qdir != 0 {
                cell.qdir.to_string()
            } else {
                "0".into()
            };
            let ca2 = if cell.qnum != -2 {
                cell.qnum.to_string()
            } else {
                "-".into()
            };
            format!("{ca1},{ca2} ")
        } else {
            ". ".into()
        }
    });
}

/// pzprjs `getTranslateDir` for the composed `Transform`: direction 1 = up,
/// 2 = down, 3 = left, 4 = right; values outside 1–4 are left alone, as in
/// `trans[cell.qdir]`.
fn translate_dir(t: Transform, dir: i32) -> i32 {
    let map: &[(i32, i32)] = match t {
        Transform::Identity => return dir,
        Transform::Rot90 => &[(1, 4), (2, 3), (3, 1), (4, 2)],
        Transform::Rot180 => &[(1, 2), (2, 1), (3, 4), (4, 3)],
        Transform::Rot270 => &[(1, 3), (2, 4), (3, 2), (4, 1)],
        Transform::FlipX => &[(3, 4), (4, 3)],
        Transform::FlipY => &[(1, 2), (2, 1)],
        Transform::Transpose => &[(1, 3), (2, 4), (3, 1), (4, 2)],
        Transform::AntiTranspose => &[(1, 4), (2, 3), (3, 2), (4, 1)],
    };
    map.iter()
        .find(|(from, _)| *from == dir)
        .map_or(dir, |(_, to)| *to)
}

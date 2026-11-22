//! Wall Logic (pzprjs `variety/walllogic.js`, pid `walllogic`).
//!
//! `hasborder` 0: the puzzle is drawn with cell arrows (`anum`), not borders.
//! The URL is `decodeWallLogic` (`+` a wall cell, hex numbers, `-` plus two hex
//! digits for 16–255, `.` for `?`, `g`–`z` blank runs). FileIO stores `ques` and
//! `qnum` in one cell layer (`decodeQuesQnum`) and `anum`/`qcmp` in a second
//! (`decodeCellAnumcmp`). Rotations and reflections must rotate `anum`
//! (`adjustCellAnumArrow` in `BoardExec.adjustBoardData`).

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &WallLogic;

struct WallLogic;

impl Genre for WallLogic {
    fn pid(&self) -> &'static str {
        "walllogic"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_walllogic(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_walllogic(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_ques_qnum(r, b, d)?;
        decode_cell_anumcmp(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_ques_qnum(w, b);
        encode_cell_anumcmp(w, b);
        Ok(())
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // `adjustCellAnumArrow`: `trans[cell.anum]` only rewrites a direction.
        // pzprjs runs it in `adjustBoardData`, before the coordinates move; the
        // map acts on the object's own value, so applying it to the transformed
        // board gives the same result (and the before-hook's writes are
        // discarded by `Board::transform`'s copy loop in this crate revision).
        for cell in &mut b.cells {
            cell.anum = translate_dir(t, cell.anum);
        }
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum`: `board.cols + board.rows - 2`; `Cell.minnum` 0.
        let maxnum = (b.cols() + b.rows()) as i32 - 2;
        if b.cells.iter().any(|c| {
            !(c.ques == 0 || c.ques == 1)
                || !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
                || !matches!(c.anum, -1 | 1..=4)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!(
                    "walllogic cell value outside wall 0/1, clue 0-{maxnum}/?, arrow 1-4 or done mark 0/1"
                ),
            );
        }
        if b.cells
            .iter()
            .any(|c| (0..=maxnum).contains(&c.qnum) && c.qnum >= 256)
        {
            d.push(
                DiagnosticCode::LossyUrl,
                "walllogic numbers of 256 or more are dropped from the URL",
            );
        }
    }
}

/// `decodeWallLogic`.
fn decode_walllogic(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let Some(cell) = b.cells.get_mut(c) else {
            break;
        };
        if ca == '+' {
            cell.ques = 1;
        } else if ca.is_ascii_digit() || ('a'..='f').contains(&ca) {
            cell.qnum = ca.to_digit(16).unwrap() as i32;
        } else if ca == '-' {
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qn) => cell.qnum = qn as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal digits after '-' leave the cell empty",
                ),
            }
            i += 2;
        } else if ca == '.' {
            cell.qnum = -2;
        } else if ('g'..='z').contains(&ca) {
            c += jsnum::digit(ca, 36).unwrap() as usize - 16;
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

/// `encodeWallLogic`.
fn encode_walllogic(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let mut pstr = String::new();
        if cell.ques == 1 {
            pstr = "+".into();
        } else if cell.qnum == -1 {
            count += 1;
        } else if cell.qnum == -2 {
            pstr = ".".into();
        } else if cell.qnum < 16 {
            pstr = jsnum::to_radix(cell.qnum.unsigned_abs(), 16);
            if cell.qnum < 0 {
                pstr.insert(0, '-');
            }
        } else if cell.qnum < 256 {
            pstr = format!("-{}", jsnum::to_radix(cell.qnum as u32, 16));
        } else {
            count += 1;
        }
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 20 {
            cm.push_str(&jsnum::to_radix(15 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(15 + count, 36));
    }
    cm
}

/// `decodeQuesQnum`: `#` a wall cell, `-` for `?`, otherwise a clue number.
fn decode_ques_qnum(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "#" {
            cell.ques = 1;
        } else if ca == "-" {
            cell.qnum = -2;
        } else if ca != "." {
            cell.qnum = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodeQuesQnum`: `# ` then the clue, `- ` for `?`, `. ` for empty.
fn encode_ques_qnum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.ques == 1 {
            "# ".into()
        } else if cell.qnum >= 0 {
            format!("{} ", cell.qnum)
        } else if cell.qnum == -2 {
            "- ".into()
        } else {
            ". ".into()
        }
    });
}

/// `decodeCellAnumcmp`: `-` marks a cell done, otherwise the token is `anum`.
fn decode_cell_anumcmp(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "-" {
            cell.qcmp = 1;
        } else if ca != "." {
            cell.anum = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellAnumcmp`: the arrow, `- ` for a done mark, `. ` for empty.
fn encode_cell_anumcmp(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.anum != -1 {
            format!("{} ", cell.anum)
        } else if cell.qcmp == 1 {
            "- ".into()
        } else {
            ". ".into()
        }
    });
}

/// pzprjs `getTranslateDir` for the composed `Transform`: direction 1 = up,
/// 2 = down, 3 = left, 4 = right; values outside 1–4 are left alone, as in
/// `trans[cell.anum]`.
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

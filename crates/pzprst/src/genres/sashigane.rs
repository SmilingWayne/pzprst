//! Sashigane (pzprjs `variety/loute.js`, pid `sashigane`; `loute`, `sashikabe`
//! and `sashikazune` share the file). Cells carry an arrow direction in `qdir`
//! (1–4), a numbered circle in `qdir` 5 + `qnum`, or `?` as `qdir` -2; FileIO
//! adds the shading/background marks and the border answers. The URL is the
//! genre's own `decodeSashigane`/`encodeSashigane`.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Sashigane;

struct Sashigane;

impl Genre for Sashigane {
    fn pid(&self) -> &'static str {
        "sashigane"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_sashigane(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_sashigane(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca.starts_with('o') {
                cell.qdir = 5;
                let rest: String = ca.chars().skip(1).collect();
                if !rest.is_empty() {
                    cell.qnum = fileio::int(&rest)?;
                }
            } else if ca == "-" {
                cell.qdir = -2;
            } else if ca == "#" {
                cell.qans = 1;
            } else if ca == "_" {
                cell.qsub = 1;
            } else if ca != "." {
                cell.qdir = fileio::int(ca)?;
            }
            Ok(())
        })?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.qdir == 5 {
                let number = if cell.qnum != -1 {
                    cell.qnum.to_string()
                } else {
                    String::new()
                };
                format!("o{number} ")
            } else if cell.qdir == -2 {
                "- ".into()
            } else if cell.qdir != 0 {
                format!("{} ", cell.qdir)
            } else if cell.qsub != 0 {
                "_ ".into()
            } else if cell.qans != 0 {
                "# ".into()
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let mut out_of_range = false;
        let mut lossy = false;
        for (i, cell) in b.cells.iter().enumerate() {
            let (bx, by) = b.geom.cell_pos(i);
            if !matches!(cell.qdir, -2 | 0 | 1 | 2 | 3 | 4 | 5)
                || !(0..=1).contains(&cell.qans)
                || !(0..=1).contains(&cell.qsub)
            {
                out_of_range = true;
                continue;
            }
            if cell.qdir == 5 {
                // `Cell.maxnum`: distance to the far edge (plus one) in both axes.
                let cx = bx >> 1;
                let cy = by >> 1;
                let col = if bx < cols { cols - cx } else { cx + 1 };
                let row = if by < rows { rows - cy } else { cy + 1 };
                let maxnum = col + row - 1;
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
                "sashigane cell value outside arrow 1-4 / circle 5 / ? / clue 3-maxnum",
            );
        }
        if lossy {
            d.push(
                DiagnosticCode::LossyUrl,
                "sashigane clue numbers 256 or more are dropped from the URL",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "sashigane border value outside answer 0/1 or cross 0/1",
            );
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        apply_ops(t, |op| {
            for cell in &mut b.cells {
                cell.qdir = rotate_qdir(op, cell.qdir);
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

/// `adjustNumberArrow`'s `getTranslateDir` maps; other directions are unchanged.
fn rotate_qdir(op: Transform, qdir: i32) -> i32 {
    match op {
        Transform::FlipY => match qdir {
            1 => 2,
            2 => 1,
            _ => qdir,
        },
        Transform::FlipX => match qdir {
            3 => 4,
            4 => 3,
            _ => qdir,
        },
        Transform::Rot90 => match qdir {
            1 => 4,
            2 => 3,
            3 => 1,
            4 => 2,
            _ => qdir,
        },
        Transform::Rot270 => match qdir {
            1 => 3,
            2 => 4,
            3 => 2,
            4 => 1,
            _ => qdir,
        },
        _ => qdir,
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

/// `decodeSashigane`: arrows `g`–`j`, circles `0`–`f` or `-` + two hex digits,
/// `.` for an unnumbered circle, `@`/`%` for `?`, `k`–`z` run lengths.
fn decode_sashigane(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let Some(cell) = b.cells.get_mut(c) else {
            break;
        };
        if ca.is_ascii_digit() || ('a'..='f').contains(&ca) {
            cell.qdir = 5;
            cell.qnum = jsnum::digit(ca, 16).unwrap() as i32;
        } else if ca == '-' {
            cell.qdir = 5;
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qnum) => cell.qnum = qnum as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no two hex digits after '-' leave the circle unnumbered",
                ),
            }
            i += 2;
        } else if ca == '.' {
            cell.qdir = 5;
        } else if ca == '%' || ca == '@' {
            cell.qdir = -2;
        } else if ('g'..='j').contains(&ca) {
            cell.qdir = jsnum::digit(ca, 20).unwrap() as i32 - 15;
        } else if ('k'..='z').contains(&ca) {
            c += jsnum::digit(ca, 36).unwrap() as usize - 20;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} skips one cell"),
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
    chars.iter().skip(i + 1).collect()
}

/// `encodeSashigane`: one hex digit or `-` + two for circles, `@` for `?`,
/// base-20 `g`–`j` for arrows and `k`–`z` run lengths of empty cells.
fn encode_sashigane(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let dir = cell.qdir;
        let qn = cell.qnum;
        let pstr = if dir == 5 {
            if (0..16).contains(&qn) {
                jsnum::to_radix(qn as u32, 16)
            } else if (16..256).contains(&qn) {
                format!("-{}", jsnum::to_radix(qn as u32, 16))
            } else {
                ".".into()
            }
        } else if dir == -2 {
            "@".into()
        } else if dir != 0 {
            to_radix_signed(dir + 15, 20)
        } else {
            count += 1;
            String::new()
        };

        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 16 {
            cm.push_str(&jsnum::to_radix(count + 19, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(count + 19, 36));
    }
    cm
}

//! Compass (pzprjs `variety/compass.js`, pid `compass`).
//!
//! A compass cell (`ques` 51) carries one number per direction, stored in
//! `qnum` (right), `qnum2` (down), `qnum3` (left) and `qnum4` (up); -1 means the
//! direction has no number. Shaded cells are `ques` 7. The URL writes `_` for a
//! shaded cell, four `writeNumber16` numbers (`.` for an absent direction) per
//! compass cell, and base-36 run lengths for plain cells; FileIO writes `#`,
//! `qnum,qnum2,qnum3,qnum4` or `.` per cell and then the border answer layer.
//! `mukkonn` shares the file and is not ported.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;
use crate::Obj;

pub(crate) static GENRE: &dyn Genre = &Compass;

/// pzprjs `Position.UP` / `DN` / `LT` / `RT`.
const UP: i32 = 1;
const DN: i32 = 2;
const LT: i32 = 3;
const RT: i32 = 4;

struct Compass;

impl Genre for Compass {
    fn pid(&self) -> &'static str {
        "compass"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_compass(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_compass(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "#" {
                cell.ques = 7;
                return Ok(());
            }
            if ca == "." {
                return Ok(());
            }
            let mut inp = ca.split(',');
            let mut field = |name: &str| -> std::result::Result<i32, String> {
                let token = inp
                    .next()
                    .ok_or_else(|| format!("expected \"up,down,left,right\", found {ca:?} missing {name}"))?;
                fileio::int(token)
            };
            let qnum = field("right")?;
            let qnum2 = field("down")?;
            let qnum3 = field("left")?;
            let qnum4 = field("up")?;
            // `set51cell`: the compass cell owns all four numbers.
            cell.ques = 51;
            cell.qnum = qnum;
            cell.qnum2 = qnum2;
            cell.qnum3 = qnum3;
            cell.qnum4 = qnum4;
            Ok(())
        })?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 7 {
                "# ".into()
            } else if cell.ques == 51 {
                format!(
                    "{},{},{},{} ",
                    cell.qnum, cell.qnum2, cell.qnum3, cell.qnum4
                )
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum`: `cols * rows`; `Cell.minnum` 0.
        let maxnum = (b.cols() * b.rows()) as i32;
        let out_of_range = |q: i32| !(q == -1 || (0..=maxnum).contains(&q));
        if b.cells.iter().any(|c| {
            !matches!(c.ques, 0 | 7 | 51)
                || if c.ques == 51 {
                    out_of_range(c.qnum)
                        || out_of_range(c.qnum2)
                        || out_of_range(c.qnum3)
                        || out_of_range(c.qnum4)
                } else {
                    c.qnum != -1 || c.qnum2 != -1 || c.qnum3 != -1 || c.qnum4 != -1
                }
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("compass cell value outside shaded / empty / direction clue 0-{maxnum}"),
            );
        }
        // A direction number of -2 is legal FileIO data, but `encodePzpr` writes
        // it as `.` and the decoder turns that back into -1.
        if b.cells.iter().any(|c| {
            c.ques == 51 && [c.qnum, c.qnum2, c.qnum3, c.qnum4].contains(&-2)
        }) {
            d.push(
                DiagnosticCode::LossyUrl,
                "compass direction number -2 becomes an absent direction in a URL",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "compass border value outside line 0/1 or cross 0/1",
            );
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData2`: per-operation direction rotation.
        for op in turn_ops(t) {
            for cell in &mut b.cells {
                if cell.ques != 51 {
                    continue;
                }
                let (q1, q2, q3, q4) = (cell.qnum, cell.qnum2, cell.qnum3, cell.qnum4);
                match op {
                    Turn::FlipY => {
                        cell.qnum2 = q4;
                        cell.qnum4 = q2;
                    }
                    Turn::FlipX => {
                        cell.qnum = q3;
                        cell.qnum3 = q1;
                    }
                    Turn::TurnR => {
                        cell.qnum = q4;
                        cell.qnum2 = q1;
                        cell.qnum3 = q2;
                        cell.qnum4 = q3;
                    }
                    Turn::TurnL => {
                        cell.qnum = q2;
                        cell.qnum2 = q3;
                        cell.qnum3 = q4;
                        cell.qnum4 = q1;
                    }
                }
            }
        }
    }
}

/// `setQnumDir`: the property that holds the number of one direction.
fn set_qnum_dir(cell: &mut Obj, dir: i32, val: i32) {
    match dir {
        RT => cell.qnum = val,
        DN => cell.qnum2 = val,
        LT => cell.qnum3 = val,
        UP => cell.qnum4 = val,
        _ => {}
    }
}

/// `getQnumDir`: the number of one direction, -1 when absent.
fn get_qnum_dir(cell: &Obj, dir: i32) -> i32 {
    match dir {
        RT => cell.qnum,
        DN => cell.qnum2,
        LT => cell.qnum3,
        UP => cell.qnum4,
        _ => -1,
    }
}

/// `decodeCompass`: `_` shaded, four `readNumber16` direction numbers per
/// compass cell, base-36 run lengths for plain cells.
fn decode_compass(b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = body.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() && c < n {
        let ca = chars[i];
        if ca == '_' {
            b.cells[c].ques = 7;
            c += 1;
            i += 1;
            continue;
        }
        if ('g'..='z').contains(&ca) {
            c += jsnum::digit(ca, 36).unwrap() as usize - 15;
            i += 1;
            continue;
        }
        b.cells[c].ques = 51;
        for dir in UP..=RT {
            let (value, used) = encode::read_number16(&chars, i);
            match value {
                Some(-1) => {
                    // Not a number at all: consume the character and stop.
                    i += 1;
                    break;
                }
                Some(v) => {
                    // `.` (-2) means the direction has no number.
                    set_qnum_dir(&mut b.cells[c], dir, if v == -2 { -1 } else { v });
                    i += used;
                }
                None => {
                    // pzprjs stores NaN; pzprst keeps the absent default.
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        "incomplete hex escape in a compass direction number",
                    );
                    i += used;
                }
            }
        }
        c += 1;
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    // pzprjs never rewrites `outbstr` here; the unconsumed tail is what is left.
    chars[i.min(chars.len())..].iter().collect()
}

/// `encodeCompass`: the inverse of [`decode_compass`].
fn encode_compass(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let mut pstr = String::new();
        if cell.ques == 7 {
            pstr.push('_');
        } else if cell.ques == 51 {
            for dir in UP..=RT {
                let qn = get_qnum_dir(cell, dir);
                // An absent direction is a `.`; other negatives are dropped.
                pstr += &encode::write_number16(if qn == -1 { -2 } else { i64::from(qn) });
            }
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

#[derive(Clone, Copy)]
enum Turn {
    TurnR,
    TurnL,
    FlipX,
    FlipY,
}

/// The pzprjs board operations a pzprst transform is composed of, in order
/// (`rot90` = turnr, `transpose` = turnl + flipy, …).
fn turn_ops(t: Transform) -> &'static [Turn] {
    match t {
        Transform::Identity => &[],
        Transform::Rot90 => &[Turn::TurnR],
        Transform::Rot180 => &[Turn::TurnR, Turn::TurnR],
        Transform::Rot270 => &[Turn::TurnL],
        Transform::FlipX => &[Turn::FlipX],
        Transform::FlipY => &[Turn::FlipY],
        Transform::Transpose => &[Turn::TurnL, Turn::FlipY],
        Transform::AntiTranspose => &[Turn::TurnR, Turn::FlipY],
    }
}

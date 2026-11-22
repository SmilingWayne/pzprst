//! Tents (pzprjs `variety/tents.js`, pid `tents`).
//!
//! `hasborder` 1, `hasexcell` 1. The URL is the top/left number layer
//! (`decodeNumber16ExCell`) followed by one bit per cell for the trees
//! (`decode1Cell(1)`). FileIO stores the combined cell/excell layer
//! (`decodeCellExCell`): an excell holds a clue number with an optional `c`
//! (`qcmp`) prefix, a cell a tree (`qnum` 1), a tent (`anum` 2, token `A`) or
//! a dot (`qsub` 1, token `-`); the answer border layer follows
//! (`decodeBorderAns`).
//!
//! pzprjs's `adjustExCellTopLeft_1`/`_2` hooks redistribute the top and left
//! numbers under board transforms.

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;
use crate::Obj;

pub(crate) static GENRE: &dyn Genre = &Tents;

struct Tents;

impl Genre for Tents {
    fn pid(&self) -> &'static str {
        "tents"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hasexcell(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_number16_excell(b, body, d);
        decode_1cell(b, &rest, d, 1)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_number16_excell(b);
        body.push_str(&encode_1cell(b, 1));
        Ok(body)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_excell(r, b, d, |obj, group, ca| {
            if ca == "." {
                return Ok(());
            }
            if group == Group::ExCell {
                let mut token = ca;
                if let Some(rest) = token.strip_prefix('c') {
                    obj.qcmp = 1;
                    token = rest;
                }
                obj.qnum = fileio::int(token)?;
            } else if ca == "A" {
                obj.anum = 2;
            } else if ca == "-" {
                obj.qsub = 1;
            } else if let Some(v) = jsnum::integer(ca) {
                // pzprjs tests `+ca > 0`; other tokens are left alone.
                if v > 0 {
                    obj.qnum = v;
                }
            }
            Ok(())
        })?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_excell(w, b, |obj, group| {
            if group == Group::ExCell {
                if obj.qnum != -1 {
                    let cmp = if obj.qcmp != 0 { "c" } else { "" };
                    return format!("{cmp}{} ", obj.qnum);
                }
            } else if obj.qnum != -1 {
                return format!("{} ", obj.qnum);
            } else if obj.anum == 2 {
                return "A ".into();
            } else if obj.qsub == 1 {
                return "- ".into();
            }
            ". ".into()
        });
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell@tents`: `maxnum` 1 in edit mode, 3 in play mode; answers are
        // `anum` 2 (tent) or the dot mark `qsub` 1.
        if b.cells
            .iter()
            .any(|c| !(-1..=3).contains(&c.qnum) || !matches!(c.anum, -1 | 2) || !(0..=1).contains(&c.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tents cell value outside tree 0-3, tent anum 2 or dot 0/1",
            );
        }
        // `ExCell@tents`: the top excells allow `rows`, the left ones `cols`.
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let bad = b.excells.iter().enumerate().any(|(i, ex)| {
            let max = match b.geom.excell_pos(i) {
                Some((_, -1)) => rows,
                Some(_) => cols,
                None => return true,
            };
            !(-2..=max).contains(&ex.qnum) || !(0..=1).contains(&ex.qcmp)
        });
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tents excell number outside empty / ? / 0-maxnum or qcmp 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tents border answer mark outside 0/1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`/`adjustBoardData2` are called once
    /// per board operation; pzprst calls the hooks once for the composed
    /// transform, so the whole replay happens here, before the coordinates
    /// move (the values are placed in pre-transform object order).
    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        adjust_excell_top_left(t, b, false);
    }
}

/// `decode1Cell` (variety-common/Encode.js): run-length markers for one
/// `qnum` value.
fn decode_1cell(b: &mut Board, bstr: &str, d: &mut Diagnostics, val: i32) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let Some(v) = jsnum::digit(ca, 36) else {
            // pzprjs sets the current cell for characters in "0".."h" before
            // parseInt returns NaN and the loop breaks.
            if ('0'..='h').contains(&ca) {
                if let Some(cell) = b.cells.get_mut(c) {
                    cell.qnum = val;
                }
            }
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} ends the marker layer"),
            );
            break;
        };
        if ('0'..='h').contains(&ca) {
            if let Some(cell) = b.cells.get_mut(c) {
                cell.qnum = val;
            }
            c += v as usize;
        } else {
            c += v as usize - 18;
        }
        c += 1;
        i += 1;
        if c >= n {
            break;
        }
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    chars[i.min(chars.len())..].iter().collect()
}

/// `encode1Cell` (variety-common/Encode.js).
fn encode_1cell(b: &Board, val: i32) -> String {
    let mut count: i64 = -1;
    let mut have = false;
    let mut cm = String::new();
    for cell in &b.cells {
        if cell.qnum == val {
            if count >= 0 {
                if have {
                    cm.push_str(&jsnum::to_radix(count as u32, 36));
                } else {
                    cm.push_str(&jsnum::to_radix((count + 18) as u32, 36));
                }
            }
            have = true;
            count = 0;
            continue;
        }
        count += 1;
        if count == 17 {
            if have {
                cm.push_str(&jsnum::to_radix(count as u32, 36));
            } else {
                cm.push_str(&jsnum::to_radix((count + 18) as u32, 36));
            }
            have = false;
            count = -1;
        }
    }
    if count >= 0 {
        if have {
            cm.push_str(&jsnum::to_radix(count as u32, 36));
        } else {
            cm.push_str(&jsnum::to_radix((count + 18) as u32, 36));
        }
    }
    cm
}

/// pzprjs `adjustExCellTopLeft_1`/`_2` for `hasexcell` 1: the top row and the
/// left column of excells are captured before each operation and written back
/// after it. `preserve` is `adjustExCellTopLeft_2`'s third argument (magnets
/// passes `true`). The replay yields the value of every post-transform
/// position; those values are stored in the pre-transform objects so that
/// `Board::transform`'s `turnflipGroup("excell", …)` puts them in place.
fn adjust_excell_top_left(t: Transform, b: &mut Board, preserve: bool) {
    if b.geom.hasexcell != 1 || b.excells.is_empty() {
        return;
    }
    let xr = b.geom.excell_rows;
    let xc = b.geom.excell_cols;
    // The pzprjs pop() loops pair the top vectors (excellRows) with the left
    // positions (excellCols); every hasexcell-1 genre has the same count.
    if xr != xc {
        return;
    }
    let (pre_c, pre_r) = (b.cols(), b.rows());
    if b.excells.len() != pre_c * xr + pre_r * xc {
        return;
    }
    let (post_c, post_r) = if t.swaps_axes() {
        (pre_r, pre_c)
    } else {
        (pre_c, pre_r)
    };
    // Values in pzprjs's capture order (far from the grid first, near last).
    let mut top: Vec<Vec<i32>> = (0..pre_c)
        .map(|c| (0..xr).rev().map(|j| b.excells[c * xr + j].qnum).collect())
        .collect();
    let mut left: Vec<Vec<i32>> = (0..pre_r)
        .map(|r| {
            (0..xc)
                .rev()
                .map(|i| b.excells[pre_c * xr + r * xc + i].qnum)
                .collect()
        })
        .collect();
    for &op in operations(t) {
        let mut a = left.clone();
        let mut h = top.clone();
        if !preserve {
            match op {
                Transform::FlipX | Transform::Rot270 => {
                    for v in &mut a {
                        v.reverse();
                    }
                }
                Transform::FlipY | Transform::Rot90 => {
                    for v in &mut h {
                        v.reverse();
                    }
                }
                _ => {}
            }
        }
        match op {
            Transform::FlipX => {
                top.reverse();
                left = a;
            }
            Transform::FlipY => {
                left.reverse();
                top = h;
            }
            Transform::Rot90 => {
                top = a;
                top.reverse();
                left = h;
            }
            Transform::Rot270 => {
                top = a;
                left = h;
                left.reverse();
            }
            _ => {}
        }
    }
    if top.len() != post_c || left.len() != post_r {
        return;
    }
    let mut final_vals = vec![-1i32; post_c * xr + post_r * xc];
    for (c, vals) in top.iter().enumerate() {
        for (j, &v) in vals.iter().enumerate() {
            final_vals[c * xr + (xr - 1 - j)] = v;
        }
    }
    for (r, vals) in left.iter().enumerate() {
        for (i, &v) in vals.iter().enumerate() {
            final_vals[post_c * xr + r * xc + (xc - 1 - i)] = v;
        }
    }
    let markers: Vec<Obj> = (0..b.excells.len())
        .map(|i| Obj {
            qnum: i as i32,
            ..Obj::default()
        })
        .collect();
    let moved = b.move_excells(t, &markers);
    for (post_i, marker) in moved.iter().enumerate() {
        let pre_i = marker.qnum as usize;
        if pre_i < b.excells.len() && post_i < final_vals.len() {
            b.excells[pre_i].qnum = final_vals[post_i];
        }
    }
}

/// The pzprjs board operations a `Transform` stands for, in
/// `tools/gen_fixtures.js` order.
fn operations(t: Transform) -> &'static [Transform] {
    match t {
        Transform::Identity => &[],
        Transform::Rot90 => &[Transform::Rot90],
        Transform::Rot180 => &[Transform::Rot90, Transform::Rot90],
        Transform::Rot270 => &[Transform::Rot270],
        Transform::FlipX => &[Transform::FlipX],
        Transform::FlipY => &[Transform::FlipY],
        Transform::Transpose => &[Transform::Rot270, Transform::FlipY],
        Transform::AntiTranspose => &[Transform::Rot90, Transform::FlipY],
    }
}

//! Box (pzprjs `variety/box.js`, pid `box`).
//!
//! `hasborder` 0, `hasexcell` 1. The URL is the custom `decodeBox` layer: one
//! base-32 digit per top/left clue (`-` plus two digits for numbers 32 and
//! above), in id order. A clue is the sum of a row/column and empty is `0`
//! (pzprjs `ExCell.qnum` defaults to 0 and `setQnum` clamps at 0), so pzprst
//! also stores `0` for an empty real excell and `-1` only for the missing
//! top-left corner object. FileIO stores the combined cell/excell layer
//! (`decodeCellExCell`): an excell is its number (`0` when empty), a cell only
//! answer marks (`#` shading, `+` sub mark).
//!
//! pzprjs's `adjustExCellTopLeft_1`/`_2` hooks redistribute the top and left
//! numbers under board transforms.

use super::Genre;
use crate::board::{Board, Group};
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;
use crate::Obj;

pub(crate) static GENRE: &dyn Genre = &BoxGenre;

struct BoxGenre;

impl Genre for BoxGenre {
    fn pid(&self) -> &'static str {
        "box"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn hasexcell(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        // pzprjs `ExCell.qnum` defaults to 0, so excells past the body stay 0.
        for obj in &mut b.excells {
            obj.qnum = 0;
        }
        let chars: Vec<char> = body.chars().collect();
        let n = b.cols() + b.rows();
        let (mut a, mut ec) = (0usize, 0usize);
        while a < chars.len() && ec < n {
            let ca = chars[a];
            let (val, used) = if ca == '-' {
                match jsnum::parse_int(&jsnum::substr(&chars, a + 1, 2), 32) {
                    Some(v) => (v as i32, 3),
                    None => {
                        // pzprjs stores JavaScript's NaN here; keep the 0 default.
                        d.push(
                            DiagnosticCode::UnexpectedChar,
                            format!("incomplete base-32 escape at offset {a} leaves excell {ec} empty"),
                        );
                        (0, 3)
                    }
                }
            } else {
                match jsnum::digit(ca, 32) {
                    Some(v) => (v as i32, 1),
                    None => {
                        d.push(
                            DiagnosticCode::UnexpectedChar,
                            format!("unexpected character {ca:?} leaves excell {ec} empty"),
                        );
                        (0, 1)
                    }
                }
            };
            b.excells[ec].qnum = val;
            ec += 1;
            a += used;
        }
        if ec < n {
            d.push(
                DiagnosticCode::TruncatedBody,
                "body ends before the last excell",
            );
        }
        chars[a.min(chars.len())..].iter().collect()
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut cm = String::new();
        for obj in b.excells.iter().take(b.cols() + b.rows()) {
            let qnum = obj.qnum.max(0) as u32;
            if qnum < 32 {
                cm.push_str(&jsnum::to_radix(qnum, 32));
            } else {
                cm.push('-');
                cm.push_str(&jsnum::to_radix(qnum, 32));
            }
        }
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_excell(r, b, d, |obj, group, ca| {
            if ca == "." {
                // An empty real excell is pzprjs's default 0; the missing
                // top-left corner keeps pzprst's -1 and encodes as ".".
                if group == Group::ExCell {
                    obj.qnum = 0;
                }
            } else if group == Group::ExCell {
                obj.qnum = fileio::int(ca)?;
            } else if ca == "#" {
                obj.qans = 1;
            } else if ca == "+" {
                obj.qsub = 1;
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_excell(w, b, |obj, group| {
            if group == Group::ExCell {
                if obj.qnum >= 0 {
                    return format!("{} ", obj.qnum);
                }
            } else if obj.qans == 1 {
                return "# ".into();
            } else if obj.qsub == 1 {
                return "+ ".into();
            }
            ". ".into()
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| c.qnum != -1 || !(0..=1).contains(&c.qans) || !(0..=1).contains(&c.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "box cell value outside empty or answer mark 0/1",
            );
        }
        // `ExCell@box.maxnum`: a top clue allows `rows*(rows+1)/2`, a left clue
        // `cols*(cols+1)/2`.
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let bad = b.excells.iter().enumerate().any(|(i, ex)| {
            let max = match b.geom.excell_pos(i) {
                Some((-1, _)) => cols * (cols + 1) / 2,
                Some(_) => rows * (rows + 1) / 2,
                None => return true,
            };
            !(-1..=max).contains(&ex.qnum)
        });
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "box excell number outside 0-maxnum",
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

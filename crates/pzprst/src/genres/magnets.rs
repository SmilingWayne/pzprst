//! Magnets (pzprjs `variety/magnets.js`, pid `magnets`).
//!
//! `hasborder` 1, `hasexcell` 1 with two clue rows/columns (`excellRows` and
//! `excellCols` are 2: the `+` count line is farther out than the `-` line).
//! The URL is the excell number layer (`decodeNumber16ExCell`), the border
//! wall layer (`decodeBorder`), then — only when some cell has question data —
//! the circle layer (`decodeCircle`) and the shaded-invalid-cell layer
//! (`decodeEmpty`). FileIO stores a config flag (`a`), the border wall layer
//! (`decodeBorderQues`) and the combined cell/excell layer
//! (`decodeCellExCell`): excells are a clue with an optional `c` (`qcmp`)
//! prefix, cells are `#` (invalid/shaded), `q<n>` (pole), `s<n>` (sub mark) or
//! `a<n>` (answer mark).
//!
//! pzprjs calls `adjustExCellTopLeft_2(key, d, true)` here, so the per-line
//! value order is preserved under transforms.

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::fileio::int;
use crate::transform::Transform;
use crate::Obj;

pub(crate) static GENRE: &dyn Genre = &Magnets;

struct Magnets;

impl Genre for Magnets {
    fn pid(&self) -> &'static str {
        "magnets"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hasexcell(&self) -> u8 {
        1
    }

    fn excell_rows(&self, _cols: usize, _rows: usize) -> usize {
        2
    }

    fn excell_cols(&self, _cols: usize, _rows: usize) -> usize {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('a', "magnets_anti")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_number16_excell(b, body, d);
        let rest = encode::decode_border(b, &rest, d);
        let rest = encode::decode_circle(b, &rest, d);
        encode::decode_empty(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_number16_excell(b);
        body.push_str(&encode::encode_border(b));
        // pzprjs writes both answer layers only when some cell has question
        // data (`qnum !== -1` or an invalid `ques === 7` cell).
        if b.cells.iter().any(|c| c.qnum != -1 || c.ques == 7) {
            body.push_str(&encode::encode_circle(b)?);
            body.push_str(&encode::encode_empty(b));
        }
        Ok(body)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "a", "magnets_anti");
        fileio::decode_border_ques(r, b, d)?;
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
                obj.qnum = int(token)?;
            } else if ca == "#" {
                obj.ques = 7;
            } else if let Some(rest) = ca.strip_prefix('q') {
                obj.qnum = int(rest)?;
            } else if let Some(rest) = ca.strip_prefix('s') {
                obj.qsub = int(rest)?;
            } else if let Some(rest) = ca.strip_prefix('a') {
                obj.anum = int(rest)?;
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "a", "magnets_anti");
        fileio::encode_border_ques(w, b);
        fileio::encode_cell_excell(w, b, |obj, group| {
            if group == Group::ExCell {
                if obj.qnum != -1 {
                    let cmp = if obj.qcmp != 0 { "c" } else { "" };
                    return format!("{cmp}{} ", obj.qnum);
                }
            } else if obj.ques == 7 {
                return "# ".into();
            } else if obj.qnum != -1 {
                return format!("q{} ", obj.qnum);
            } else if obj.qsub != 0 {
                return format!("s{} ", obj.qsub);
            } else if obj.anum != -1 {
                return format!("a{} ", obj.anum);
            }
            ". ".into()
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(-1..=2).contains(&c.qnum)
                || !matches!(c.ques, 0 | 7)
                || !(0..=3).contains(&c.qsub)
                || !(-1..=3).contains(&c.anum)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "magnets cell value outside pole 0-2, shaded 0/7 or answer marks 0-3",
            );
        }
        // `ExCell@magnets.maxnum`: the top clue allows `(rows+1)/2`, the left
        // one `(cols+1)/2`.
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let bad = b.excells.iter().enumerate().any(|(i, ex)| {
            let max = match b.geom.excell_pos(i) {
                Some((_, by)) if by < 0 => (rows + 1) / 2,
                Some(_) => (cols + 1) / 2,
                None => return true,
            };
            !(-2..=max).contains(&ex.qnum) || !(0..=1).contains(&ex.qcmp)
        });
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "magnets excell number outside empty / ? / 0-maxnum or qcmp 0/1",
            );
        }
        if b.borders.iter().any(|e| !(0..=1).contains(&e.ques)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "magnets border wall outside 0/1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`/`adjustBoardData2` are called once
    /// per board operation; pzprst calls the hooks once for the composed
    /// transform, so the whole replay happens here, before the coordinates
    /// move (the values are placed in pre-transform object order).
    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        adjust_excell_top_left(t, b, true);
    }
}

/// pzprjs `adjustExCellTopLeft_1`/`_2` for `hasexcell` 1: the top row and the
/// left column of excells are captured before each operation and written back
/// after it. `preserve` is `adjustExCellTopLeft_2`'s third argument (magnets
/// passes `true`, so the per-line value order is not reversed). The replay
/// yields the value of every post-transform position; those values are stored
/// in the pre-transform objects so that `Board::transform`'s
/// `turnflipGroup("excell", …)` puts them in place.
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

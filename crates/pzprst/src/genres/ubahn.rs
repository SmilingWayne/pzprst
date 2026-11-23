//! U-Bahn (pzprjs `variety/ubahn.js`, pid `ubahn`).
//!
//! The URL is the `decodeNumber16ExCell` outer-clue layer followed by the
//! `decodeEmpty` shaded-cell layer. FileIO stores one combined cell/excell layer
//! (`decodeCellExCell`): an excell is its clue number with an optional `c`
//! (`qcmp`) prefix, a shaded cell is `#`, everything else is `.`; the border
//! line layer follows.
//!
//! `hasexcell` 1 allocates a 4x4 top-left band of excells (`excellRows` and
//! `excellCols` are 4). pzprjs redistributes the band under board transforms
//! with `adjustExCellTopLeft_1`/`_2`, whose `preserve` flag is set here.

use std::collections::BTreeMap;

use super::Genre;
use crate::board::{Board, Group, Obj};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Ubahn;

struct Ubahn;

/// pzprjs `Board.excellRows` / `Board.excellCols` for ubahn.
const EXROWS: i32 = 4;
const EXCOLS: i32 = 4;

impl Genre for Ubahn {
    fn pid(&self) -> &'static str {
        "ubahn"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn hasexcell(&self) -> u8 {
        1
    }

    fn excell_rows(&self, _cols: usize, _rows: usize) -> usize {
        EXROWS as usize
    }

    fn excell_cols(&self, _cols: usize, _rows: usize) -> usize {
        EXCOLS as usize
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_number16_excell(b, body, d);
        encode::decode_empty(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_number16_excell(b);
        body.push_str(&encode::encode_empty(b));
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
            } else if ca == "#" {
                obj.ques = 7;
            }
            // Every other cell token is a `snum` array in pzprjs; `Obj` has no
            // `snum`, so those tokens set nothing here.
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_excell(w, b, |obj, group| {
            if group == Group::ExCell {
                if obj.qnum != -1 {
                    let cmp = if obj.qcmp != 0 { "c" } else { "" };
                    return format!("{cmp}{} ", obj.qnum);
                }
            } else if obj.ques != 7 {
                return ". ".into();
            } else {
                return "# ".into();
            }
            ". ".into()
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `ExCell.maxnum`: the top band allows `rows`, the left band `cols`;
        // `ExCell.minnum` is 0 and `disInputHatena` only affects input.
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let bad = b.excells.iter().enumerate().any(|(i, ex)| {
            let (bx, by) = b.geom.excell_pos(i).unwrap_or((0, 0));
            let max = if bx < 0 && by < 0 {
                0
            } else if by < 0 {
                rows
            } else {
                cols
            };
            !(ex.qnum == -1 || ex.qnum == -2 || (0..=max).contains(&ex.qnum))
                || !(0..=1).contains(&ex.qcmp)
        });
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "ubahn excell value outside empty / ? / 0-maxnum or qcmp 0/1",
            );
        }
        if b.cells.iter().any(|c| c.ques != 0 && c.ques != 7) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "ubahn cell value outside empty / shaded",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "ubahn border value outside line 0/1 or cross mark",
            );
        }
    }

    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        adjust_excells(t, b, true);
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

/// pzprjs `setposExCells` for `hasexcell` 1: the top band (top to bottom) then
/// the left band (left to right), no corner object.
fn positions(cols: i32, rows: i32) -> Vec<(i32, i32)> {
    let qc = cols * EXROWS;
    let qr = rows * EXCOLS;
    let mut out = Vec::with_capacity((qc + qr) as usize);
    for i in 0..qc {
        out.push(((i / EXROWS) * 2 + 1, (i % EXROWS) * -2 - 1));
    }
    for j in 0..qr {
        out.push(((j % EXCOLS) * -2 - 1, (j / EXCOLS) * 2 + 1));
    }
    out
}

fn index(pos: &[(i32, i32)], bx: i32, by: i32) -> Option<usize> {
    pos.iter().position(|&p| p == (bx, by))
}

/// Reverse the whole top band, which is what pzprst's `turnflipGroup`
/// (`hasexcell` 1) does for `FlipX`; the same map inverts it.
fn reverse_top_row(vals: &mut [Obj], pos: &[(i32, i32)], cols: i32) {
    let src = vals.to_vec();
    for (i, &(bx, by)) in pos.iter().enumerate() {
        if by < 0 {
            if let Some(j) = index(pos, 2 * cols - bx, by) {
                vals[j] = src[i].clone();
            }
        }
    }
}

/// Reverse the whole left band, pzprst `turnflipGroup`'s `FlipY`.
fn reverse_left_col(vals: &mut [Obj], pos: &[(i32, i32)], rows: i32) {
    let src = vals.to_vec();
    for (i, &(bx, by)) in pos.iter().enumerate() {
        if bx < 0 {
            if let Some(j) = index(pos, bx, 2 * rows - by) {
                vals[j] = src[i].clone();
            }
        }
    }
}

fn capture_left(vals: &[Obj], pos: &[(i32, i32)], rows: i32) -> BTreeMap<i32, Vec<i32>> {
    let mut out = BTreeMap::new();
    for by in (1..=2 * rows - 1).step_by(2) {
        let mut list = Vec::with_capacity(EXCOLS as usize);
        for k in 0..EXCOLS {
            let bx = -1 - 2 * (EXCOLS - 1) + 2 * k;
            let i = index(pos, bx, by).expect("left band position");
            list.push(vals[i].qnum);
        }
        out.insert(by, list);
    }
    out
}

fn capture_top(vals: &[Obj], pos: &[(i32, i32)], cols: i32) -> BTreeMap<i32, Vec<i32>> {
    let mut out = BTreeMap::new();
    for bx in (1..=2 * cols - 1).step_by(2) {
        let mut list = Vec::with_capacity(EXROWS as usize);
        for k in 0..EXROWS {
            let by = -1 - 2 * (EXROWS - 1) + 2 * k;
            let i = index(pos, bx, by).expect("top band position");
            list.push(vals[i].qnum);
        }
        out.insert(bx, list);
    }
    out
}

fn set_qnum(vals: &mut [Obj], pos: &[(i32, i32)], bx: i32, by: i32, qnum: i32) {
    if let Some(i) = index(pos, bx, by) {
        vals[i].qnum = qnum;
    }
}

fn pop(map: &mut BTreeMap<i32, Vec<i32>>, key: i32) -> i32 {
    map.get_mut(&key)
        .and_then(Vec::pop)
        .expect("captured band value")
}

/// pzprjs `BoardExec.adjustExCellTopLeft_1`/`_2` for `hasexcell` 1.
///
/// pzprst calls the hooks once per composed transform and has already moved the
/// excells with its own `turnflipGroup`; this undoes that layout, then replays
/// the pzprjs capture / turnflip / rewrite sequence one operation at a time.
/// `preserve` is ubahn's `adjustExCellTopLeft_2(key, d, true)` argument.
fn adjust_excells(t: Transform, b: &mut Board, preserve: bool) {
    if b.geom.hasexcell != 1 || b.excells.is_empty() {
        return;
    }
    let (mut cols, mut rows) = (b.cols() as i32, b.rows() as i32);
    let mut vals = b.excells.clone();

    // Undo pzprst's `turnflipGroup`: turns only swap the dimensions, flips
    // reverse their own band (with the default one-row band only `by == -1` /
    // `bx == -1` moves).
    for &op in operations(t).iter().rev() {
        let pos = positions(cols, rows);
        match op {
            Transform::Rot90 | Transform::Rot270 => std::mem::swap(&mut cols, &mut rows),
            Transform::FlipX => reverse_top_row(&mut vals, &pos, cols),
            Transform::FlipY => reverse_left_col(&mut vals, &pos, rows),
            _ => {}
        }
    }

    for &op in operations(t) {
        let pos = positions(cols, rows);
        // adjustExCellTopLeft_1: capture the top and left bands.
        let mut qnumw = capture_left(&vals, &pos, rows);
        let mut qnumh = capture_top(&vals, &pos, cols);

        // turnflipGroup("excell", …): a flip mirrors its whole band (all rows
        // of the top band for FlipX, all columns of the left band for FlipY).
        let mut next = vals.clone();
        match op {
            Transform::FlipX => {
                for (i, &(bx, by)) in pos.iter().enumerate() {
                    if by < 0 {
                        if let Some(j) = index(&pos, 2 * cols - bx, by) {
                            next[j] = vals[i].clone();
                        }
                    }
                }
            }
            Transform::FlipY => {
                for (i, &(bx, by)) in pos.iter().enumerate() {
                    if bx < 0 {
                        if let Some(j) = index(&pos, bx, 2 * rows - by) {
                            next[j] = vals[i].clone();
                        }
                    }
                }
            }
            _ => {}
        }
        vals = next;
        if op.swaps_axes() {
            std::mem::swap(&mut cols, &mut rows);
        }

        // adjustExCellTopLeft_2.
        let pos = positions(cols, rows);
        if !preserve {
            match op {
                Transform::FlipX | Transform::Rot270 => {
                    for list in qnumw.values_mut() {
                        list.reverse();
                    }
                }
                Transform::FlipY | Transform::Rot90 => {
                    for list in qnumh.values_mut() {
                        list.reverse();
                    }
                }
                _ => {}
            }
        }
        match op {
            Transform::FlipY => {
                for bx in (1..=2 * cols - 1).step_by(2) {
                    for k in 0..EXROWS {
                        let by = -1 - 2 * k;
                        set_qnum(&mut vals, &pos, bx, by, pop(&mut qnumh, bx));
                    }
                }
            }
            Transform::FlipX => {
                for by in (1..=2 * rows - 1).step_by(2) {
                    for k in 0..EXCOLS {
                        let bx = -1 - 2 * k;
                        set_qnum(&mut vals, &pos, bx, by, pop(&mut qnumw, by));
                    }
                }
            }
            Transform::Rot90 => {
                for by in (1..=2 * rows - 1).step_by(2) {
                    for k in 0..EXCOLS {
                        let bx = -1 - 2 * k;
                        set_qnum(&mut vals, &pos, bx, by, pop(&mut qnumh, by));
                    }
                }
                for bx in (1..=2 * cols - 1).step_by(2) {
                    for k in 0..EXROWS {
                        let by = -1 - 2 * k;
                        set_qnum(&mut vals, &pos, bx, by, pop(&mut qnumw, 2 * cols - bx));
                    }
                }
            }
            Transform::Rot270 => {
                for by in (1..=2 * rows - 1).step_by(2) {
                    for k in 0..EXCOLS {
                        let bx = -1 - 2 * k;
                        set_qnum(&mut vals, &pos, bx, by, pop(&mut qnumh, 2 * rows - by));
                    }
                }
                for bx in (1..=2 * cols - 1).step_by(2) {
                    for k in 0..EXROWS {
                        let by = -1 - 2 * k;
                        set_qnum(&mut vals, &pos, bx, by, pop(&mut qnumw, bx));
                    }
                }
            }
            _ => {}
        }
    }

    // Write the final layout back in pzprst's final dimensions.
    let pos = positions(cols, rows);
    for (i, &(bx, by)) in pos.iter().enumerate() {
        if let Some(j) = b.geom.excell_index(bx, by) {
            b.excells[j] = vals[i].clone();
        }
    }
}

//! Kakuro (pzprjs `variety/kakuro.js`, pid `kakuro`).
//!
//! A `[＼]` cell (`ques` 51) stores its across clue in `qnum` and its down
//! clue in `qnum2`; a top outside clue is an excell's `qnum2` and a left one
//! its `qnum`. The URL is `decodeKakuro`: a run-length coded cell layer (`.` is
//! an empty `[＼]` cell) followed by the outside clues. FileIO uses the shared
//! `decodeCellQnum51` / `encodeCellQnum51` layer plus `decodeCellAnumsub`.
//! Board transforms replay pzprjs's `BoardExec.adjustQues51_1`/`_2` pair.

use super::Genre;
use crate::board::{Board, Group};
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Kakuro;

struct Kakuro;

impl Genre for Kakuro {
    fn pid(&self) -> &'static str {
        "kakuro"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hasexcell(&self) -> u8 {
        1
    }

    /// pzprjs's `ExCell.ques` is 51, so `Board.emptyexcell` is the top-left
    /// corner excell.
    fn excell_corner(&self) -> bool {
        true
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_kakuro(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_kakuro(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum51(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum51(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let mut bad = false;
        let mut lossy = false;
        for cell in &b.cells {
            if !(cell.ques == 0 || cell.ques == 51)
                || !(cell.anum == -1 || (1..=9).contains(&cell.anum))
                || !(0..=4).contains(&cell.qsub)
                || !(0..=1).contains(&cell.qcmp)
                || cell.qans != 0
            {
                bad = true;
            }
            if cell.ques == 51 {
                // `Cell@kakuro#1`: edit-mode clues are 0–45 (0 is allowed by
                // `prehook`); a `?` (-2) cannot be entered (`disInputHatena`).
                if !(-1..=45).contains(&cell.qnum) || !(-1..=45).contains(&cell.qnum2) {
                    bad = true;
                }
            } else if cell.qnum != -1 || cell.qnum2 != -1 {
                lossy = true;
            }
        }
        // `ExCell@kakuro`: minnum 0, maxnum 45.
        if b
            .excells
            .iter()
            .any(|ex| !(-1..=45).contains(&ex.qnum) || !(-1..=45).contains(&ex.qnum2))
        {
            bad = true;
        }
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kakuro cell value outside [＼] 0-45 / answer 1-9 or outside clue 0-45",
            );
        }
        if lossy {
            d.push(
                DiagnosticCode::LossyUrl,
                "kakuro clue numbers on non-[＼] cells are dropped from the URL",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData` → `adjustQues51_1`. pzprst calls the
    /// hook once per composed transform, so the pre-transform clue sequences
    /// are saved on the (otherwise unused) `qnum3`/`qnum4` of the excells; the
    /// `_1`/`_2` pair is then replayed operation by operation in
    /// `adjust_board_data`.
    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        if t == Transform::Identity || b.geom.hasexcell != 1 {
            return;
        }
        let (cols, rows) = (b.cols(), b.rows());
        for (i, ex) in b.excells.iter_mut().enumerate() {
            // `adjustQues51_1` reads the left column's `qnum` (cell across
            // clues) and the top row's `qnum2` (cell down clues).
            ex.qnum3 = if i < cols {
                ex.qnum2
            } else if i < cols + rows {
                ex.qnum
            } else {
                -1
            };
            ex.qnum4 = i as i32;
        }
    }

    /// pzprjs `BoardExec.adjustBoardData2` → `adjustQues51_2`, replayed for
    /// every pzprjs operation of the composed transform in
    /// `tools/gen_fixtures.js` order.
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        if t == Transform::Identity || b.geom.hasexcell != 1 || b.excells.is_empty() {
            return;
        }
        let (cols, rows) = (b.cols(), b.rows());
        let (pc, pr) = if t.swaps_axes() {
            (rows, cols)
        } else {
            (cols, rows)
        };
        // Recover the pre-transform clue sequences from the scratch slots. The
        // objects moved with `turnflipGroup("excell", …)`, so their `qnum4` is
        // the pre-transform excell index.
        let mut top = vec![-1; pc];
        let mut left = vec![-1; pr];
        for ex in &b.excells {
            if ex.qnum4 < 0 {
                continue;
            }
            let i = ex.qnum4 as usize;
            if i < pc {
                top[i] = ex.qnum3;
            } else if i < pc + pr {
                left[i - pc] = ex.qnum3;
            }
        }
        // The cell values travelled with the coordinate map; invert it to get
        // the pre-transform board `adjustQues51_1` captured.
        let mut qnum = vec![-1; pc * pr];
        let mut qnum2 = vec![-1; pc * pr];
        let mut is51 = vec![false; pc * pr];
        for y in 0..pr {
            for x in 0..pc {
                let (bx, by) = (2 * x as i32 + 1, 2 * y as i32 + 1);
                let (fx, fy) =
                    t.map_point(bx, by, pc as i32, pr as i32, cols as i32, rows as i32);
                let Some(fi) = b.geom.cell_index(fx, fy) else {
                    continue;
                };
                let cell = &b.cells[fi];
                let i = y * pc + x;
                qnum[i] = cell.qnum;
                qnum2[i] = cell.qnum2;
                is51[i] = cell.ques == 51;
            }
        }
        let mut state = Q51 {
            cols: pc,
            rows: pr,
            qnum,
            qnum2,
            is51,
            top,
            left,
        };
        for &op in operations(t) {
            state.apply(op);
        }
        if state.cols != cols || state.rows != rows {
            for ex in b.excells.iter_mut() {
                ex.qnum3 = -1;
                ex.qnum4 = -1;
            }
            return;
        }
        for (i, cell) in b.cells.iter_mut().enumerate() {
            cell.qnum = state.qnum[i];
            cell.qnum2 = state.qnum2[i];
        }
        let top_len = cols;
        let left_len = rows;
        for ex in b.excells.iter_mut() {
            ex.qnum3 = -1;
            ex.qnum4 = -1;
        }
        for (i, ex) in b.excells.iter_mut().enumerate() {
            if i < top_len {
                ex.qnum2 = state.top[i];
                ex.qnum = -1;
            } else if i < top_len + left_len {
                ex.qnum = state.left[i - top_len];
                ex.qnum2 = -1;
            } else {
                ex.qnum = -1;
                ex.qnum2 = -1;
            }
        }
    }
}

/// The `qnum`/`qnum2` data `adjustQues51_1`/`_2` permute: per-cell clues,
/// the top outside clues (`top`, `qnum2`) and the left ones (`left`, `qnum`).
struct Q51 {
    cols: usize,
    rows: usize,
    qnum: Vec<i32>,
    qnum2: Vec<i32>,
    is51: Vec<bool>,
    top: Vec<i32>,
    left: Vec<i32>,
}

impl Q51 {
    /// pzprjs `adjustQues51_1`: every row's `qnum` sequence (`qnumw`, left
    /// excell first) and every column's `qnum2` sequence (`qnumh`, top excell
    /// first).
    fn capture(&self) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
        let mut qnumw = Vec::with_capacity(self.rows);
        for y in 0..self.rows {
            let mut seq = vec![self.left.get(y).copied().unwrap_or(-1)];
            for x in 0..self.cols {
                let i = y * self.cols + x;
                if self.is51[i] {
                    seq.push(self.qnum[i]);
                }
            }
            qnumw.push(seq);
        }
        let mut qnumh = Vec::with_capacity(self.cols);
        for x in 0..self.cols {
            let mut seq = vec![self.top.get(x).copied().unwrap_or(-1)];
            for y in 0..self.rows {
                let i = y * self.cols + x;
                if self.is51[i] {
                    seq.push(self.qnum2[i]);
                }
            }
            qnumh.push(seq);
        }
        (qnumw, qnumh)
    }

    /// Replace the cell grid, keeping `map(x, y)` as the destination index on
    /// the new `cols × rows` board.
    fn remap(&mut self, cols: usize, rows: usize, map: impl Fn(usize, usize) -> usize) {
        let (old_cols, _) = (self.cols, self.rows);
        let mut qnum = vec![-1; cols * rows];
        let mut qnum2 = vec![-1; cols * rows];
        let mut is51 = vec![false; cols * rows];
        for y in 0..self.rows {
            for x in 0..old_cols {
                let from = y * old_cols + x;
                let to = map(x, y);
                qnum[to] = self.qnum[from];
                qnum2[to] = self.qnum2[from];
                is51[to] = self.is51[from];
            }
        }
        self.cols = cols;
        self.rows = rows;
        self.qnum = qnum;
        self.qnum2 = qnum2;
        self.is51 = is51;
    }

    /// pzprjs's `turnflipGroup("cell"/"excell", …)` followed by
    /// `adjustQues51_2` for one board operation.
    fn apply(&mut self, op: Transform) {
        let (qnumw, qnumh) = self.capture();
        let (old_cols, old_rows) = (self.cols, self.rows);
        match op {
            Transform::FlipX => {
                self.remap(old_cols, old_rows, |x, y| {
                    y * old_cols + (old_cols - 1 - x)
                });
                self.top.reverse();
                for (y, seq) in qnumw.iter().enumerate().take(self.rows) {
                    let mut rev = seq.clone();
                    rev.reverse();
                    self.left[y] = rev.first().copied().unwrap_or(-1);
                    let mut k = 1;
                    for x in 0..self.cols {
                        let i = y * self.cols + x;
                        if self.is51[i] {
                            self.qnum[i] = rev.get(k).copied().unwrap_or(-1);
                            k += 1;
                        }
                    }
                }
            }
            Transform::FlipY => {
                self.remap(old_cols, old_rows, |x, y| {
                    (old_rows - 1 - y) * old_cols + x
                });
                self.left.reverse();
                for (x, seq) in qnumh.iter().enumerate().take(self.cols) {
                    let mut rev = seq.clone();
                    rev.reverse();
                    self.top[x] = rev.first().copied().unwrap_or(-1);
                    let mut k = 1;
                    for y in 0..self.rows {
                        let i = y * self.cols + x;
                        if self.is51[i] {
                            self.qnum2[i] = rev.get(k).copied().unwrap_or(-1);
                            k += 1;
                        }
                    }
                }
            }
            Transform::Rot90 => {
                let nc = old_rows;
                self.remap(nc, old_cols, |x, y| x * nc + (old_rows - 1 - y));
                self.reinterpret_excells();
                for (r, seq) in qnumh.iter().enumerate().take(self.rows) {
                    let mut rev = seq.clone();
                    rev.reverse();
                    self.left[r] = rev.first().copied().unwrap_or(-1);
                    let mut k = 1;
                    for c in 0..self.cols {
                        let i = r * self.cols + c;
                        if self.is51[i] {
                            self.qnum[i] = rev.get(k).copied().unwrap_or(-1);
                            k += 1;
                        }
                    }
                }
                for (c, seq) in qnumw.iter().rev().enumerate().take(self.cols) {
                    self.top[c] = seq.first().copied().unwrap_or(-1);
                    let mut k = 1;
                    for r in 0..self.rows {
                        let i = r * self.cols + c;
                        if self.is51[i] {
                            self.qnum2[i] = seq.get(k).copied().unwrap_or(-1);
                            k += 1;
                        }
                    }
                }
            }
            Transform::Rot270 => {
                let nc = old_rows;
                self.remap(nc, old_cols, |x, y| (old_cols - 1 - x) * nc + y);
                self.reinterpret_excells();
                for (r, seq) in qnumh.iter().rev().enumerate().take(self.rows) {
                    self.left[r] = seq.first().copied().unwrap_or(-1);
                    let mut k = 1;
                    for c in 0..self.cols {
                        let i = r * self.cols + c;
                        if self.is51[i] {
                            self.qnum[i] = seq.get(k).copied().unwrap_or(-1);
                            k += 1;
                        }
                    }
                }
                for (c, seq) in qnumw.iter().enumerate().take(self.cols) {
                    let mut rev = seq.clone();
                    rev.reverse();
                    self.top[c] = rev.first().copied().unwrap_or(-1);
                    let mut k = 1;
                    for r in 0..self.rows {
                        let i = r * self.cols + c;
                        if self.is51[i] {
                            self.qnum2[i] = rev.get(k).copied().unwrap_or(-1);
                            k += 1;
                        }
                    }
                }
            }
            Transform::Identity
            | Transform::Rot180
            | Transform::Transpose
            | Transform::AntiTranspose => {}
        }
    }

    /// A turn leaves the pzprjs excell vector alone; `setposAll` then reads
    /// the first `cols` entries as the new top row and the rest as the new
    /// left column. `adjustQues51_2` overwrites both, but the dimensions have
    /// to match for the replay.
    fn reinterpret_excells(&mut self) {
        let mut vec = std::mem::take(&mut self.top);
        vec.append(&mut self.left);
        self.top = vec[..self.cols].to_vec();
        self.left = vec[self.cols..self.cols + self.rows].to_vec();
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

/// `decodeKakuro`: run-length coded cell layer (`k`–`z` skip cells) followed
/// by the outside clues (top `qnum2`, then left `qnum`). pzprjs only rewrites
/// `outbstr` after the cell layer, so the returned rest is the truly
/// unconsumed tail.
fn decode_kakuro(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let cells = b.cells.len();
    let (mut c, mut a) = (0usize, 0usize);
    let mut i = 0usize;
    while i < chars.len() {
        if c >= cells {
            // pzprjs would throw reading `bd.cell[c]`; no fixture can.
            a = i + 1;
            break;
        }
        let ca = chars[i];
        if ('k'..='z').contains(&ca) {
            c += jsnum::digit(ca, 36).unwrap() as usize - 19;
        } else {
            let cell = &mut b.cells[c];
            cell.ques = 51;
            if ca != '.' {
                cell.qnum2 = decval(ca);
                cell.qnum = chars.get(i + 1).map_or(-1, |&next| decval(next));
                i += 1;
            }
            c += 1;
        }
        if c >= cells {
            a = i + 1;
            break;
        }
        i += 1;
    }
    if c < cells {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    let (cols, rows) = (b.cols(), b.rows());
    let mut i = a;
    for col in 0..cols {
        let bx = 2 * col as i32 + 1;
        if b.obj(Group::Cell, bx, 1)
            .map_or(true, |cell| cell.ques != 51)
        {
            let val = chars.get(i).map_or(-1, |&ca| decval(ca));
            if let Some(ex) = b.obj_mut(Group::ExCell, bx, -1) {
                ex.qnum2 = val;
            }
            i += 1;
        }
    }
    for row in 0..rows {
        let by = 2 * row as i32 + 1;
        if b.obj(Group::Cell, 1, by)
            .map_or(true, |cell| cell.ques != 51)
        {
            let val = chars.get(i).map_or(-1, |&ca| decval(ca));
            if let Some(ex) = b.obj_mut(Group::ExCell, -1, by) {
                ex.qnum = val;
            }
            i += 1;
        }
    }
    chars[i.min(chars.len())..].iter().collect()
}

/// `encodeKakuro`: cell layer run-length coded with 20–35 base 36 markers,
/// then the outside clues.
fn encode_kakuro(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let mut pstr = String::new();
        if cell.ques == 51 {
            if cell.qnum < 0 && cell.qnum2 < 0 {
                pstr.push('.');
            } else {
                pstr.push_str(&encval(cell.qnum2));
                pstr.push_str(&encval(cell.qnum));
            }
        } else {
            count += 1;
        }
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
    let (cols, rows) = (b.cols(), b.rows());
    for col in 0..cols {
        let bx = 2 * col as i32 + 1;
        if b.obj(Group::Cell, bx, 1)
            .map_or(true, |cell| cell.ques != 51)
        {
            let val = b.obj(Group::ExCell, bx, -1).map_or(-1, |ex| ex.qnum2);
            cm.push_str(&encval(val));
        }
    }
    for row in 0..rows {
        let by = 2 * row as i32 + 1;
        if b.obj(Group::Cell, 1, by)
            .map_or(true, |cell| cell.ques != 51)
        {
            let val = b.obj(Group::ExCell, -1, by).map_or(-1, |ex| ex.qnum);
            cm.push_str(&encval(val));
        }
    }
    cm
}

/// `decval`: `0`–`9` / `a`–`j` are 0–19 and `A`–`Z` are 20–45; anything else
/// (including an absent character) is -1.
fn decval(ca: char) -> i32 {
    if ca.is_ascii_digit() {
        ca as i32 - '0' as i32
    } else if ('a'..='j').contains(&ca) {
        10 + ca as i32 - 'a' as i32
    } else if ca.is_ascii_uppercase() {
        10 + jsnum::digit(ca, 36).unwrap() as i32
    } else {
        -1
    }
}

/// `encval`: 0–19 are lowercase base 36, 20–45 uppercase, anything else `-`.
fn encval(val: i32) -> String {
    if (0..=19).contains(&val) {
        jsnum::to_radix(val as u32, 36)
    } else if (20..=45).contains(&val) {
        jsnum::to_radix((val - 10) as u32, 36).to_uppercase()
    } else {
        "-".into()
    }
}

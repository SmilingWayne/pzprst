//! Tri-place (pzprjs `variety/triplace.js`, pid `triplace`).
//!
//! A `[＼]` cell (`ques` 51) stores its across clue in `qnum` and its down
//! clue in `qnum2`; a top outside clue is an excell's `qnum2` and a left one
//! its `qnum`. `"ExCell: { ques: 51 }"` makes `Board.emptyexcell` the top-left
//! corner, so the excell block has one extra entry. The URL is
//! `decodeTriplace`: a run-length coded cell layer (`_` empty `[＼]`, two hex
//! digits `qnum2 qnum`, `$`/`@` one base-36 number, `-`/`+`/`=` mixed-width
//! escapes) followed by the outside clues. FileIO uses the shared
//! `decodeCellQnum51` / `encodeCellQnum51` layer, `decodeBorderAns` and a
//! cell sub-mark layer. Board transforms replay pzprjs's
//! `BoardExec.adjustQues51_1`/`_2` pair.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Triplace;

struct Triplace;

impl Genre for Triplace {
    fn pid(&self) -> &'static str {
        "triplace"
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

    /// pzprjs's `ExCell.ques` is 51, so `Board.emptyexcell` is the top-left
    /// corner excell.
    fn excell_corner(&self) -> bool {
        true
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_triplace(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_triplace(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum51(r, b, d)?;
        fileio::decode_border_ans(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            if ca == "+" {
                cell.qsub = 1;
            } else if ca == "-" {
                cell.qsub = 2;
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum51(w, b);
        fileio::encode_border_ans(w, b);
        w.encode_cell(b, |cell| match cell.qsub {
            1 => "+ ".into(),
            2 => "- ".into(),
            _ => ". ".into(),
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let mut bad = false;
        let mut lossy = false;
        for (i, cell) in b.cells.iter().enumerate() {
            let (x, y) = ((i % b.cols()) as i32, (i / b.cols()) as i32);
            // `Cell.getmaxnum`: the clue counts the cells right of or below
            // this one, depending on the input target.
            let max = (cols - x - 1).max(rows - y - 1);
            if !(cell.ques == 0 || cell.ques == 51)
                || !(0..=2).contains(&cell.qsub)
                || cell.qans != 0
            {
                bad = true;
            }
            if cell.ques == 51 {
                // minnum 0; a `?` cannot be typed (`disInputHatena`).
                if !(-1..=max).contains(&cell.qnum) || !(-1..=max).contains(&cell.qnum2) {
                    bad = true;
                }
            } else if cell.qnum != -1 || cell.qnum2 != -1 {
                lossy = true;
            }
        }
        if b.borders.iter().any(|border| {
            !(0..=1).contains(&border.qans) || !(0..=1).contains(&border.qsub) || border.qnum != -1
        }) {
            bad = true;
        }
        // `ExCell`: minnum 0, one clue per column/row, plus the corner object
        // that only carries its `ques`. A top excell is encoded as `qnum2`, a
        // left one as `qnum`; the other direction of each is dropped.
        let qc = b.cols();
        let qr = b.rows();
        for (i, ex) in b.excells.iter().enumerate() {
            let max = if i < qc {
                rows
            } else if i < qc + qr {
                cols
            } else {
                rows
            };
            if !(-1..=max).contains(&ex.qnum) || !(-1..=max).contains(&ex.qnum2) {
                bad = true;
            }
            let unencoded = if i < qc {
                ex.qnum
            } else if i < qc + qr {
                ex.qnum2
            } else {
                ex.qnum.max(ex.qnum2)
            };
            if unencoded != -1 {
                lossy = true;
            }
        }
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "triplace cell value outside [＼] 0-maxnum / sub mark +- or outside clue 0-maxnum",
            );
        }
        if lossy {
            d.push(
                DiagnosticCode::LossyUrl,
                "triplace clue numbers on non-[＼] cells or in the unencoded direction are dropped from the URL",
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
                let (fx, fy) = t.map_point(bx, by, pc as i32, pr as i32, cols as i32, rows as i32);
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
        let clear = |b: &mut Board| {
            for ex in b.excells.iter_mut() {
                ex.qnum3 = -1;
                ex.qnum4 = -1;
            }
        };
        if state.cols != cols || state.rows != rows {
            clear(b);
            return;
        }
        for (i, cell) in b.cells.iter_mut().enumerate() {
            cell.qnum = state.qnum[i];
            cell.qnum2 = state.qnum2[i];
        }
        clear(b);
        for (i, ex) in b.excells.iter_mut().enumerate() {
            if i < cols {
                ex.qnum2 = state.top[i];
                ex.qnum = -1;
            } else if i < cols + rows {
                ex.qnum = state.left[i - cols];
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
        let old_cols = self.cols;
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
        self.top = vec[..self.cols.min(vec.len())].to_vec();
        self.left = vec[self.cols.min(vec.len())..self.cols + self.rows].to_vec();
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

/// pzprjs `parseInt(s, 16)` for one character (or an absent one).
fn hex_char(ca: Option<char>, d: &mut Diagnostics) -> i32 {
    let parsed = ca.map(|c| c.to_string()).and_then(|s| jsnum::parse_int(&s, 16));
    match parsed {
        Some(v) => v as i32,
        None => {
            d.push(
                DiagnosticCode::UnexpectedChar,
                "non-hex digit in a base-16 clue leaves it empty",
            );
            -1
        }
    }
}

/// pzprjs `parseInt(s, 36)` for one character (or an absent one).
fn base36_char(ca: Option<char>, d: &mut Diagnostics) -> i32 {
    let parsed = ca.map(|c| c.to_string()).and_then(|s| jsnum::parse_int(&s, 36));
    match parsed {
        Some(v) if (0..=35).contains(&v) => v as i32,
        _ => {
            d.push(
                DiagnosticCode::UnexpectedChar,
                "non-base-36 digit in a clue leaves it empty",
            );
            -1
        }
    }
}

/// pzprjs `parseInt(bstr.substr(i, 2), 16)` (up to two characters, all of the
/// longest hex prefix counts).
fn hex_span(chars: &[char], start: usize, d: &mut Diagnostics) -> i32 {
    let text = jsnum::substr(chars, start, 2);
    match jsnum::parse_int(&text, 16) {
        Some(v) => v as i32,
        None => {
            d.push(
                DiagnosticCode::UnexpectedChar,
                "incomplete hex escape leaves a clue empty",
            );
            -1
        }
    }
}

/// `decodeTriplace`: run-length coded cell layer followed by the outside clues
/// (top `qnum2`, then left `qnum`). pzprjs only rewrites `outbstr` at the end,
/// so the returned value is `bstr.substr(a)` for its final `a`.
fn decode_triplace(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let cells = b.cells.len();
    let (mut id, mut a) = (0usize, 0usize);
    let mut i = 0usize;
    let mut complete = false;
    while i < chars.len() {
        let ca = chars[i];
        if ('g'..='z').contains(&ca) {
            id += jsnum::digit(ca, 36).unwrap() as usize - 16;
        } else {
            let cell = &mut b.cells[id];
            // pzprjs `Cell.set51cell`
            cell.ques = 51;
            cell.qnum = -1;
            cell.qnum2 = -1;
            match ca {
                '_' => {}
                '%' | '@' => {
                    cell.qnum2 = base36_char(chars.get(i + 1).copied(), d);
                    i += 1;
                }
                '$' => {
                    cell.qnum = base36_char(chars.get(i + 1).copied(), d);
                    i += 1;
                }
                '-' => {
                    cell.qnum2 = if chars.get(i + 1) == Some(&'.') {
                        -1
                    } else {
                        hex_char(chars.get(i + 1).copied(), d)
                    };
                    cell.qnum = hex_span(&chars, i + 2, d);
                    i += 3;
                }
                '+' => {
                    cell.qnum2 = hex_span(&chars, i + 1, d);
                    cell.qnum = if chars.get(i + 3) == Some(&'.') {
                        -1
                    } else {
                        hex_char(chars.get(i + 3).copied(), d)
                    };
                    i += 3;
                }
                '=' => {
                    cell.qnum2 = hex_span(&chars, i + 1, d);
                    cell.qnum = hex_span(&chars, i + 3, d);
                    i += 4;
                }
                _ => {
                    cell.qnum2 = if ca == '.' {
                        -1
                    } else {
                        hex_char(Some(ca), d)
                    };
                    cell.qnum = if chars.get(i + 1) == Some(&'.') {
                        -1
                    } else {
                        hex_char(chars.get(i + 1).copied(), d)
                    };
                    i += 1;
                }
            }
        }
        id += 1;
        if id >= cells {
            a = i + 1;
            complete = true;
            break;
        }
        i += 1;
    }
    if !complete {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    // Outside clues: the top excells' `qnum2`, then the left ones' `qnum`.
    let (cols, rows) = (b.cols(), b.rows());
    id = 0;
    let mut j = a;
    let mut top_done = false;
    while j < chars.len() {
        let ca = chars[j];
        let value = match ca {
            '.' => -1,
            '-' => {
                let v = hex_span(&chars, j + 1, d);
                j += 2;
                v
            }
            _ => hex_char(Some(ca), d),
        };
        if let Some(ex) = b.excells.get_mut(id) {
            ex.qnum2 = value;
        }
        id += 1;
        if id >= cols {
            a = j + 1;
            top_done = true;
            break;
        }
        j += 1;
    }
    if !top_done {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last top clue",
        );
    }
    let mut left_done = false;
    let mut j = a;
    while j < chars.len() {
        let ca = chars[j];
        let value = match ca {
            '.' => -1,
            '-' => {
                let v = hex_span(&chars, j + 1, d);
                j += 2;
                v
            }
            _ => hex_char(Some(ca), d),
        };
        if let Some(ex) = b.excells.get_mut(id) {
            ex.qnum = value;
        }
        id += 1;
        if id >= cols + rows {
            a = j + 1;
            left_done = true;
            break;
        }
        j += 1;
    }
    if !left_done {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last left clue",
        );
    }
    chars[a.min(chars.len())..].iter().collect()
}

/// `n.toString(radix)` including the `-` sign of a negative value, as
/// pzprjs's encoder would produce for an out-of-range clue.
fn to_base(n: i32, radix: u32) -> String {
    if n < 0 {
        format!("-{}", jsnum::to_radix(n.unsigned_abs(), radix))
    } else {
        jsnum::to_radix(n as u32, radix)
    }
}

/// `encodeTriplace`: cell layer run-length coded with 15–35 base 36 markers,
/// then the outside clues.
fn encode_triplace(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let mut pstr = String::new();
        if cell.ques == 51 {
            if cell.qnum == -1 && cell.qnum2 == -1 {
                pstr.push('_');
            } else if cell.qnum2 == -1 && cell.qnum < 35 {
                pstr.push('$');
                pstr.push_str(&to_base(cell.qnum, 36));
            } else if cell.qnum == -1 && cell.qnum2 < 35 {
                pstr.push('@');
                pstr.push_str(&to_base(cell.qnum2, 36));
            } else {
                pstr.push_str(&to_base(cell.qnum2, 16));
                pstr.push_str(&to_base(cell.qnum, 16));
                if cell.qnum >= 16 && cell.qnum2 >= 16 {
                    pstr.insert(0, '=');
                } else if cell.qnum >= 16 {
                    pstr.insert(0, '-');
                } else if cell.qnum2 >= 16 {
                    pstr.insert(0, '+');
                }
            }
        } else {
            count += 1;
        }
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 20 {
            cm.push_str(&jsnum::to_radix(count + 15, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(count + 15, 36));
    }
    let (cols, rows) = (b.cols(), b.rows());
    for col in 0..cols {
        push_clue(&mut cm, b.excells.get(col).map_or(-1, |ex| ex.qnum2));
    }
    for row in 0..rows {
        push_clue(&mut cm, b.excells.get(cols + row).map_or(-1, |ex| ex.qnum));
    }
    cm
}

/// One outside clue: `.` empty, one hex digit, or `-` plus two hex digits.
fn push_clue(cm: &mut String, num: i32) {
    if num < 0 {
        cm.push('.');
    } else if num < 16 {
        cm.push_str(&jsnum::to_radix(num as u32, 16));
    } else if num < 256 {
        cm.push('-');
        cm.push_str(&jsnum::to_radix(num as u32, 16));
    }
}

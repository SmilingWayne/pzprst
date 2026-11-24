//! Slalom (pzprjs `variety/slalom.js`, pid `slalom`, alias `suraromu`).
//!
//! A cell's `ques` is 0 (empty), 1 (wall with a gate number in `qnum` and a
//! direction in `qdir`), 21 (vertical gate) or 22 (horizontal gate). pzprjs
//! also keeps the start in an extra `board.startpos` address object; pzprst's
//! board model has no extra objects, so the start cell carries the `qnum2 = 1`
//! marker (it moves with the cell, exactly like pzprjs maps the address).
//!
//! The URL is `decodeSlalom`/`encodeSlalom`, whose body is
//! `cellLayer + gateLayer "/" startId`; the pzprv3 `d` flag selects the current
//! version 2, the decode-only `p` flag version 1 and no flag the old version 0
//! (which stores one number per gate component and distributes it to the
//! adjacent cells), exactly as `checkpflag("d")` / `checkpflag("p")` do.
//! pzprjs always re-encodes URL_PZPRV3 with `d`, so both rules are normalized
//! after decoding. FileIO is `decodeBoard_pzpr2` + `decodeBorderArrowAns` for
//! filever 3, with older `decodeBoard_pzpr1` / `decodeBoard_old` branches.
//! `BoardExec` swaps gate orientations under turns, rotates `qdir`
//! (`adjustNumberArrow`) and re-maps the start position.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Slalom;

/// pzprjs `checkpflag("d")`: decode version 2, always written for URL_PZPRV3.
const URL_V2: &str = "slalom_urlver2";
/// pzprjs `checkpflag("p")`: decode-only version 1.
const URL_V1: &str = "slalom_urlver1";

struct Slalom;

impl Genre for Slalom {
    fn pid(&self) -> &'static str {
        "slalom"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('d', URL_V2), ('p', URL_V1)]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let ver = if b.rules.get(URL_V2).copied().unwrap_or(false) {
            2
        } else if b.rules.get(URL_V1).copied().unwrap_or(false) {
            1
        } else {
            0
        };
        // `encodePzpr` always sets `outpflag = "d"` for URL_PZPRV3 and encodes
        // version 2, so the canonical URL never keeps `p`.
        b.rules.insert(URL_V2, true);
        b.rules.insert(URL_V1, false);
        decode_slalom(b, body, ver, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_slalom(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        if r.filever >= 2 {
            decode_board_pzpr2(r, b, d)?;
        } else if r.filever == 1 {
            decode_board_pzpr1(r, b, d)?;
        } else {
            decode_board_old(r, b, d)?;
        }
        if r.filever == 3 {
            decode_border_arrow_ans(r, b, d)?;
        } else {
            fileio::decode_border_line(r, b, d)?;
        }
        // `encodePzpr(URL_PZPRV3)` always writes the `d` flag.
        b.rules.insert(URL_V2, true);
        b.rules.insert(URL_V1, false);
        Ok(())
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.filever = 3;
        encode_board_pzpr2(b, w);
        encode_border_arrow_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum`: `Math.min(999, gatemgr.components.length)`; `minnum` 1.
        let (_, gate_count) = gate_components(b);
        let maxnum = gate_count.min(999) as i32;
        if b.cells.iter().any(|c| {
            !matches!(c.ques, 0 | 1 | 21 | 22)
                || !(-1..=4).contains(&c.qdir)
                || !(c.qnum == -2 || c.qnum == -1 || (0..=maxnum).contains(&c.qnum))
                || !(c.qnum2 == -1 || c.qnum2 == 1)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("slalom cell value outside empty/wall/gate or clue 0-{maxnum}/?"),
            );
        }
        // The URL gate layer only stores a positive clue on a wall cell.
        if b.cells.iter().any(|c| {
            (c.ques == 1 && c.qnum != -1 && c.qnum < 1)
                || (c.ques != 1 && (c.qnum != -1 || c.qdir != 0))
        }) {
            d.push(
                DiagnosticCode::LossyUrl,
                "slalom clues on non-wall cells, or non-positive wall clues, are dropped from the URL",
            );
        }
        if b.borders
            .iter()
            .any(|e| e.line < 0 || !matches!(e.qsub, 0 | 2 | 11..=14))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "slalom border value outside line >= 0 or aux mark 0/cross/u/d/l/r",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`: swaps the gate orientation under
    /// turns, rotates every cell's `qdir` (`adjustNumberArrow`) and maps the
    /// start position. The start marker travels with its cell, which is the
    /// same map `getAfterPos` applies to `board.startpos`, so only the value
    /// transforms run here, in `tools/gen_fixtures.js` operation order.
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        for op in ops(t) {
            if matches!(op, Op::TurnR | Op::TurnL) {
                for cell in &mut b.cells {
                    cell.ques = match cell.ques {
                        21 => 22,
                        22 => 21,
                        q => q,
                    };
                }
            }
            for cell in &mut b.cells {
                cell.qdir = rotate_qdir(*op, cell.qdir);
            }
        }
    }
}

/// The pzprjs board operations a composed [`Transform`] stands for.
#[derive(Clone, Copy)]
enum Op {
    TurnR,
    TurnL,
    FlipX,
    FlipY,
}

fn ops(t: Transform) -> &'static [Op] {
    match t {
        Transform::Identity => &[],
        Transform::Rot90 => &[Op::TurnR],
        Transform::Rot180 => &[Op::TurnR, Op::TurnR],
        Transform::Rot270 => &[Op::TurnL],
        Transform::FlipX => &[Op::FlipX],
        Transform::FlipY => &[Op::FlipY],
        Transform::Transpose => &[Op::TurnL, Op::FlipY],
        Transform::AntiTranspose => &[Op::TurnR, Op::FlipY],
    }
}

/// `getTranslateDir` as one direction mapping; other values are unchanged.
fn rotate_qdir(op: Op, qdir: i32) -> i32 {
    match (op, qdir) {
        (Op::TurnR, 1) => 4,
        (Op::TurnR, 2) => 3,
        (Op::TurnR, 3) => 1,
        (Op::TurnR, 4) => 2,
        (Op::TurnL, 1) => 3,
        (Op::TurnL, 2) => 4,
        (Op::TurnL, 3) => 2,
        (Op::TurnL, 4) => 1,
        (Op::FlipX, 3) => 4,
        (Op::FlipX, 4) => 3,
        (Op::FlipY, 1) => 2,
        (Op::FlipY, 2) => 1,
        _ => qdir,
    }
}

/// The start cell's index (`board.startpos.getc().id`).
fn start_index(b: &Board) -> usize {
    b.cells.iter().position(|c| c.qnum2 == 1).unwrap_or(0)
}

/// Store the start cell as the `qnum2 = 1` marker on that cell.
fn set_start(b: &mut Board, index: usize) {
    for cell in &mut b.cells {
        cell.qnum2 = -1;
    }
    if let Some(cell) = b.cells.get_mut(index) {
        cell.qnum2 = 1;
    }
}

/// `decodeSlalom`: the `ver` argument is 0, 1 or 2 from the URL flag.
fn decode_slalom(b: &mut Board, body: &str, ver: u8, d: &mut Diagnostics) -> String {
    let parts: Vec<&str> = body.split('/').collect();
    let layer: Vec<char> = parts.first().copied().unwrap_or("").chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < layer.len() {
        let ca = layer[i];
        let Some(cell) = b.cells.get_mut(c) else {
            break;
        };
        match ca {
            '1' => cell.ques = 1,
            '2' => cell.ques = 21,
            '3' => cell.ques = 22,
            '4'..='9' | 'a'..='z' => {
                c += jsnum::digit(ca, 36).unwrap() as usize - 4;
            }
            _ => {}
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

    if ver == 0 {
        let (gate_of, gate_count) = gate_components(b);
        let mut numbers = vec![-1i32; gate_count];
        let mut r = 0usize;
        i += 1;
        while i < layer.len() && r < gate_count {
            let ca = layer[i];
            if ca.is_ascii_digit() || ('a'..='f').contains(&ca) {
                numbers[r] = jsnum::digit(ca, 16).unwrap() as i32;
            } else if ca == '-' {
                match jsnum::parse_int(&jsnum::substr(&layer, i + 1, 2), 16) {
                    Some(v) => numbers[r] = v as i32,
                    None => d.push(
                        DiagnosticCode::UnexpectedChar,
                        "no two hexadecimal digits after '-' leave a gate unnumbered",
                    ),
                }
                i += 2;
            } else if ('g'..='z').contains(&ca) {
                r += jsnum::digit(ca, 36).unwrap() as usize - 16;
            }
            r += 1;
            if r > gate_count {
                break;
            }
            i += 1;
        }
        distribute_gate_numbers(b, &gate_of, &numbers);
    } else {
        let mut c = 0usize;
        let mut spare = 0i32;
        i += 1;
        while i < layer.len() {
            let Some(cell) = b.cells.get_mut(c) else {
                break;
            };
            let mut advance = true;
            if cell.ques != 1 {
                advance = false;
            } else if spare > 0 {
                spare -= 1;
                advance = false;
            } else {
                let ca = layer[i];
                if ver == 1 && (ca.is_ascii_digit() || ('a'..='f').contains(&ca)) {
                    cell.qnum = jsnum::digit(ca, 16).unwrap() as i32;
                } else if ver == 1 && ca == '-' {
                    match jsnum::parse_int(&jsnum::substr(&layer, i + 1, 2), 16) {
                        Some(v) => cell.qnum = v as i32,
                        None => d.push(
                            DiagnosticCode::UnexpectedChar,
                            "no two hexadecimal digits after '-' leave a clue empty",
                        ),
                    }
                    i += 2;
                } else if ver == 2 && ('0'..='4').contains(&ca) {
                    cell.qdir = jsnum::digit(ca, 16).unwrap() as i32;
                    match jsnum::digit(layer.get(i + 1).copied().unwrap_or(' '), 16) {
                        Some(v) => cell.qnum = v as i32,
                        None => d.push(
                            DiagnosticCode::UnexpectedChar,
                            "no hexadecimal number after a gate direction",
                        ),
                    }
                    i += 1;
                } else if ver == 2 && ('5'..='9').contains(&ca) {
                    cell.qdir = jsnum::digit(ca, 16).unwrap() as i32 - 5;
                    match jsnum::parse_int(&jsnum::substr(&layer, i + 1, 2), 16) {
                        Some(v) => cell.qnum = v as i32,
                        None => d.push(
                            DiagnosticCode::UnexpectedChar,
                            "no two hexadecimal digits after a gate direction",
                        ),
                    }
                    i += 2;
                } else if ver == 2 && ca == '-' {
                    match jsnum::digit(layer.get(i + 1).copied().unwrap_or(' '), 16) {
                        Some(v) => cell.qdir = v as i32,
                        None => d.push(
                            DiagnosticCode::UnexpectedChar,
                            "no hexadecimal direction after '-'",
                        ),
                    }
                    match jsnum::parse_int(&jsnum::substr(&layer, i + 2, 3), 16) {
                        Some(v) => cell.qnum = v as i32,
                        None => d.push(
                            DiagnosticCode::UnexpectedChar,
                            "no three hexadecimal digits after a gate direction",
                        ),
                    }
                    i += 4;
                } else if ('g'..='z').contains(&ca) {
                    spare = jsnum::digit(ca, 36).unwrap() as i32 - 15 - 1;
                }
            }
            c += 1;
            if c >= n {
                break;
            }
            if advance {
                i += 1;
            }
        }
    }

    let start = parts.get(1).copied().unwrap_or("");
    set_start(b, parse_start_id(start, n, d));
    // pzprjs sets `outbstr = array[0].substr(i)`, which is always the last
    // consumed character or the tail of the gate layer; nothing reads it, so
    // the port consumes it instead of reporting trailing data.
    String::new()
}

/// `+array[1] || 0` as a cell index (pzprjs would crash on an invalid id).
fn parse_start_id(token: &str, n: usize, d: &mut Diagnostics) -> usize {
    let v = jsnum::number(token);
    if v.is_nan() || v == 0.0 {
        return 0;
    }
    if v.fract() != 0.0 || v < 0.0 || v >= n as f64 {
        d.push(
            DiagnosticCode::UnexpectedChar,
            format!("start cell {token:?} is outside the board"),
        );
        return 0;
    }
    v as usize
}

/// `encodeSlalom(2)`: the cell layer, the wall-clue layer and `/startId`.
fn encode_slalom(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let pstr = match cell.ques {
            1 => "1",
            21 => "2",
            22 => "3",
            _ => {
                count += 1;
                ""
            }
        };
        if count == 0 {
            cm.push_str(pstr);
        } else if !pstr.is_empty() || count == 32 {
            cm.push_str(&jsnum::to_radix(3 + count, 36));
            cm.push_str(pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(3 + count, 36));
    }

    count = 0;
    for cell in &b.cells {
        if cell.ques != 1 {
            continue;
        }
        let (val, dir) = (cell.qnum, cell.qdir);
        let pstr = if (1..16).contains(&val) {
            format!("{dir}{}", jsnum::to_radix(val as u32, 16))
        } else if (16..256).contains(&val) {
            format!("{}{}", dir + 5, jsnum::to_radix(val as u32, 16))
        } else if (256..4096).contains(&val) {
            format!("-{dir}{}", jsnum::to_radix(val as u32, 16))
        } else {
            count += 1;
            String::new()
        };
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

    cm.push('/');
    cm.push_str(&start_index(b).to_string());
    cm
}

/// `AreaHurdleGraph.components`: cells with `ques` 21 connect vertically and
/// 22 horizontally. Returns the component index per cell (in first-cell order,
/// like `GraphBase.searchGraph`) and the number of components.
fn gate_components(b: &Board) -> (Vec<Option<usize>>, usize) {
    let (cols, rows) = (b.cols(), b.rows());
    let n = b.cells.len();
    let mut comp: Vec<Option<usize>> = vec![None; n];
    let mut total = 0usize;
    for start in 0..n {
        let ques = b.cells[start].ques;
        if comp[start].is_some() || (ques != 21 && ques != 22) {
            continue;
        }
        let id = total;
        total += 1;
        let mut stack = vec![start];
        comp[start] = Some(id);
        while let Some(c) = stack.pop() {
            let (r, col) = (c / cols, c % cols);
            let mut visit = |ci: usize| {
                if comp[ci].is_none() && b.cells[ci].ques == ques {
                    comp[ci] = Some(id);
                    stack.push(ci);
                }
            };
            if ques == 21 {
                if r > 0 {
                    visit(c - cols);
                }
                if r + 1 < rows {
                    visit(c + cols);
                }
            } else {
                if col > 0 {
                    visit(c - 1);
                }
                if col + 1 < cols {
                    visit(c + 1);
                }
            }
        }
    }
    (comp, total)
}

/// `getConnectingGate` + the number distribution of `decodeSlalom(0)` /
/// `decodeBoard_old`: every cell gets the smallest positive number of the
/// vertical/horizontal gates next to it, or -1.
fn distribute_gate_numbers(b: &mut Board, gate_of: &[Option<usize>], numbers: &[i32]) {
    let (cols, rows) = (b.cols(), b.rows());
    for c in 0..b.cells.len() {
        let (r, col) = (c / cols, c % cols);
        let mut min = 1000i32;
        let mut consider = |ci: usize| {
            if b.cells[ci].ques == 21 || b.cells[ci].ques == 22 {
                if let Some(g) = gate_of[ci] {
                    let val = numbers.get(g).copied().unwrap_or(-1);
                    if val > 0 {
                        min = min.min(val);
                    }
                }
            }
        };
        // `getConnectingGate`: top/bottom only for vertical gates (21),
        // left/right only for horizontal gates (22).
        if r > 0 && b.cells[c - cols].ques == 21 {
            consider(c - cols);
        }
        if r + 1 < rows && b.cells[c + cols].ques == 21 {
            consider(c + cols);
        }
        if col > 0 && b.cells[c - 1].ques == 22 {
            consider(c - 1);
        }
        if col + 1 < cols && b.cells[c + 1].ques == 22 {
            consider(c + 1);
        }
        b.cells[c].qnum = if min < 1000 { min } else { -1 };
    }
}

/// `decodeBoard_pzpr2`: `o` start, `i`/`-` gates, `#` wall, `dir,num` clue.
fn decode_board_pzpr2(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let mut idx = 0usize;
    let mut start = 0usize;
    r.decode_cell(b, d, |cell, ca| {
        let i = idx;
        idx += 1;
        if ca == "o" {
            start = i;
        } else if ca == "i" {
            cell.ques = 21;
        } else if ca == "-" {
            cell.ques = 22;
        } else if ca == "#" {
            cell.ques = 1;
        } else if ca != "." {
            let mut inp = ca.split(',');
            let dir = inp.next().unwrap_or("");
            let num = inp.next().ok_or("missing ',' in a direction,number token")?;
            cell.ques = 1;
            cell.qdir = if dir != "0" { fileio::int(dir)? } else { 0 };
            cell.qnum = if num != "-" {
                fileio::int(num)?
            } else {
                -2
            };
        }
        Ok(())
    })?;
    set_start(b, start);
    Ok(())
}

/// `decodeBoard_pzpr1`: `o` start, `i`/`-` gates, `#` wall, a number clue.
fn decode_board_pzpr1(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let mut idx = 0usize;
    let mut start = 0usize;
    r.decode_cell(b, d, |cell, ca| {
        let i = idx;
        idx += 1;
        if ca == "o" {
            start = i;
        } else if ca == "i" {
            cell.ques = 21;
        } else if ca == "-" {
            cell.ques = 22;
        } else if ca == "#" {
            cell.ques = 1;
        } else if ca != "." {
            cell.ques = 1;
            cell.qnum = fileio::int(ca)?;
        }
        Ok(())
    })?;
    set_start(b, start);
    Ok(())
}

/// `decodeBoard_old`: `#` wall, `o` start, `i`/`w` gates with an optional
/// trailing per-cell gate number that is distributed afterwards.
fn decode_board_old(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let n = b.cells.len();
    let mut sv_num = vec![-1i32; n];
    let mut idx = 0usize;
    let mut start = 0usize;
    let mut bad = Vec::new();
    r.decode_cell(b, d, |cell, ca| {
        let i = idx;
        idx += 1;
        if ca == "#" {
            cell.ques = 1;
        } else if ca == "o" {
            start = i.min(n.saturating_sub(1));
        } else if ca != "." {
            if ca.starts_with('i') {
                cell.ques = 21;
            } else if ca.starts_with('w') {
                cell.ques = 22;
            }
            if ca.chars().count() > 1 {
                let rest: String = ca.chars().skip(1).collect();
                match jsnum::integer(&rest) {
                    Some(v) if i < n => sv_num[i] = v,
                    _ => bad.push(format!("no base-10 gate number in {ca:?}")),
                }
            }
        }
        Ok(())
    })?;
    for message in bad {
        d.push(DiagnosticCode::UnexpectedChar, message);
    }
    let (gate_of, gate_count) = gate_components(b);
    let mut numbers = vec![-1i32; gate_count];
    for (c, &num) in sv_num.iter().enumerate() {
        if num != -1 {
            // `cell[c].gate.number = num`; a pzprjs cell without a gate throws.
            if let Some(g) = gate_of[c] {
                numbers[g] = num;
            }
        }
    }
    distribute_gate_numbers(b, &gate_of, &numbers);
    set_start(b, start);
    Ok(())
}

/// `encodeBoard_pzpr2`: the start cell, gates, walls and `dir,num` clues.
fn encode_board_pzpr2(b: &Board, w: &mut FileWriter) {
    let mut idx = 0usize;
    w.encode_cell(b, |cell| {
        let i = idx;
        idx += 1;
        if i == start_index(b) {
            "o ".into()
        } else if cell.ques == 21 {
            "i ".into()
        } else if cell.ques == 22 {
            "- ".into()
        } else if cell.ques == 1 && cell.qnum <= 0 {
            "# ".into()
        } else if cell.ques == 1 {
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

/// `decodeBorderArrowAns`: border line, or `-n` for a line plus cross
/// (`qsub` 2), with an optional trailing `u`/`d`/`l`/`r` aux mark (`qsub`
/// 11–14). Any other lowercase letter is stripped without setting a mark.
pub(super) fn decode_border_arrow_ans(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_border(b, d, |border, ca| {
        let mut ca = ca.to_string();
        if let Some(lca) = ca.chars().last() {
            if lca.is_ascii_lowercase() {
                border.qsub = match lca {
                    'u' => 11,
                    'd' => 12,
                    'l' => 13,
                    'r' => 14,
                    _ => border.qsub,
                };
                ca.pop();
            }
        }
        if !ca.is_empty() && ca != "0" {
            if ca.starts_with('-') {
                border.line = -fileio::int(&ca)? - 1;
                border.qsub = 2;
            } else {
                border.line = fileio::int(&ca)?;
            }
        }
        Ok(())
    })
}

/// `encodeBorderArrowAns`: `-1-line` for a line plus cross, the line itself
/// otherwise, then the `u`/`d`/`l`/`r` aux mark; `0` when all are empty.
pub(super) fn encode_border_arrow_ans(w: &mut FileWriter, b: &Board) {
    w.encode_border(b, |border| {
        let mut ca = String::new();
        if border.qsub == 2 {
            ca.push_str(&(-1 - border.line).to_string());
        } else if border.line > 0 {
            ca.push_str(&border.line.to_string());
        }
        if let Some(dir) = match border.qsub {
            11 => Some('u'),
            12 => Some('d'),
            13 => Some('l'),
            14 => Some('r'),
            _ => None,
        } {
            ca.push(dir);
        }
        if ca.is_empty() {
            "0 ".into()
        } else {
            format!("{ca} ")
        }
    });
}

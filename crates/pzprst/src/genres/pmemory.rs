//! Persistence of Memory (pzprjs `variety/pmemory.js`, pid `pmemory`).
//!
//! The URL stores border walls plus one bit per room (pzprjs `roommgr`
//! components); `decode1Cell`/`encode1Cell` carry the two memory markers.

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Pmemory;

struct Pmemory;

impl Genre for Pmemory {
    fn pid(&self) -> &'static str {
        "pmemory"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        for cell in &mut b.cells {
            cell.qnum = -1;
        }
        let rest = encode::decode_border(b, body, d);
        let rest = decode_1cell(b, &rest, d, 1);
        let rooms = rooms(b);
        let rest = encode::generic_decode_binary(rooms.len(), &rest, d, |r, on| {
            if on {
                for &c in &rooms[r] {
                    b.cells[c].ques = 6;
                }
            }
        });
        // pzprjs `decodeURL` ends with `bd.rebuildInfo()`; pmemory's override
        // forces at least two memory markers.
        normalize_markers(b);
        rest
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut cm = encode::encode_border(b);
        cm += &encode_1cell(b, 1);
        let rooms = rooms(b);
        cm += &encode::generic_encode_binary(
            rooms.len(),
            |r| b.cells[rooms[r][0]].ques == 6,
            false,
        );
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_border_ques(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            let val = fileio::int(ca)?;
            cell.ques = if val & 1 != 0 { 6 } else { 0 };
            cell.qnum = if val & 2 != 0 { 1 } else { -1 };
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)?;
        fileio::decode_cell_qsub(r, b, d)?;
        // `filedecode` also calls `bd.rebuildInfo()`.
        normalize_markers(b);
        Ok(())
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_border_ques(w, b);
        w.encode_cell(b, |cell| {
            let mut val = 0;
            if cell.ques == 6 {
                val |= 1;
            }
            if cell.qnum == 1 {
                val |= 2;
            }
            format!("{val} ")
        });
        fileio::encode_border_line(w, b);
        fileio::encode_cell_qsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.ques == 0 || c.ques == 6) || !(c.qnum == -1 || c.qnum == 1) || c.qsub < 0
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "pmemory cell value outside ice 0/6, marker -1/1, background mark >=0",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(e.ques == 0 || e.ques == 1) || !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "pmemory border value outside wall 0/1, line 0/1, cross -1",
            );
        }
        // The URL stores one ice bit per room, so a room that is not uniformly
        // icy loses the other cells' marks.
        if rooms(b)
            .iter()
            .any(|room| room.iter().any(|&c| b.cells[c].ques != b.cells[room[0]].ques))
        {
            d.push(
                DiagnosticCode::LossyUrl,
                "pmemory rooms that mix ice and empty cells keep only the first cell's mark in a URL",
            );
        }
    }
}

/// The pzprjs `roommgr` components of `b`.
///
/// `AreaRoomGraph` joins two cells when the border between them is not a
/// separator (`border.ques == 0 && border.qans == 0`) and drops `ques == 7`
/// cells. `components` are ordered by their first cell, which is also
/// `clist[0]` — the only cell `encodePzpr` looks at.
fn rooms(b: &Board) -> Vec<Vec<usize>> {
    let (cols, rows) = (b.cols(), b.rows());
    let n = cols * rows;
    let mut seen = vec![false; n];
    let mut out = Vec::new();
    for start in 0..n {
        if seen[start] || b.cells[start].ques == 7 {
            continue;
        }
        seen[start] = true;
        let mut component = vec![start];
        let mut stack = vec![start];
        while let Some(c) = stack.pop() {
            let (col, row) = (c % cols, c / cols);
            let bx = 2 * col as i32 + 1;
            let by = 2 * row as i32 + 1;
            let mut neighbours = [None; 4];
            if col > 0 {
                neighbours[0] = Some((bx - 1, by));
            }
            if col + 1 < cols {
                neighbours[1] = Some((bx + 1, by));
            }
            if row > 0 {
                neighbours[2] = Some((bx, by - 1));
            }
            if row + 1 < rows {
                neighbours[3] = Some((bx, by + 1));
            }
            for (dir, pos) in neighbours.iter().enumerate() {
                let Some((nbx, nby)) = *pos else { continue };
                let Some(border) = b.obj(Group::Border, nbx, nby) else {
                    continue;
                };
                if border.ques > 0 || border.qans > 0 {
                    continue;
                }
                let next = match dir {
                    0 => c - 1,
                    1 => c + 1,
                    2 => c - cols,
                    _ => c + cols,
                };
                if !seen[next] {
                    seen[next] = true;
                    component.push(next);
                    stack.push(next);
                }
            }
        }
        out.push(component);
    }
    out
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

/// pmemory `Board.rebuildInfo`: at least two `qnum == 1` markers.
fn normalize_markers(b: &mut Board) {
    let mut items = b.cells.iter().filter(|c| c.qnum == 1).count();
    if items < 2 && b.cells[0].qnum != 1 {
        b.cells[0].qnum = 1;
        items += 1;
    }
    if items < 2 {
        let last = b.cells.len() - 1;
        b.cells[last].qnum = 1;
    }
}

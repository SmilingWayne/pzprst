//! Pipelink Returns (pzprjs `variety/pipelink.js`, pid `pipelinkr`).

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Pipelinkr;

struct Pipelinkr;

impl Genre for Pipelinkr {
    fn pid(&self) -> &'static str {
        "pipelinkr"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_pipelink(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_pipelink(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        // `decodeDispType`: obsolete "pipe"/"circle" line, read and ignored.
        r.read_line();
        decode_pipelink_fileio(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.write_line("circle");
        encode_pipelink_fileio(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows() / 4) as i32;
        if b.cells.iter().any(|c| {
            !matches!(c.ques, -2 | 0 | 6 | 11..=17)
                || !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "pipelinkr cell value outside empty / ? / ice / clue 1-4 / line part 11-17",
            );
        }
        if b.cells.iter().any(|c| c.qnum != -1) {
            d.push(
                DiagnosticCode::LossyUrl,
                "pipelinkr clues dropped from a URL: it stores ice, line parts and ? only",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "pipelinkr border value outside line 0/1, cross -1",
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData` (pipelink.js, shared by both pids): rotate/flip the line
    /// parts stored in `cell.ques`.
    ///
    /// pzprjs calls it before the coordinates move, once per board operation.
    /// The value permutation commutes with the move, and the composed
    /// `Transform` is applied here after the move; the operations still run in
    /// `tools/gen_fixtures.js` order (rot90 = turnr, rot180 = turnr+turnr, …).
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        for op in ops(t) {
            let tques: &[(i32, i32)] = match op {
                Op::FlipY => &[(14, 17), (15, 16), (16, 15), (17, 14)],
                Op::FlipX => &[(14, 15), (15, 14), (16, 17), (17, 16)],
                Op::TurnR => &[(12, 13), (13, 12), (14, 17), (15, 14), (16, 15), (17, 16)],
                Op::TurnL => &[(12, 13), (13, 12), (14, 15), (15, 16), (16, 17), (17, 14)],
            };
            for cell in &mut b.cells {
                if let Some(&(_, to)) = tques.iter().find(|(from, _)| *from == cell.ques) {
                    cell.ques = to;
                }
            }
        }
    }
}

/// The pzprjs board operations a composed [`Transform`] stands for.
#[derive(Clone, Copy, PartialEq, Eq)]
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

/// `decodePipelink`: `.` is `?`, digits are runs of ice (one more than the
/// digit), `a`–`g` are the line parts 11–17, `h`–`z` skip cells.
///
/// pzprjs re-exposes the current character (`substr(i)`); like every existing
/// port we consume it (`substr(i + 1)`), otherwise a body that exactly fills
/// the board would report trailing data.
fn decode_pipelink(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        if ca.is_ascii_digit() {
            let max = ca.to_digit(10).unwrap() as usize + 1;
            for _ in 0..max {
                if let Some(cell) = b.cells.get_mut(c) {
                    cell.ques = 6;
                    c += 1;
                }
            }
            // pzprjs decrements `c` after the run and the loop increments it again.
            c = c.saturating_sub(1);
        } else if ('a'..='g').contains(&ca) {
            if let Some(cell) = b.cells.get_mut(c) {
                cell.ques = jsnum::digit(ca, 36).unwrap() as i32 + 1;
            }
        } else if ('h'..='z').contains(&ca) {
            c += jsnum::digit(ca, 36).unwrap() as usize - 17;
        } else if ca == '.' {
            if let Some(cell) = b.cells.get_mut(c) {
                cell.ques = -2;
            }
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
    chars[(i + 1).min(chars.len())..].iter().collect()
}

/// `encodePipelink` for `URL_PZPRV3`.
fn encode_pipelink(b: &Board) -> String {
    let n = b.cells.len();
    let mut count = 0u32;
    let mut cm = String::new();
    let mut c = 0usize;
    while c < n {
        let mut pstr = String::new();
        let qu = b.cells[c].ques;
        if qu == -2 {
            pstr.push('.');
        } else if qu == 6 {
            let mut run = 1usize;
            while run < 10 {
                if c + run >= n || b.cells[c + run].ques != 6 {
                    break;
                }
                run += 1;
            }
            pstr = (run - 1).to_string();
            c += run - 1;
        } else if (11..=17).contains(&qu) {
            pstr = jsnum::to_radix((qu - 1) as u32, 36);
        } else {
            count += 1;
        }

        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 19 {
            cm.push_str(&jsnum::to_radix(16 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
        c += 1;
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(16 + count, 36));
    }
    cm
}

/// `decodePipelink` (FileIO): `o` ice, `-` unknown, `a`–`g` line parts, `.`
/// empty, anything else a clue number.
fn decode_pipelink_fileio(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "o" {
            cell.ques = 6;
        } else if ca == "-" {
            cell.ques = -2;
        } else if ca.len() == 1 && ('a'..='g').contains(&ca.chars().next().unwrap()) {
            cell.ques = jsnum::digit(ca.chars().next().unwrap(), 36).unwrap() as i32 + 1;
        } else if ca != "." {
            cell.qnum = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodePipelink` (FileIO).
fn encode_pipelink_fileio(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.ques == 6 {
            "o ".into()
        } else if cell.ques == -2 {
            "- ".into()
        } else if (11..=17).contains(&cell.ques) {
            format!("{} ", jsnum::to_radix((cell.ques - 1) as u32, 36))
        } else if cell.qnum != -1 {
            format!("{} ", cell.qnum)
        } else {
            ". ".into()
        }
    });
}

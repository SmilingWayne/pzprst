//! Hitori / ひとりにしてくれ (pzprjs `variety/hitori.js`, pid `hitori`).
//!
//! The URL is the genre's own `decodeHitori`/`encodeHitori`: base-36 one-character
//! numbers, `-` plus two base-36 digits for 36–1295, `%`/`@` for `?` and `.` for
//! an empty cell. Cells are shaded in the answer (`qans`) and circles/lines are
//! the FileIO snapshot layers.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Hitori;

struct Hitori;

impl Genre for Hitori {
    fn pid(&self) -> &'static str {
        "hitori"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let chars: Vec<char> = body.chars().collect();
        let n = b.cells.len();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() {
            let ca = chars[i];
            let Some(cell) = b.cells.get_mut(c) else {
                break;
            };
            if ca.is_ascii_digit() || ca.is_ascii_lowercase() {
                cell.qnum = jsnum::digit(ca, 36).unwrap() as i32;
            } else if ca == '-' {
                match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 36) {
                    Some(qn) => cell.qnum = qn as i32,
                    None => d.push(
                        DiagnosticCode::UnexpectedChar,
                        "no base-36 digits after '-' leave the cell empty",
                    ),
                }
                i += 2;
            } else if ca == '%' || ca == '@' {
                cell.qnum = -2;
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
        // pzprjs's loop consumed the character at `i` before it breaks; the rest
        // of the body starts after it.
        chars[(i + 1).min(chars.len())..].iter().collect()
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        // pzprjs counts empty cells and flushes one `.` per empty cell.
        Ok(encode_hitori(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line_if_present(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = b.cols().max(b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("hitori cell value outside empty / ? / clue 1-{maxnum} or answer mark"),
            );
        }
        if b.cells
            .iter()
            .any(|c| !matches!(c.qnum, -2 | -1) && !(0..=1295).contains(&c.qnum))
        {
            d.push(
                DiagnosticCode::LossyUrl,
                "hitori numbers outside empty / ? / 0-1295 are dropped from the URL",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "hitori border value outside line 0/1, cross -1",
            );
        }
    }
}

/// `encodeHitori`: base-36 numbers, `-` plus two base-36 digits for 36–1295,
/// `@` for `?` and `.` for every cell the URL cannot store.
fn encode_hitori(b: &Board) -> String {
    let mut cm = String::new();
    for cell in &b.cells {
        let qn = cell.qnum;
        if qn == -2 {
            cm.push('@');
        } else if (0..36).contains(&qn) {
            cm.push_str(&jsnum::to_radix(qn as u32, 36));
        } else if (36..1296).contains(&qn) {
            cm.push('-');
            cm.push_str(&jsnum::to_radix(qn as u32, 36));
        } else {
            cm.push('.');
        }
    }
    cm
}

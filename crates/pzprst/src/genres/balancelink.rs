//! Balance Link (pzprjs `variety/balancelink.js`, pid `balancelink`).
//!
//! A cell stores a clue number 1-12 (`qnum`; -2 is `?`) or the black-cell mark
//! `ques = 7`. The URL packs both into one number where `0` means the black
//! cell, so a shaded cell carrying a clue is not representable in a URL.
//! Answer border lines use `line` / `qsub`.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Balancelink;

struct Balancelink;

impl Genre for Balancelink {
    fn pid(&self) -> &'static str {
        "balancelink"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        // `genericDecodeNumber16` reads the value first and only treats `-1` as
        // "no number". A `-` escape whose two digits parse to `-1` (`--1`)
        // therefore skips one character and keeps scanning, so the loop is
        // written out here rather than using the shared helper.
        let chars: Vec<char> = body.chars().collect();
        let n = b.cells.len();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() && c < n {
            let ca = chars[i];
            let (val, used) = encode::read_number16(&chars, i);
            match val {
                Some(v) if v != -1 => {
                    if v == 0 {
                        b.cells[c].ques = 7;
                    } else {
                        b.cells[c].qnum = v;
                    }
                    i += used;
                    c += 1;
                }
                // `g`-`z` skip base-36 digits minus 15 cells.
                _ if ('g'..='z').contains(&ca) => {
                    c += jsnum::digit(ca, 36).unwrap() as usize - 15;
                    i += 1;
                }
                // JavaScript's NaN: consume the escape and the cell, but leave
                // the cell empty (pzprjs stores NaN, which no output can carry).
                _ if val.is_none() => {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("escape at character {i} is not hexadecimal; the cell is left empty"),
                    );
                    i += used;
                    c += 1;
                }
                _ => {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("unexpected character {ca:?} is skipped"),
                    );
                    i += 1;
                }
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

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::generic_encode_number16(b.cells.len(), |c| {
            if b.cells[c].ques == 7 {
                0
            } else {
                b.cells[c].qnum
            }
        }))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "#" {
                cell.ques = 7;
            } else if ca != "." {
                cell.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 7 {
                "# ".into()
            } else if cell.qnum != -1 {
                format!("{} ", cell.qnum)
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (1..=12).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "balancelink cell value outside empty / ? / clue 1-12",
            );
        }
        if b.cells.iter().any(|c| c.ques != 0 && c.ques != 7) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "balancelink cell mark outside none 0 / shaded 7",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "balancelink border value outside line 0/1, cross -1",
            );
        }
        if b.cells.iter().any(|c| c.ques == 7 && c.qnum != -1) {
            d.push(
                DiagnosticCode::LossyUrl,
                "a shaded balancelink cell drops its clue from a URL",
            );
        }
    }
}

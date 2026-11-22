//! Brownies (pzprjs `variety/yosenabe.js`, pid `brownies`). `yosenabe` and
//! `yajisoko` share the script but override `Encode`/`FileIO`, so they are
//! separate pids with their own genre files.
//!
//! A cell has a dark-cell mark (`qnum = -2`, shown as `#`) or a filling number
//! in `qnum2` (-1 empty, -2 `?`, 0-8 clues); the two are exclusive. The FileIO
//! also stores the answer line layer and a packed `qsub` / `qcmp` mark.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Brownies;

struct Brownies;

/// `decodeCellQnum2`: `#` is the dark cell, a number fills `qnum2`.
fn decode_cell_qnum2(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "#" {
            cell.qnum = -2;
        } else if ca != "." {
            cell.qnum2 = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQnum2`.
fn encode_cell_qnum2(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qnum == -2 {
            "# ".into()
        } else if cell.qnum2 != -1 {
            format!("{} ", cell.qnum2)
        } else {
            ". ".into()
        }
    });
}

/// `decodeCellQsubQcmp`: one token packs `qsub` in the low nibble and `qcmp`
/// above it; `0` means both are zero.
fn decode_cell_qsub_qcmp(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca != "0" {
            let val = fileio::int(ca)?;
            cell.qsub = val & 0x0f;
            cell.qcmp = val >> 4;
        }
        Ok(())
    })
}

/// `encodeCellQsubQcmp`.
fn encode_cell_qsub_qcmp(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        format!("{} ", cell.qsub + (cell.qcmp << 4))
    });
}

impl Genre for Brownies {
    fn pid(&self) -> &'static str {
        "brownies"
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
                    if v == 9 {
                        b.cells[c].qnum = -2;
                    } else {
                        b.cells[c].qnum2 = v;
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
            let cell = &b.cells[c];
            if cell.qnum == -2 {
                9
            } else {
                cell.qnum2
            }
        }))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_cell_qnum2(r, b, d)?;
        fileio::decode_border_line(r, b, d)?;
        decode_cell_qsub_qcmp(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_qnum2(w, b);
        fileio::encode_border_line(w, b);
        encode_cell_qsub_qcmp(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| c.qnum != -1 && c.qnum != -2)
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "brownies cell value outside empty / dark cell",
            );
        }
        if b.cells
            .iter()
            .any(|c| !(c.qnum2 == -2 || c.qnum2 == -1 || (0..=8).contains(&c.qnum2)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "brownies number outside empty / ? / clue 0-8",
            );
        }
        if b.cells.iter().any(|c| !(0..=15).contains(&c.qsub)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "brownies sub mark outside 0-15",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "brownies border value outside line 0/1, cross -1",
            );
        }
        if b.cells.iter().any(|c| c.qnum == -2 && c.qnum2 != -1) {
            d.push(
                DiagnosticCode::LossyUrl,
                "a dark brownies cell drops its number from a URL",
            );
        }
    }
}

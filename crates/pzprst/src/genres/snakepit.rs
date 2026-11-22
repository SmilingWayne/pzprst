//! Snake Pit (pzprjs `variety/fillomino.js`, pid `snakepit`).
//!
//! Cells hold numbers 2–maxnum (maxnum is 3 with the `fillomino_tri` FileIO
//! flag, otherwise `cols * rows`); `ques` 1/6 are circles and `ques` 7 marks a
//! cell the URL writes as clue 0. The URL layer also stores border walls in
//! `border.ques`; FileIO stores borders and crosses in the answer layer.
//!
//! The `t` pflag is not read by `Encode@snakepit` (only the FileIO
//! `decodeConfigFlag` line is), so it is not a URL rule flag: pzprjs drops it
//! from every generated URL.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Snakepit;

struct Snakepit;

impl Genre for Snakepit {
    fn pid(&self) -> &'static str {
        "snakepit"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        // `Encode@snakepit` neither calls `checkpflag` nor sets `outpflag`; the
        // `fillomino_tri` config is only written/read as a FileIO flag line.
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = decode_number16(b, body, d);
        let rest = decode_ques(b, &rest, d);
        // `encodeBorderIfPresent` omits the whole border layer when no border
        // is a wall; an empty tail is that layer, not a truncated body.
        if rest.is_empty() {
            return rest;
        }
        encode::decode_border(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut cm = encode_number16(b);
        cm.push_str(&encode_ques(b)?);
        if b.borders.iter().any(|border| border.ques == 1) {
            cm.push_str(&encode::encode_border(b));
        }
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        // `fillomino_tri` is FileIO-only (the URL encoder never writes the
        // `t` pflag), so the rules map stays empty when the line is absent.
        if r.peek_line() == Some("t") {
            r.read_line();
            b.rules.insert("fillomino_tri", true);
        }
        fileio::decode_cell_qnum(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            cell.ques = if ca == "." { 0 } else { fileio::int(ca)? };
            Ok(())
        })?;
        fileio::decode_cell_anumsub(r, b, d)?;
        decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "t", "fillomino_tri");
        fileio::encode_cell_qnum(w, b);
        w.encode_cell(b, |cell| {
            if cell.ques >= 0 {
                format!("{} ", cell.ques)
            } else {
                ". ".into()
            }
        });
        fileio::encode_cell_anumsub(w, b);
        encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = if b.rules.get("fillomino_tri").copied().unwrap_or(false) {
            3
        } else {
            (b.cols() * b.rows()) as i32
        };
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (2..=maxnum).contains(&c.qnum))
                || !(c.ques == 0 || c.ques == 1 || c.ques == 6 || c.ques == 7)
                || !(c.anum == -1 || (2..=maxnum).contains(&c.anum))
                || !(0..=4).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!(
                    "snakepit cell value outside empty / ? / clue 2-{maxnum} or answer mark"
                ),
            );
        }
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.ques)
                || !(0..=1).contains(&e.qans)
                || !(0..=1).contains(&e.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "snakepit border value outside wall 0/1, answer 0/1 or cross 0/1",
            );
        }
    }
}

/// `decodeNumber16`: clue 0 marks an empty (`ques = 7`) cell; every other
/// value is a cell number.
fn decode_number16(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let n = b.cells.len();
    let cells = &mut b.cells;
    encode::generic_decode_number16(n, bstr, d, |c, val| {
        if val == 0 {
            cells[c].ques = 7;
        } else {
            cells[c].qnum = val;
        }
    })
}

/// `encodeNumber16`: empty (`ques = 7`) cells are written as clue 0.
fn encode_number16(b: &Board) -> String {
    encode::generic_encode_number16(b.cells.len(), |c| {
        let cell = &b.cells[c];
        if cell.ques == 7 {
            0
        } else {
            cell.qnum
        }
    })
}

/// `decodeQues`: base-3 `ques` marks, 1 shades the cell and 2 is a circle.
fn decode_ques(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let n = b.cells.len();
    let cells = &mut b.cells;
    encode::decode_three(n, bstr, d, |c, val| match val {
        1 => cells[c].ques = 1,
        2 => cells[c].ques = 6,
        _ => {}
    })
}

/// `encodeQues`.
fn encode_ques(b: &Board) -> Result<String> {
    encode::encode_three(b.cells.len(), |c| match b.cells[c].ques {
        6 => 2,
        1 => 1,
        _ => 0,
    })
}

/// `decodeBorderAns`: `-X`/`X` are walls, `2`/`1` the answer border and
/// `-1` a cross.
fn decode_border_ans(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_border(b, d, |border, ca| {
        if ca == "-X" {
            border.qsub = 1;
            border.ques = 1;
        } else if ca == "X" {
            border.ques = 1;
        } else if ca == "2" {
            border.qans = 1;
            border.qsub = 1;
        } else if ca == "1" {
            border.qans = 1;
        } else if ca == "-1" {
            border.qsub = 1;
        }
        Ok(())
    })
}

/// `encodeBorderAns`.
fn encode_border_ans(w: &mut FileWriter, b: &Board) {
    w.encode_border(b, |border| {
        if border.ques == 1 && border.qsub == 1 {
            "-X ".into()
        } else if border.ques == 1 {
            "X ".into()
        } else if border.qans == 1 && border.qsub == 1 {
            "2 ".into()
        } else if border.qans == 1 {
            "1 ".into()
        } else if border.qsub == 1 {
            "-1 ".into()
        } else {
            "0 ".into()
        }
    });
}

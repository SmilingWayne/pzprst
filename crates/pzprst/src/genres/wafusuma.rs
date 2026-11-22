//! Wafusuma (pzprjs `variety/fillomino.js`, pid `wafusuma`).
//!
//! The URL stores one number per border (how many cells of the enclosed area
//! the clue covers) and then the cell numbers. `fillomino_tri` is a FileIO-only
//! config: `Encode@wafusuma,topo` never reads or writes a URL pflag for it.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Wafusuma;

struct Wafusuma;

impl Genre for Wafusuma {
    fn pid(&self) -> &'static str {
        "wafusuma"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let borders = &mut b.borders;
        let rest = encode::generic_decode_number16(borders.len(), body, d, |i, val| {
            borders[i].qnum = val;
        });
        // The cell layer is omitted when every cell is empty, so an empty rest
        // is not a truncated body (pzprjs's decoder is a no-op on it).
        if rest.is_empty() {
            String::new()
        } else {
            encode::decode_number16(b, &rest, d)
        }
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut out = encode::generic_encode_number16(b.borders.len(), |i| b.borders[i].qnum);
        if b.cells.iter().any(|cell| cell.qnum != -1) {
            out.push_str(&encode::encode_number16(b));
        }
        Ok(out)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        // `decodeConfigFlag("t", "fillomino_tri")`: the flag is FileIO-only, and
        // its default false is a missing key in `rules` (like `Board::empty`).
        if r.peek_line() == Some("t") {
            r.read_line();
            b.rules.insert("fillomino_tri", true);
        }
        r.decode_border(b, d, |border, ca| {
            if ca != "." {
                border.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        if r.filever == 1 {
            fileio::decode_cell_qnum(r, b, d)?;
        }
        fileio::decode_cell_anumsub(r, b, d)?;
        decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "t", "fillomino_tri");
        w.filever = 1;
        w.encode_border(b, |border| {
            if border.qnum == -1 {
                ". ".into()
            } else {
                format!("{} ", border.qnum)
            }
        });
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell@wafusuma` keeps the shared `Cell.maxnum`: cols*rows, or 3 when
        // `fillomino_tri` is on.
        let tri = b.rules.get("fillomino_tri").copied().unwrap_or(false);
        let maxnum = if tri {
            3
        } else {
            (b.cols() * b.rows()) as i32
        };
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || c.anum < -1
                || !(0..=1).contains(&c.qans)
                || !(0..=4).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
                || !(0..=1).contains(&c.line)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("wafusuma cell value outside empty / ? / clue 1-{maxnum} or answer mark"),
            );
        }
        let bordermax = (b.cols() * b.rows()) as i32;
        if b.borders.iter().any(|e| {
            !(e.qnum == -1 || e.qnum == -2 || (3..=bordermax).contains(&e.qnum))
                || !(0..=1).contains(&e.ques)
                || !(0..=1).contains(&e.qans)
                || !(0..=1).contains(&e.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("wafusuma border value outside empty / ? / clue 3-{bordermax} or answer mark"),
            );
        }
    }
}

/// `decodeBorderAns`: `-X` marks a border and its cross, `X` only the border,
/// `2`/`1` the answer line (with cross), `-1` only the cross.
fn decode_border_ans(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
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

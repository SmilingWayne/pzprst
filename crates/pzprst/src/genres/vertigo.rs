//! Vertigo (pzprjs `variety/simpleloop.js`, pid `vertigo`).
//!
//! The URL is the shared `decodeEmpty` shaded-cell layer (no pflag). Vertigo
//! overrides `FileIO@vertigo`: its cell layer writes `*` for a shaded cell, and
//! the answer layer is `decodeBorderArrowAns` (border line plus a `u`/`d`/`l`/`r`
//! aux mark), unlike Simple Loop's `decodeBorderLine`.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Vertigo;

struct Vertigo;

impl Genre for Vertigo {
    fn pid(&self) -> &'static str {
        "vertigo"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_empty(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_empty(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_empty(r, b, d)?;
        decode_border_arrow_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_empty(w, b);
        encode_border_arrow_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| !(c.ques == 0 || c.ques == 7)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "vertigo cell value outside empty 0 / shaded 7",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !matches!(e.qsub, 0 | 2 | 11..=14))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "vertigo border value outside line 0/1 or aux mark 0/cross/u/d/l/r",
            );
        }
    }
}

/// `decodeEmpty`: `*` marks a shaded cell.
fn decode_empty(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "*" {
            cell.ques = 7;
        }
        Ok(())
    })
}

/// `encodeEmpty`: `* ` for a shaded cell, `. ` otherwise.
fn encode_empty(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.ques == 7 {
            "* ".into()
        } else {
            ". ".into()
        }
    });
}

/// `decodeBorderArrowAns`: border line, or `-n` for a line plus cross
/// (`qsub` 2), with an optional trailing `u`/`d`/`l`/`r` aux mark (`qsub`
/// 11–14). Any other lowercase letter is stripped without setting a mark.
fn decode_border_arrow_ans(
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
fn encode_border_arrow_ans(w: &mut FileWriter, b: &Board) {
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

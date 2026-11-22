//! Train Stations (pzprjs `variety/trainstations.js`, pid `trainstations`).
//!
//! The URL is the `decodeNumber16` clue layer followed by the `decodeEmpty`
//! shaded-cell layer; `f` is the `loop_full` rule. FileIO stores the same two
//! question layers as one cell layer (`#` shaded, `-` for `?`, numbers) and then
//! the answer layer of border lines with directional aux marks
//! (`decodeBorderArrowAns`). `turnrun` shares the file (`Cell@turnrun` only
//! changes `maxnum`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &TrainStations;

struct TrainStations;

impl Genre for TrainStations {
    fn pid(&self) -> &'static str {
        "trainstations"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('f', "loop_full")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_number16(b, body, d);
        encode::decode_empty(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_number16(b),
            encode::encode_empty(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "f", "loop_full");
        r.decode_cell(b, d, |cell, ca| {
            if ca == "#" {
                cell.ques = 7;
            } else if ca == "-" {
                cell.qnum = -2;
            } else if ca != "." {
                cell.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        decode_border_arrow_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "f", "loop_full");
        w.encode_cell(b, |cell| {
            if cell.ques == 7 {
                "# ".into()
            } else if cell.qnum >= 0 {
                format!("{} ", cell.qnum)
            } else if cell.qnum == -2 {
                "- ".into()
            } else {
                ". ".into()
            }
        });
        encode_border_arrow_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum`: `board.cols * board.rows`; `Cell.minnum` 0.
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.ques == 0 || c.ques == 7)
                || !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("trainstations cell value outside shaded / empty / ? / clue 0-{maxnum}"),
            );
        }
        if b.borders
            .iter()
            .any(|e| e.line < 0 || !matches!(e.qsub, 0 | 2 | 11..=14))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "trainstations border value outside line >= 0 or aux mark 0/cross/u/d/l/r",
            );
        }
    }
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

//! Mid-loop (pzprjs `variety/midloop.js`, pid `midloop`).
//!
//! The URL stores dot values on cells, borders and crosses; FileIO stores the
//! same dots plus the line layer. A dot's piece is its cell, border or cross,
//! and midloop's `Dot.setDot` ignores the value 2 on a cross.

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Midloop;

struct Midloop;

/// pzprjs midloop `Dot.setDot`: a cross never takes the value 2.
fn set_dot(dot: &mut crate::Obj, group: Group, val: i32) {
    if group == Group::Cross && val == 2 {
        return;
    }
    dot.qnum = val;
}

impl Genre for Midloop {
    fn pid(&self) -> &'static str {
        "midloop"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('f', "loop_full")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_dot(b, body, d, set_dot)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_dot(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "f", "loop_full");
        fileio::decode_dot_file(r, b, d, set_dot)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "f", "loop_full");
        fileio::encode_dot_file(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let dots = || b.cells.iter().chain(&b.borders).chain(&b.crosses);
        if dots().any(|dot| {
            !matches!(dot.qnum, -2..=9) || !matches!(dot.ques, 0 | 7)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "midloop dot value outside empty / ? / 1-9, or X mark outside 0/7",
            );
        }
        if dots().any(|dot| dot.qnum == -2 || dot.ques == 7) {
            d.push(
                DiagnosticCode::LossyUrl,
                "midloop '?' dots and X marks are dropped from the URL",
            );
        }
        if b.borders
            .iter()
            .any(|border| border.line < 0 || !matches!(border.qsub, 0 | 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "midloop border line outside 0.. or sub mark outside 0/-1",
            );
        }
    }
}

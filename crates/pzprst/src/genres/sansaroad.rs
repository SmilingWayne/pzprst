//! Sansa Road (pzprjs `variety/kurarin.js`, pid `sansaroad`).
//!
//! Same dot model as kurarin: values on cells, borders and crosses, two dots
//! per `genericDecodeNumber16` slot in the URL. Sansa Road's cell dots stop at
//! 1 and its FileIO has no border line layer (`if (this.pid === "kurarin")`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &SansaRoad;

struct SansaRoad;

/// Number of pzprjs `hasdots === 1` dots: `(2*cols-1) * (2*rows-1)`.
fn dot_count(b: &Board) -> usize {
    (2 * b.cols() - 1) * (2 * b.rows() - 1)
}

/// Position of flat dot index `s` in pzprjs's `hasdots === 1` dot grid.
fn dot_point(b: &Board, s: usize) -> (i32, i32) {
    let width = 2 * b.cols() - 1;
    ((s % width) as i32 + 1, (s / width) as i32 + 1)
}

/// `Dot.getDot` for flat dot index `s`; pzprst's empty dot (`-1`) reads as 0.
fn get_dot(b: &Board, s: usize) -> i32 {
    let (bx, by) = dot_point(b, s);
    match b.dot(bx, by).map_or(-1, |dot| dot.qnum) {
        -1 => 0,
        qnum => qnum,
    }
}

/// `Dot.setDot` for flat dot index `s`: pzprst's empty dot is `-1`, not 0.
fn set_dot(b: &mut Board, s: usize, val: i32) {
    let (bx, by) = dot_point(b, s);
    if let Some(dot) = b.dot_mut(bx, by) {
        dot.qnum = if val == 0 { -1 } else { val };
    }
}

/// `decodeKurarin` for sansaroad: two bits per dot, two dots per slot.
fn decode_sansaroad(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let dotsmax = dot_count(b);
    let length = (dotsmax + 1) >> 1;
    encode::generic_decode_number16(length, bstr, d, |idx, val| {
        set_dot(b, idx * 2, (val >> 2) & 3);
        if idx * 2 + 1 < dotsmax {
            set_dot(b, idx * 2 + 1, val & 3);
        }
    })
}

/// `encodeKurarin` for sansaroad: the inverse of [`decode_sansaroad`].
fn encode_sansaroad(b: &Board) -> String {
    let dotsmax = dot_count(b);
    let mut values = Vec::with_capacity(dotsmax);
    values.extend((0..dotsmax).map(|s| get_dot(b, s)));
    encode::generic_encode_number16((dotsmax + 1) >> 1, |idx| {
        let dot2 = values.get(idx * 2 + 1).copied().unwrap_or(0);
        let value = (values[idx * 2] << 2) | dot2;
        if value > 0 {
            value
        } else {
            -1
        }
    })
}

impl Genre for SansaRoad {
    fn pid(&self) -> &'static str {
        "sansaroad"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_sansaroad(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_sansaroad(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_dot_file(r, b, d, |dot, _, val| dot.qnum = val)?;
        fileio::decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_dot_file(w, b);
        fileio::encode_cell_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let dots = || b.cells.iter().chain(&b.borders).chain(&b.crosses);
        if dots().any(|dot| !matches!(dot.qnum, -2..=9) || !matches!(dot.ques, 0 | 7)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "sansaroad dot value outside empty / ? / 1-9, or X mark outside 0/7",
            );
        }
        if dots().any(|dot| dot.qnum == -2 || dot.ques == 7) {
            d.push(
                DiagnosticCode::LossyUrl,
                "sansaroad '?' dots and X marks are dropped from the URL",
            );
        }
    }
}

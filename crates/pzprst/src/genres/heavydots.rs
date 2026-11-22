//! Heavy Dots (pzprjs `variety/heavydots.js`, pid `heavydots`).
//!
//! The question data is a grid of cell weights (`Cell.maxnum = cols * rows`,
//! `minnum` 1) plus black (`qnum` 3) and white (`qnum` 4) dots on the cross
//! grid (`Board.hascross` 2). The URL stores both: `decodeNumber16` over the
//! cells, then `decodeDots` (base-3 triples over `bd.cross`). FileIO adds no
//! config flags and no `filever` branch; its border layer is answer data only.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Heavydots;

struct Heavydots;

impl Genre for Heavydots {
    fn pid(&self) -> &'static str {
        "heavydots"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_number16(b, body, d);
        decode_dots(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut out = encode::encode_number16(b);
        out.push_str(&encode_dots(b)?);
        Ok(out)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        r.decode_cross(b, d, false, |cross, ca| {
            if ca != "." {
                cross.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        w.encode_cross(b, false, |cross| {
            let ca = if cross.qnum > 0 {
                cross.qnum.to_string()
            } else {
                ".".into()
            };
            format!("{ca} ")
        });
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -2 || c.qnum == -1 || (1..=maxnum).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("heavydots cell value outside empty / ? / weight 1-{maxnum}"),
            );
        }
        if b.crosses.iter().any(|c| !matches!(c.qnum, -2 | -1 | 3 | 4)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "heavydots cross value outside empty / ? / black 3 / white 4",
            );
        }
        if b.crosses.iter().any(|c| c.qnum == -2) {
            d.push(
                DiagnosticCode::LossyUrl,
                "an unknown-colour dot is dropped from a heavydots URL",
            );
        }
    }
}

/// `decodeDots`: base-3 values over every cross, `1` → black dot (`qnum` 3),
/// `2` → white dot (`qnum` 4) and `0` stays empty.
fn decode_dots(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let n = b.crosses.len();
    let crosses = &mut b.crosses;
    encode::decode_triple(n, bstr, d, |c, val| crosses[c].qnum = val + 2)
}

/// `encodeDots`: black dots are `1` and white dots `2`; every other cross
/// value becomes `0`, so the value passed to `encodeTriple` is always 0–2.
fn encode_dots(b: &Board) -> Result<String> {
    encode::encode_triple(b.crosses.len(), |c| match b.crosses[c].qnum {
        4 => 2,
        3 => 1,
        _ => 0,
    })
}

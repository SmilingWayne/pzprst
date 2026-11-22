//! Goats and Wolves / ヤギとオオカミ (pzprjs `variety/kramma.js`, pid `shwolf`).
//!
//! Same board model and `Encode@kramman,shwolf` sections as `kramman`: the
//! cross-mark layer (a cross with `qnum = 1` is a black dot) comes before the
//! circles, and a cell `qnum` of 1 is a goat, 2 a wolf.
//!
//! FileIO stores cell numbers, cross numbers and border answers, like `kramma`.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Shwolf;

struct Shwolf;

impl Genre for Shwolf {
    fn pid(&self) -> &'static str {
        "shwolf"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    /// pzprjs `Board.hascross: 1` (only the inner crosses are usable).
    fn hascross(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        // `Encode@kramman,shwolf`: `decodeCrossMark(); decodeCircle();`
        let rest = encode::decode_cross_mark(b, body, d);
        encode::decode_circle(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        // `Encode@kramman,shwolf`: `encodeCrossMark(); encodeCircle();`
        let mut cm = encode::encode_cross_mark(b);
        cm.push_str(&encode::encode_circle(b)?);
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cross_num(r, b, d)?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cross_num(w, b);
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cell.maxnum: 2` (minnum stays 1); `Cross` keeps the default
        // 1..999. `?` (`-2`) is possible in both layers.
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=2).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "shwolf cell value outside goat/wolf 1/2, empty/? or answers 0/1",
            );
        }
        if b.crosses
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (1..=999).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "shwolf cross value outside 1-999, empty or ?",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "shwolf border value outside line 0/1 or cross 0/1",
            );
        }
        // `encodeCircle` stores `max(0, qnum)`, so a `?` cell becomes empty.
        if b.cells.iter().any(|c| c.qnum == -2) {
            d.push(
                DiagnosticCode::LossyUrl,
                "a shwolf URL stores a `?` cell as an empty cell",
            );
        }
        // `encodeCrossMark` writes a dot only for `qnum === 1`.
        if b.crosses
            .iter()
            .any(|c| c.qnum != -1 && c.qnum != 1)
        {
            d.push(
                DiagnosticCode::LossyUrl,
                "a shwolf URL stores only a black dot (1) on a cross",
            );
        }
    }
}

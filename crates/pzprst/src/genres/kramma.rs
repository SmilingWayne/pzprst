//! Kaito Ramma / 快刀乱麻 (pzprjs `variety/kramma.js`, pid `kramma`).
//!
//! `kramman` and `shwolf` share the script but override `Encode` and use the
//! `kramman`/`shwolf` pids, so they have their own genre files.
//!
//! A cell stores a circle number (1 or 2) in `qnum`; cross marks are dots
//! (`qnum = 1`) on the inner cross grid, and border answers are lines
//! (`qans`) plus cross marks (`qsub`).
//!
//! pzprjs `encodePzpr` always sets `outpflag = "c"`, so every canonical kramma
//! URL carries the `c` flag and therefore never stores cross marks; a URL
//! written without `c` still decodes the cross-mark layer first.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Kramma;

struct Kramma;

/// Rule key for the URL flag `c`: cross marks are absent from the body. The
/// flag is checked by pzprjs `checkpflag("c")` and always written by
/// `encodePzpr`, even though pzprjs has no FileIO config for it.
const NO_CROSS_MARK: &str = "kramma_no_cross_mark";

impl Genre for Kramma {
    fn pid(&self) -> &'static str {
        "kramma"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    /// pzprjs `Board.hascross: 1` (only the inner crosses are usable).
    fn hascross(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('c', NO_CROSS_MARK)]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        // `decodePzpr`: `if (!this.checkpflag("c")) { this.decodeCrossMark(); } this.decodeCircle();`
        let rest = if b.rules.get(NO_CROSS_MARK).copied().unwrap_or(false) {
            body.to_string()
        } else {
            encode::decode_cross_mark(b, body, d)
        };
        let rest = encode::decode_circle(b, &rest, d);
        // `encodePzpr` writes the `c` flag unconditionally, so the canonical URL
        // of the board just decoded omits the cross marks again.
        b.rules.insert(NO_CROSS_MARK, true);
        rest
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        // `encodePzpr`: `this.outpflag = "c"; this.encodeCircle();`
        encode::encode_circle(b)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cross_num(r, b, d)?;
        fileio::decode_border_ans(r, b, d)?;
        b.rules.insert(NO_CROSS_MARK, true);
        Ok(())
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
                "kramma cell value outside circle 1/2, empty/? or answers 0/1",
            );
        }
        if b.crosses
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (1..=999).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kramma cross value outside 1-999, empty or ?",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kramma border value outside line 0/1 or cross 0/1",
            );
        }
        // `encodeCircle` stores `max(0, qnum)`, so a `?` circle becomes empty,
        // and `encodePzpr` never writes the cross-mark layer at all.
        if b.cells.iter().any(|c| c.qnum == -2) {
            d.push(
                DiagnosticCode::LossyUrl,
                "a kramma URL stores a `?` circle as an empty cell",
            );
        }
        if b.crosses.iter().any(|c| c.qnum != -1) {
            d.push(
                DiagnosticCode::LossyUrl,
                "a kramma URL has no cross-mark layer; every cross value is dropped",
            );
        }
    }
}

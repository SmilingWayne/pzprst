//! Time Bomb (pzprjs `variety/bonsan.js`, pid `timebomb`). Cells hold numbers
//! up to the cell count (capped at 999); the URL stores only cell numbers,
//! FileIO adds the `qsub`/`qcmp`/`anum` marks and the border line layer.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Timebomb;

struct Timebomb;

impl Genre for Timebomb {
    fn pid(&self) -> &'static str {
        "timebomb"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        decode_cell_qsub_qcmp(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        encode_cell_qsub_qcmp(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cell@timebomb.maxnum`: every cell, independent of position.
        let maxnum = (b.cells.len() as i32 - 1).min(999);
        if b.cells.iter().any(|c| {
            !(c.qnum == -2 || c.qnum == -1 || (0..=maxnum).contains(&c.qnum))
                || !(0..=15).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
                || !matches!(c.anum, -1 | 0)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("timebomb cell value outside 0-{maxnum} / ? / empty, or packed marks"),
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "timebomb border value outside line 0/1, cross -1",
            );
        }
    }
}

/// `decodeCellQsubQcmp`: `qsub` in the low nibble, `qcmp` in bit 4 and
/// `anum == 0` in bit 5.
fn decode_cell_qsub_qcmp(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca != "0" {
            let n = fileio::int(ca)?;
            cell.qsub = n & 0x0f;
            cell.qcmp = (n >> 4) & 1;
            cell.anum = if n >> 5 != 0 { 0 } else { -1 };
        }
        Ok(())
    })
}

/// `encodeCellQsubQcmp`.
fn encode_cell_qsub_qcmp(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        let n = cell.qsub + (cell.qcmp << 4) + if cell.anum == 0 { 1 << 5 } else { 0 };
        format!("{n} ")
    });
}

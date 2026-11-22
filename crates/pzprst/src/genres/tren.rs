//! Tren (pzprjs `variety/tren.js`, pid `tren`).
//!
//! Numbers 0-`max(cols, rows) - 2` mark train cells; the trains themselves are
//! answer data (`border.qans` for the sleepers, cell background marks). The URL
//! stores only the numbers with `decodeNumber16` plus the `n` flag
//! (`tren_new`); FileIO stores the flag line, numbers, border answers and cell
//! marks.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Tren;

struct Tren;

impl Genre for Tren {
    fn pid(&self) -> &'static str {
        "tren"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('n', "tren_new")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "n", "tren_new");
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_border_ans(r, b, d)?;
        fileio::decode_cell_qanssub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "n", "tren_new");
        fileio::encode_cell_qnum(w, b);
        fileio::encode_border_ans(w, b);
        fileio::encode_cell_qanssub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = b.cols().max(b.rows()) as i32 - 2;
        for cell in &b.cells {
            if !matches!(cell.qnum, -2 | -1) && !(0..=maxnum).contains(&cell.qnum) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    format!("tren cell value outside empty / ? / number 0-{maxnum}"),
                );
            }
            if !(0..=1).contains(&cell.qans) || !(0..=4).contains(&cell.qsub) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    "tren cell answer value outside shading 0/1, background 0-4",
                );
            }
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tren border answer value outside border 0/1, cross 0/1",
            );
        }
    }
}

//! Bhai Bahan (pzprjs `variety/bhaibahan.js`, pid `bhaibahan`). A single loop
//! through the numbered circles; each clue counts the length of the straight
//! walk through it. Borders are `hasborder` 1; cell backgrounds (`qsub` 1/2)
//! and the generic answer shade (`qans`) are stored by FileIO only.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Bhaibahan;

struct Bhaibahan;

impl Genre for Bhaibahan {
    fn pid(&self) -> &'static str {
        "bhaibahan"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('f', "loop_full")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "f", "loop_full");
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_qanssub(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "f", "loop_full");
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_qanssub(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() * b.rows()) as i32;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=2).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "bhaibahan cell value outside clue 1..cols*rows / ? or background 0-2",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "bhaibahan border value outside line 0/1, cross -1",
            );
        }
    }
}

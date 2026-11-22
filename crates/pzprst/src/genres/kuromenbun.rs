//! Kuromenbun / クロメンブン (pzprjs `variety/kuromenbun.js`, pid `kuromenbun`).
//!
//! Walls are `border.ques` 1 (URL `decodeBorder`/`encodeBorder`), cell clues are
//! `cell.qnum` (`decodeNumber16`/`encodeNumber16`), and FileIO adds the answer
//! layers `cell.qans`/`cell.qsub`.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Kuromenbun;

struct Kuromenbun;

impl Genre for Kuromenbun {
    fn pid(&self) -> &'static str {
        "kuromenbun"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        encode::decode_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_border(b),
            encode::encode_number16(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_border_ques(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_border_ques(w, b);
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum: Math.round(cols * rows / 1.5)`, `minnum: 0`.
        let maxnum = ((b.cols() * b.rows()) as f64 / 1.5).round() as i32;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("kuromenbun cell value outside empty / ? / clue 0-{maxnum} or answer mark"),
            );
        }
        if b.borders.iter().any(|e| !(0..=1).contains(&e.ques)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kuromenbun border value outside wall 0/1",
            );
        }
    }
}

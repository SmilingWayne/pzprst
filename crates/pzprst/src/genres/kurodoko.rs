//! Kurodoko (pzprjs `variety/kurodoko.js`, pid `kurodoko`). The FileIO of the
//! shared script stores a border line layer for `kurodoko`/`cave` (the "peke"
//! subline marks); `cityspace` and `teri` are separate genres.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Kurodoko;

struct Kurodoko;

impl Genre for Kurodoko {
    fn pid(&self) -> &'static str {
        "kurodoko"
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
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line_if_present(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = b.cols() as i32 + b.rows() as i32 - 1;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (2..=maxnum).contains(&c.qnum))
                || c.ques != 0
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
                || c.qcmp != 0
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kurodoko cell value outside clue 2..cols+rows-1, ? or answer mark",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !matches!(e.qsub, 0 | 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kurodoko border value outside line 0/1 or cross -1",
            );
        }
    }
}

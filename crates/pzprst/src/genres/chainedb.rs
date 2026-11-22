//! Chained Block (pzprjs `variety/chainedb.js`, pid `chainedb`). One number
//! per cell; `Cell.maxnum` is `cols * rows - 3`. FileIO stores the clue and the
//! shading layers (`#` shaded, `+` sub mark) that the URL never carries. pzprjs
//! `hascross` 2 draws 2x2 block corners only; nothing in URL or FileIO reads or
//! writes them.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Chainedb;

struct Chainedb;

impl Genre for Chainedb {
    fn pid(&self) -> &'static str {
        "chainedb"
    }

    fn hasborder(&self) -> u8 {
        0
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
        fileio::decode_cell_qnum_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum`: this.board.cols * this.board.rows - 3.
        let maxnum = (b.cols() * b.rows()) as i32 - 3;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("chainedb cell value outside clue 1-{maxnum}/? or the shading layers"),
            );
        }
    }
}

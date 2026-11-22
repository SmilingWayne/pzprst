//! City Space (pzprjs `variety/kurodoko.js`, pid `cityspace`). The FileIO of the
//! shared script stores border marks for `teri`/`kurodoko`/`cave` only, so
//! cityspace holds cell data alone. `hasborder` is pzprjs's "for pekes" flag and
//! is not part of either format.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Cityspace;

struct Cityspace;

impl Genre for Cityspace {
    fn pid(&self) -> &'static str {
        "cityspace"
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
        fileio::decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_ans(w, b);
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
                "cityspace cell value outside clue 2..cols+rows-1, ? or answer mark",
            );
        }
    }
}

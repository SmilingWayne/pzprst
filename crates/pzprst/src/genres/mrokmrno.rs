//! Mr. OK and Mr. NO / OKさんとNOくん (pzprjs `variety/mrokmrno.js`, pid
//! `mrokmrno`). Clues are `cell.qnum`; the answer uses `cell.anum` (including -3
//! for the dot mark) plus the `qcmp`/`qsub` marks. `enableSubNumberArray` is on
//! in pzprjs, but a board only carries `snum` arrays through `[...]` FileIO
//! tokens, which `Obj` does not model.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Mrokmrno;

struct Mrokmrno;

impl Genre for Mrokmrno {
    fn pid(&self) -> &'static str {
        "mrokmrno"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number10(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        encode::encode_number10(b)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=6).contains(&c.qnum))
                || !(c.anum == -3 || c.anum == -1 || (0..=6).contains(&c.anum))
                || !(0..=1).contains(&c.qcmp)
                || !(0..=4).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "mrokmrno cell value outside clue 0-6 / ? / empty or answer mark",
            );
        }
    }
}

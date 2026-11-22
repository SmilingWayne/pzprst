//! Nurimisaki (pzprjs `variety/kurodoko.js`, pid `nurimisaki`). Cells hold
//! numbers 2..cols+rows-1; the border infrastructure exists only for the
//! (answer-only) shading display, so no border layer is stored.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Nurimisaki;

struct Nurimisaki;

impl Genre for Nurimisaki {
    fn pid(&self) -> &'static str {
        "nurimisaki"
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
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("nurimisaki cell value outside clue 2-{maxnum} / ? or answer mark"),
            );
        }
    }
}

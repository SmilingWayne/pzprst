//! Scrin (pzprjs `variety/scrin.js`, pid `scrin`; `antmill` shares the file
//! with its own encoders). A cell holds a clue number in `qnum` (`?` is -2) and
//! answer shading in `qans`; the URL is the shared `decodeNumber16` /
//! `encodeNumber16` layer and FileIO stores `decodeCellQnum` followed by
//! `decodeCellAns`. `Board.hascross` is 1 and `hasborder` 2, but neither the URL
//! nor FileIO carries border or cross data. `BoardExec.adjustBoardData2` only
//! rebuilds the (unencoded) room objects, so transforms change no board value.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Scrin;

struct Scrin;

impl Genre for Scrin {
    fn pid(&self) -> &'static str {
        "scrin"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn hascross(&self) -> u8 {
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
        // `Cell@scrin.maxnum`: `(cols - 2) * (rows - 2)`; `minnum` 1.
        let maxnum = (b.cols() as i32 - 2) * (b.rows() as i32 - 2);
        if b.cells.iter().any(|c| {
            !(c.qnum == -2 || c.qnum == -1 || (1..=maxnum).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("scrin cell value outside clue 1-{maxnum}/?, or answer mark 0/1"),
            );
        }
    }
}

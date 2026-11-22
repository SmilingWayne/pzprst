//! Key West (pzprjs `variety/keywest.js`, pid `keywest`). Cell clues 0–4 are
//! encoded with the shared `4Cell` codec; FileIO stores question numbers,
//! border lines and the answer layer.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Keywest;

struct Keywest;

impl Genre for Keywest {
    fn pid(&self) -> &'static str {
        "keywest"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_4cell(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_4cell(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_border_line(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_border_line(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=4).contains(&c.qnum))
                || !(c.anum == -1 || (0..=4).contains(&c.anum))
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "keywest cell value outside empty / ? / 0..4",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !matches!(e.qsub, 0 | 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "keywest border value outside line 0/1 or cross -1",
            );
        }
    }
}

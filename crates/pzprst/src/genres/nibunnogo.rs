//! Nibun-nogo (pzprjs `variety/lightshadow.js`, pid `nibunnogo`). The script is
//! shared with `lightshadow`; `"Encode@nibunnogo"` and `"FileIO@nibunnogo"`
//! override the shared sections, so this pid only stores cross numbers and the
//! three-state cell answer.
//!
//! The URL body is `encode4Cross` (base-16 `0`–`4` cross numbers with base-36
//! run-length blanks). FileIO stores `decodeCrossNum` (cross clue or `?`)
//! followed by the cell answer layer (`decodeCell`: pzprjs writes `cell.qans`
//! verbatim, 0 empty / 1 shaded / 2 dot).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Nibunnogo;

struct Nibunnogo;

impl Genre for Nibunnogo {
    fn pid(&self) -> &'static str {
        "nibunnogo"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_4cross(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_4cross(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cross_num(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            cell.qans = fileio::int(ca)?;
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cross_num(w, b);
        w.encode_cell(b, |cell| format!("{} ", cell.qans));
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cross@nibunnogo: { maxnum: 4, minnum: 0 }`; -1 is empty and
        // -2 is `?`. The answer layer stores `cell.qans` as-is: 0 empty,
        // 1 shaded, 2 dot (`Cell.isDot`).
        if b.crosses
            .iter()
            .any(|cross| !(cross.qnum == -1 || cross.qnum == -2 || (0..=4).contains(&cross.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "nibunnogo cross value outside 0-4 / ?",
            );
        }
        if b.cells.iter().any(|cell| !(0..=2).contains(&cell.qans)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "nibunnogo cell answer outside empty / shaded / dot 0-2",
            );
        }
    }
}

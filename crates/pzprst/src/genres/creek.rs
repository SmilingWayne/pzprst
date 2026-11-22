//! Creek (pzprjs `variety/creek.js`, pid `creek`): cross clues 0–4 on the
//! `(cols + 1) x (rows + 1)` cross grid, shading as answer cells.
//!
//! The URL body is `encode4Cross` (base-16 `0`–`4` clues with base-36
//! run-length blanks), except that the pzprv3 flag `d` selects the old
//! `decodecross_old` format: one base-10 `0`–`4` per cross, everything else
//! ignored. pzprjs never writes that flag back (`encodePzpr` always calls
//! `encode4Cross`), so `decode_url` clears it to keep the canonical URL
//! free of it; this is the only URL flag creek has.
//!
//! FileIO stores `decodeCrossNum` (cross clue or `?`) followed by
//! `decodeCellAns` (`#` shade, `+` background mark).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Creek;

/// pzprjs `Board.rules` key for the decode-only `d` URL flag (no pzprjs config
/// of that name exists; the flag is never encoded).
const OLD_FLAG: &str = "decodecross_old";

struct Creek;

impl Genre for Creek {
    fn pid(&self) -> &'static str {
        "creek"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('d', OLD_FLAG)]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let old = b.rules.get(OLD_FLAG).copied().unwrap_or(false);
        b.rules.insert(OLD_FLAG, false);
        if old {
            encode::decode_cross_old(b, body, d)
        } else {
            encode::decode_4cross(b, body, d)
        }
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_4cross(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cross_num(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cross_num(w, b);
        fileio::encode_cell_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cross: { maxnum: 4, minnum: 0 }`; -1 is empty and -2 is `?`.
        if b.crosses
            .iter()
            .any(|cross| !(cross.qnum == -1 || cross.qnum == -2 || (0..=4).contains(&cross.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "creek cross value outside 0-4 / ?",
            );
        }
        if b.cells
            .iter()
            .any(|cell| !(0..=1).contains(&cell.qans) || !(0..=1).contains(&cell.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "creek cell value outside shade / background mark 0/1",
            );
        }
    }
}

//! Hashiwokakero / Hashikake (pzprjs `variety/hashikake.js`, pid `hashikake`,
//! URL id `hashi`). Bridges are border lines 0/1/2.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Hashikake;

struct Hashikake;

impl Genre for Hashikake {
    fn pid(&self) -> &'static str {
        "hashikake"
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
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=8).contains(&c.qnum))
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "hashikake cell value outside clue 1-8 / ?",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=2).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "hashikake border value outside line 0/1/2 or cross -1",
            );
        }
    }
}

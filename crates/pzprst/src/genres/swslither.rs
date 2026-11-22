//! Sheep and Wolves Slitherlink (pzprjs `variety/slither.js`, pid `swslither`).
//!
//! The same board as Slitherlink (`hasborder` 2, `f` → `slither_full`, FileIO
//! version 1 with the cell background layer) but the URL stores the cell clues
//! with `decodeNumber10`/`encodeNumber10` and no border layer, and
//! `Cell@swslither` raises the clue limit to 6.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Error, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Swslither;

struct Swslither;

impl Genre for Swslither {
    fn pid(&self) -> &'static str {
        "swslither"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('f', "slither_full")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number10(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        encode::encode_number10(b)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "f", "slither_full");
        match r.filever {
            1 => {
                fileio::decode_cell_qnum(r, b, d)?;
                fileio::decode_cell_qsub(r, b, d)?;
                fileio::decode_border_line(r, b, d)
            }
            0 => {
                fileio::decode_cell_qnum(r, b, d)?;
                fileio::decode_border_line(r, b, d)
            }
            v => Err(Error::UnsupportedFileVersion(format!("swslither pzprv3.{v}"))),
        }
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.filever = 1;
        w.encode_config_flag(b, "f", "slither_full");
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_qsub(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=6).contains(&c.qnum))
                || !(0..=2).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "swslither cell value outside clue 0-6/? or background 0-2",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "swslither border value outside line 0/1, cross -1",
            );
        }
    }
}

//! Slitherlink (pzprjs `variety/slither.js`, pid `slither`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Error, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Slither;

struct Slither;

impl Genre for Slither {
    fn pid(&self) -> &'static str {
        "slither"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('f', "slither_full")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_4cell(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_4cell(b))
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
            v => Err(Error::UnsupportedFileVersion(format!("slither pzprv3.{v}"))),
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
            !(c.qnum == -1 || c.qnum == -2 || (0..=4).contains(&c.qnum))
                || !(0..=2).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "slither cell value outside clue 0-4/? or background 0-2",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "slither border value outside line 0/1, cross -1",
            );
        }
    }
}

//! Masyu (pzprjs `variety/mashu.js`, pid `mashu`). Ura-masyu (`uramashu`) is a
//! volatile UI setting stored in neither URL nor FileIO, so it is not modelled.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Mashu;

struct Mashu;

impl Genre for Mashu {
    fn pid(&self) -> &'static str {
        "mashu"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('f', "loop_full")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_circle(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        encode::encode_circle(b)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "f", "loop_full");
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "f", "loop_full");
        fileio::encode_cell_qnum(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| !matches!(c.qnum, -2 | -1 | 1 | 2)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "mashu cell value outside white 1 / black 2 / ?",
            );
        }
        if b.cells.iter().any(|c| c.qnum == -2) {
            d.push(
                DiagnosticCode::LossyUrl,
                "unknown-colour circles are dropped from a mashu URL",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "mashu border value outside line 0/1, cross -1",
            );
        }
    }
}

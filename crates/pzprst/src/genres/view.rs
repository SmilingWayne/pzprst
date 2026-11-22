//! View (pzprjs `variety/sukoro.js`, pid `view`). Cells hold a clue 0 to
//! `min(999, cols + rows - 2)`; the URL is the plain `Number16` layer and
//! FileIO stores the clue plus the `CellAnumsub` answer layer. The `snum`
//! sub-number arrays that `Cell.enableSubNumberArray` enables are not modelled;
//! pzprjs writes their `[...]` suffix only when one is set.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &View;

struct View;

impl Genre for View {
    fn pid(&self) -> &'static str {
        "view"
    }

    fn hasborder(&self) -> u8 {
        0
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
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols() + b.rows()) as i32 - 2;
        let maxnum = maxnum.min(999);
        if b.cells
            .iter()
            .any(|c| !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("view cell value outside empty / ? / clue 0-{maxnum}"),
            );
        }
    }
}

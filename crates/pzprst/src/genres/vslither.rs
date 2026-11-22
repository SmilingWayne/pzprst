//! Vertex Slitherlink (pzprjs `variety/vslither.js`, pid `vslither`).
//!
//! The URL stores only the cell clues (`decode4Cell`: 0–4, `.` for `?`) with
//! URL flag `f` (`slither_full`). FileIO adds the cell question background
//! layer (`decodeCellQsub`), the answer border lines and the answer cross marks
//! (`cross.qsub`); its header stays `pzprv3` (no `filever` branch). `vslither`
//! shares the whole variety file with `tslither`: the only difference is the
//! answer checker's `Cell@…` sections, which never touch encoding.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Vslither;

struct Vslither;

impl Genre for Vslither {
    fn pid(&self) -> &'static str {
        "vslither"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    /// pzprjs `Board.hascross` (the inherited default 2: crosses exist on the
    /// even/even points of the whole board, outer frame included).
    fn hascross(&self) -> u8 {
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
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_qsub(r, b, d)?;
        fileio::decode_border_line(r, b, d)?;
        r.decode_cross(b, d, false, |cross, ca| {
            cross.qsub = fileio::int(ca)?;
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "f", "slither_full");
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_qsub(w, b);
        fileio::encode_border_line(w, b);
        w.encode_cross(b, false, |cross| format!("{} ", cross.qsub));
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=4).contains(&c.qnum))
                || !(0..=2).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "vslither cell value outside clue 0-4/? or background 0-2",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "vslither border value outside line 0/1, cross -1",
            );
        }
        if b.crosses.iter().any(|c| !(0..=2).contains(&c.qsub)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "vslither cross value outside empty / dot / cross mark",
            );
        }
    }
}

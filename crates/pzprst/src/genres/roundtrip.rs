//! Round Trip (pzprjs `variety/roundtrip.js`, pid `roundtrip`).
//!
//! The URL is the `decodeNumber16ExCell` excell layer. FileIO stores one
//! combined cell/excell layer (`decodeCellExCell`): only excells carry a clue,
//! with every cell token written as `.`, followed by the border line layer.
//! `hasexcell` 2 puts an excell on all four sides; `ExCell.maxnum` is `rows`
//! for the top and bottom rows and `cols` for the left and right columns.

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Roundtrip;

struct Roundtrip;

impl Genre for Roundtrip {
    fn pid(&self) -> &'static str {
        "roundtrip"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn hasexcell(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16_excell(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16_excell(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_excell(r, b, d, |obj, group, ca| {
            if ca == "." {
                return Ok(());
            }
            if group == Group::ExCell {
                obj.qnum = fileio::int(ca)?;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_excell(w, b, |obj, group| {
            if group == Group::ExCell && obj.qnum != -1 {
                format!("{} ", obj.qnum)
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `ExCell.maxnum`: the top/bottom rows allow `rows`, the left/right
        // columns allow `cols`; `ExCell.disInputHatena` only affects input.
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let bad = b.excells.iter().enumerate().any(|(i, ex)| {
            let (_, by) = b.geom.excell_pos(i).unwrap_or((0, 0));
            let max = if by < 0 || by > 2 * rows { rows } else { cols };
            !(-2..=max).contains(&ex.qnum)
        });
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "roundtrip excell value outside empty / ? / 0-maxnum",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "roundtrip border value outside line 0/1 or cross mark",
            );
        }
    }
}

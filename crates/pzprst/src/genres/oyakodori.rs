//! Oyakodori (pzprjs `variety/kaero.js`, pid `oyakodori`). Kaero and Armyants
//! share that script but override `Encode`/`FileIO`, so they are separate pids.
//!
//! The URL stores border walls (`decodeBorder`), then the circles as three
//! base-3 values per character (`decodeCircle`), then the shaded nests as a
//! base-32 bit layer (`decodeIce`). FileIO has one packed number per cell
//! (`FileIO@oyakodori`: bit 3 is `ques` 6, the rest is `qnum + 2`) and no
//! answer-sub layer; the border wall and line layers follow.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Oyakodori;

struct Oyakodori;

impl Genre for Oyakodori {
    fn pid(&self) -> &'static str {
        "oyakodori"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        let rest = encode::decode_circle(b, &rest, d);
        encode::decode_ice(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}{}",
            encode::encode_border(b),
            encode::encode_circle(b)?,
            encode::encode_ice(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        // `FileIO@oyakodori.decodeCellQnum`.
        r.decode_cell(b, d, |cell, ca| {
            if ca == "." {
                return Ok(());
            }
            let num = fileio::int(ca)?;
            if num & 8 != 0 {
                cell.ques = 6;
            }
            cell.qnum = (num & !8) - 2;
            Ok(())
        })?;
        // `FileIO@oyakodori.decodeCellQanssub` is empty: no answer-sub layer.
        fileio::decode_border_ques(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        // `FileIO@oyakodori.encodeCellQnum`: empty is `.`, otherwise the packed
        // number (`qnum + 2`, plus bit 3 for a shaded cell).
        w.encode_cell(b, |cell| {
            let num = (cell.qnum + 2) | if cell.ques == 6 { 8 } else { 0 };
            if num != 1 {
                format!("{num} ")
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_ques(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| !(-2..=2).contains(&c.qnum)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "oyakodori cell value outside empty / ? / circle 0-2",
            );
        }
        if b.cells.iter().any(|c| c.qnum == -2) {
            d.push(
                DiagnosticCode::LossyUrl,
                "a ? clue is dropped from an oyakodori URL",
            );
        }
        if b.cells.iter().any(|c| !matches!(c.ques, 0 | 6)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "oyakodori cell mark outside empty/shaded",
            );
        }
        if b.borders.iter().any(|e| {
            !matches!(e.ques, 0 | 1)
                || !matches!(e.line, 0 | 1)
                || !matches!(e.qsub, 0 | 2)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "oyakodori border value outside wall 0/1, line 0/1, cross -1",
            );
        }
    }
}

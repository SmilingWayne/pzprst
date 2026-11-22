//! Anglers (pzprjs `variety/anglers.js`, pid `anglers`).
//!
//! `hasborder` 2, `hasexcell` 2 (a full ring of clue cells around the grid).
//! The URL is one `genericDecodeNumber16` number per cell and excell (cells
//! first, in id order): `0` is a shaded cell, `n > 0` a clue `n - 1`, `-3` a
//! fish (`qnum` 0). FileIO stores the combined cell/excell layer
//! (`decodeCellExCell`: `#` is a shaded cell, a number a clue) followed by the
//! answer border line layer (`decodeBorderLine`).
//!
//! The excell ring moves as one group under board transforms, so pzprjs needs
//! no `BoardExec` data hook here.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Anglers;

struct Anglers;

impl Genre for Anglers {
    fn pid(&self) -> &'static str {
        "anglers"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn hasexcell(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let cells = b.cells.len();
        let length = cells + b.excells.len();
        encode::generic_decode_number16(length, body, d, |i, val| {
            let qnum = match val.cmp(&0) {
                std::cmp::Ordering::Equal => -3,
                std::cmp::Ordering::Greater => val - 1,
                std::cmp::Ordering::Less => val,
            };
            if i < cells {
                b.cells[i].qnum = qnum;
            } else {
                b.excells[i - cells].qnum = qnum;
            }
        })
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let cells = b.cells.len();
        let qnum = |i: usize| {
            let obj = if i < cells {
                &b.cells[i]
            } else {
                &b.excells[i - cells]
            };
            if obj.qnum == -3 {
                0
            } else if obj.qnum >= 0 {
                obj.qnum + 1
            } else {
                obj.qnum
            }
        };
        Ok(encode::generic_encode_number16(
            cells + b.excells.len(),
            qnum,
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_excell(r, b, d, |obj, _group, ca| {
            if ca != "." {
                obj.qnum = if ca == "#" { -3 } else { fileio::int(ca)? };
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_excell(w, b, |obj, _group| {
            if obj.qnum == -3 {
                "# ".into()
            } else if obj.qnum != -1 {
                format!("{} ", obj.qnum)
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell@anglers` / `ExCell@anglers`: `minnum` 0, `maxnum` the board area;
        // `-3` is the shaded-cell token and `-2` the URL's `?`.
        let max = (b.cols() * b.rows()) as i32;
        if b.cells
            .iter()
            .chain(&b.excells)
            .any(|o| !(-3..=max).contains(&o.qnum))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "anglers clue outside shaded -3 / empty -1 / ? / 0-maxnum",
            );
        }
        if b.borders
            .iter()
            .any(|e| e.line < 0 || !matches!(e.qsub, 0 | 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "anglers border value outside line >= 0 or aux cross mark 0/2",
            );
        }
    }
}

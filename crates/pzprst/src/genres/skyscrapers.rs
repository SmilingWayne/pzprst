//! Skyscrapers / Building Puzzle (pzprjs `variety/skyscrapers.js`, pid
//! `skyscrapers`).
//!
//! The URL is the `decodeNumber16ExCell` outer-clue layer, followed by the
//! `decodeNumber16` cell layer only when some cell holds a clue. FileIO stores
//! one combined cell/excell layer (`decodeCellExCellQnumAnumsub`): an excell is
//! its clue number, a cell with a question clue is `q<n>`, otherwise the answer
//! number. `hasexcell` 2 puts an excell on all four sides; `ExCell.maxnum` is
//! `rows` for the top/bottom rows and `cols` for the left/right columns, and
//! `Cell.maxnum` is `max(cols, rows)`.

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Skyscrapers;

struct Skyscrapers;

/// pzprjs `Cell.maxnum`.
fn maxnum(b: &Board) -> i32 {
    b.cols().max(b.rows()) as i32
}

impl Genre for Skyscrapers {
    fn pid(&self) -> &'static str {
        "skyscrapers"
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
        let rest = encode::decode_number16_excell(b, body, d);
        // pzprjs always calls `decodeNumber16`, but `encodePzpr` omits the cell
        // layer when no cell holds a clue, so an empty remainder means "no cell
        // clues", not a cell body that ends too early.
        if rest.is_empty() {
            return rest;
        }
        encode::decode_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_number16_excell(b);
        if b.cells.iter().any(|c| c.qnum != -1) {
            body.push_str(&encode::encode_number16(b));
        }
        Ok(body)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_cell_excell_qnum_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_excell_qnum_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = maxnum(b);
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
                || !(c.anum == -1 || (0..=maxnum).contains(&c.anum))
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("skyscrapers cell value outside ? / clue 0-{maxnum} or answer 0-{maxnum}"),
            );
        }
        // `ExCell.maxnum`: the top/bottom rows allow `rows`, the left/right
        // columns allow `cols`.
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        let bad = b.excells.iter().enumerate().any(|(i, ex)| {
            let (_, by) = b.geom.excell_pos(i).unwrap_or((0, 0));
            let max = if by < 0 || by > 2 * rows { rows } else { cols };
            !(ex.qnum == -1 || ex.qnum == -2 || (0..=max).contains(&ex.qnum))
        });
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "skyscrapers excell value outside empty / ? / 0-maxnum",
            );
        }
    }
}

/// `decodeCellExCellQnumAnumsub`: an excell is its clue, a cell is the question
/// clue (`q<n>`) or the answer number.
fn decode_cell_excell_qnum_anumsub(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    fileio::decode_cell_excell(r, b, d, |obj, group, ca| {
        if ca == "." {
            return Ok(());
        }
        if group == Group::ExCell {
            obj.qnum = fileio::int(ca)?;
        } else if let Some(rest) = ca.strip_prefix('q') {
            obj.qnum = fileio::int(rest)?;
        } else {
            // pzprjs splits `[...]` `snum` arrays here when the cell class
            // enables them; `Obj` has no `snum`, so only the answer number is
            // read.
            obj.anum = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellExCellQnumAnumsub`: the question clue as `q<n>` first, then the
/// answer number, `.` when the cell is empty.
fn encode_cell_excell_qnum_anumsub(w: &mut FileWriter, b: &Board) {
    fileio::encode_cell_excell(w, b, |obj, group| {
        if group == Group::ExCell {
            if obj.qnum != -1 {
                return format!("{} ", obj.qnum);
            }
        } else {
            if obj.qnum != -1 {
                return format!("q{} ", obj.qnum);
            }
            if obj.anum != -1 {
                return format!("{} ", obj.anum);
            }
        }
        ". ".into()
    });
}

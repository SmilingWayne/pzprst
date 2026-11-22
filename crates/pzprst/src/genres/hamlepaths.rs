//! Hamle Paths (pzprjs `variety/bonsan.js`, pid `hamlepaths`). Cells hold
//! distance clues; the URL stores only cell numbers, FileIO adds the
//! `qsub`/`qcmp`/`anum` marks and the border line layer.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Hamlepaths;

struct Hamlepaths;

impl Genre for Hamlepaths {
    fn pid(&self) -> &'static str {
        "hamlepaths"
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
        decode_cell_qsub_qcmp(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        encode_cell_qsub_qcmp(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().enumerate().any(|(i, c)| {
            !(c.qnum == -2 || c.qnum == -1 || (0..=maxnum(b, i)).contains(&c.qnum))
                || !(0..=15).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
                || !matches!(c.anum, -1 | 0)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "hamlepaths cell value outside clue 0-max / ? / empty, or packed marks",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "hamlepaths border value outside line 0/1, cross -1",
            );
        }
    }
}

/// pzprjs `Cell.maxnum`: the distance from the cell to the farther edge.
fn maxnum(b: &Board, index: usize) -> i32 {
    let (cols, rows) = (b.cols() as i32, b.rows() as i32);
    let (bx, by) = b.geom.cell_pos(index);
    let col = (if bx < cols { 2 * cols - bx } else { bx }) >> 1;
    let row = (if by < rows { 2 * rows - by } else { by }) >> 1;
    col.max(row)
}

/// `decodeCellQsubQcmp`: `qsub` in the low nibble, `qcmp` in bit 4 and
/// `anum == 0` in bit 5.
fn decode_cell_qsub_qcmp(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca != "0" {
            let n = fileio::int(ca)?;
            cell.qsub = n & 0x0f;
            cell.qcmp = (n >> 4) & 1;
            cell.anum = if n >> 5 != 0 { 0 } else { -1 };
        }
        Ok(())
    })
}

/// `encodeCellQsubQcmp`.
fn encode_cell_qsub_qcmp(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        let n = cell.qsub + (cell.qcmp << 4) + if cell.anum == 0 { 1 << 5 } else { 0 };
        format!("{n} ")
    });
}

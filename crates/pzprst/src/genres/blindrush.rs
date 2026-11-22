//! Blind Rush (pzprjs `variety/blindrush.js`, pid `blindrush`).
//!
//! Cells hold a clue whose legal range grows towards the middle of the board
//! (`Cell.maxnum` = distance from the nearest edge in cells, `minnum` 0); the
//! URL stores the border walls (`decodeBorder`) and then the clues
//! (`decodeNumber16`). FileIO stores the clues, the wall layer, the
//! `CellAnumsub` answer layer and the line layer.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Blindrush;

struct Blindrush;

impl Genre for Blindrush {
    fn pid(&self) -> &'static str {
        "blindrush"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        encode::decode_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_border(b),
            encode::encode_number16(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_border_ques(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_border_ques(w, b);
        fileio::encode_cell_anumsub(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let (cols, rows) = (b.cols() as i32, b.rows() as i32);
        for (i, cell) in b.cells.iter().enumerate() {
            let (bx, by) = b.geom.cell_pos(i);
            let (cx, cy) = (bx >> 1, by >> 1);
            let maxnum = cx.max(cy).max(cols - 1 - cx).max(rows - 1 - cy);
            if !(cell.qnum == -1 || cell.qnum == -2 || (0..=maxnum).contains(&cell.qnum)) {
                d.push(
                    DiagnosticCode::ValueOutOfRange,
                    format!("blindrush cell value outside empty / ? / clue 0-{maxnum}"),
                );
                break;
            }
        }
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.ques)
                || !(0..=1).contains(&e.line)
                || !(e.qsub == 0 || e.qsub == 2)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "blindrush border value outside wall 0/1, line 0/1 or cross -1",
            );
        }
    }
}

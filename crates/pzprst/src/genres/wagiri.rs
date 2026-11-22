//! Wagiri (pzprjs `variety/gokigen.js`, pid `wagiri`): Gokigen Naname with
//! 輪/切 cell clues.
//!
//! The URL body is `encode4Cross` (base-16 `0`–`4` cross numbers with base-36
//! run-length blanks) followed by `encodeNumber10` (one cell clue per cell:
//! `1`/`2` clue, `.` for `?`, base-36 blanks). FileIO stores `decodeCrossNum`,
//! then the cell clue layer (`decodeCellQnum`), then the cell answer layer
//! (`1` = `\`, `2` = `/`). Transformations swap the two slash directions once
//! per pzprjs turn/flip operation.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Wagiri;

struct Wagiri;

impl Genre for Wagiri {
    fn pid(&self) -> &'static str {
        "wagiri"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_4cross(b, body, d);
        encode::decode_number10(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_4cross(b);
        body.push_str(&encode::encode_number10(b)?);
        Ok(body)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cross_num(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        r.decode_cell(b, d, |cell, ca| {
            if ca == "1" {
                cell.qans = 31;
            } else if ca == "2" {
                cell.qans = 32;
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cross_num(w, b);
        fileio::encode_cell_qnum(w, b);
        w.encode_cell(b, |cell| match cell.qans {
            31 => "1 ".into(),
            32 => "2 ".into(),
            _ => ". ".into(),
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cross: { maxnum: 4, minnum: 0 }`; -1 is empty and -2 is `?`.
        if b.crosses
            .iter()
            .any(|cross| !(cross.qnum == -1 || cross.qnum == -2 || (0..=4).contains(&cross.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "wagiri cross value outside 0-4 / ?",
            );
        }
        // `decodeNumber10` stores 0–9 and `?`; the editor only enters 1, 2, `?`
        // (`Cell.minnum` stays 1, and a clue above 9 makes the URL unencodable).
        if b
            .cells
            .iter()
            .any(|cell| !(-2..=9).contains(&cell.qnum))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "wagiri cell clue outside 0-9 / ?",
            );
        }
        if b.cells
            .iter()
            .any(|cell| !(cell.qans == 0 || cell.qans == 31 || cell.qans == 32))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "wagiri cell value outside empty / \\ (31) / / (32)",
            );
        }
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData` (shared by `gokigen` and
        // `wagiri`): every turn or flip swaps `\` and `/`. pzprjs runs the hook
        // once per operation, so a composed transform applies one swap per
        // operation: rot180, transpose and antitranspose are two operations and
        // leave the slashes unchanged.
        let operations = match t {
            Transform::Identity => 0,
            Transform::Rot90 | Transform::Rot270 | Transform::FlipX | Transform::FlipY => 1,
            Transform::Rot180 | Transform::Transpose | Transform::AntiTranspose => 2,
        };
        if operations % 2 == 0 {
            return;
        }
        for cell in &mut b.cells {
            cell.qans = match cell.qans {
                31 => 32,
                32 => 31,
                v => v,
            };
        }
    }
}

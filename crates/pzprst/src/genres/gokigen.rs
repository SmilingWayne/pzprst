//! Gokigen Naname (pzprjs `variety/gokigen.js`, pid `gokigen`): slash clues
//! on the cross grid, one diagonal per cell as the answer (`qans` 31 = `\`,
//! 32 = `/`).
//!
//! The URL body is `encode4Cross` (base-16 `0`–`4` cross numbers with base-36
//! run-length blanks), except that the pzprv3 flag `d` selects the old
//! `decodecross_old` format: one base-10 `0`–`4` per cross, everything else
//! ignored. pzprjs never writes that flag back, so `decode_url` clears it.
//!
//! FileIO stores `decodeCrossNum` (cross clue or `?`) followed by the cell
//! answer layer (`1` = `\`, `2` = `/`). Transformations swap the two slash
//! directions once per pzprjs turn/flip operation.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Gokigen;

/// pzprjs `Board.rules` key for the decode-only `d` URL flag (no pzprjs config
/// of that name exists; the flag is never encoded).
const OLD_FLAG: &str = "decodecross_old";

struct Gokigen;

impl Genre for Gokigen {
    fn pid(&self) -> &'static str {
        "gokigen"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('d', OLD_FLAG)]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let old = b.rules.get(OLD_FLAG).copied().unwrap_or(false);
        b.rules.insert(OLD_FLAG, false);
        if old {
            encode::decode_cross_old(b, body, d)
        } else {
            encode::decode_4cross(b, body, d)
        }
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_4cross(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cross_num(r, b, d)?;
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
                "gokigen cross value outside 0-4 / ?",
            );
        }
        if b.cells
            .iter()
            .any(|cell| !(cell.qans == 0 || cell.qans == 31 || cell.qans == 32))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "gokigen cell value outside empty / \\ (31) / / (32)",
            );
        }
    }

    fn adjust_board_data_before(&self, t: Transform, b: &mut Board) {
        // pzprjs `BoardExec.adjustBoardData`: every turn or flip swaps `\` and
        // `/`. pzprjs runs the hook once per operation, so a composed transform
        // applies one swap per operation: rot180, transpose and antitranspose
        // are two operations and leave the slashes unchanged.
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

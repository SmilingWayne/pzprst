//! Zabajaba (pzprjs `variety/kaidan.js`, pid `zabajaba`). Kaidan (`kaidan`),
//! Takoyaki (`takoyaki`), Edamame (`edamame`) and Wittgenstein Briquet
//! (`wittgen`) share that script but override `Encode`/`FileIO`/`Cell`, so they
//! are separate pids.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Zabajaba;

struct Zabajaba;

impl Genre for Zabajaba {
    fn pid(&self) -> &'static str {
        "zabajaba"
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
        // `kaidan.js` FileIO: one bit field per cell. `zabajaba` has no `qans`
        // layer, so bit 0 and bit 4 are cell sub marks.
        r.decode_cell(b, d, |cell, ca| {
            let val = fileio::int(ca)?;
            if val & 1 != 0 {
                cell.qsub = 2;
            }
            if val & 2 != 0 {
                cell.line = 1;
            }
            if val & 4 != 0 {
                cell.qsub = 1;
            }
            if val & 8 != 0 {
                cell.qcmp = 1;
            }
            if val & 16 != 0 {
                cell.qsub = 2;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        w.encode_cell(b, |cell| {
            let ans = i32::from(cell.qsub == 2);
            let sub = i32::from(cell.qsub == 1);
            let value = ans | (cell.line << 1) | (sub << 2) | (cell.qcmp << 3);
            format!("{value} ")
        });
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=8).contains(&c.qnum))
                || !(0..=2).contains(&c.qsub)
                || !(0..=1).contains(&c.line)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "zabajaba cell value outside empty / ? / clue 0..8, sub 0-2, line 0/1, cmp 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !matches!(e.qsub, 0 | 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "zabajaba border value outside line 0/1 or cross -1",
            );
        }
    }
}

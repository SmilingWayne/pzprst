//! Coffee Milk (pzprjs `variety/wblink.js`, pid `coffeemilk`). The shared
//! `wblink` sections provide the board and FileIO codecs; the `@coffeemilk`
//! sections change the circle colours/keys and use the `Number10or16` URL codec.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Coffeemilk;

struct Coffeemilk;

impl Genre for Coffeemilk {
    fn pid(&self) -> &'static str {
        "coffeemilk"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number10or16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number10or16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !matches!(c.qnum, -2 | -1 | 1 | 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "coffeemilk cell value outside white 1 / black 2 / ?",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "coffeemilk border value outside line 0/1, cross -1",
            );
        }
    }
}

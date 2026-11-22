//! Retsurin (pzprjs `variety/yajilin.js`, pid `retsurin`). A yajilin variant
//! whose clues count shaded cells along the whole row and column, so it uses
//! plain cell numbers instead of arrows.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Retsurin;

struct Retsurin;

impl Genre for Retsurin {
    fn pid(&self) -> &'static str {
        "retsurin"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('o', "yajilin_out"), ('b', "disptype_yajilin")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "o", "yajilin_out");
        decode_config(r, b);
        r.decode_config_flag(b, "b", "disptype_yajilin");
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "o", "yajilin_out");
        encode_config(w, b);
        w.encode_config_flag(b, "b", "disptype_yajilin");
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = (b.cols().max(b.rows()) >> 1) as i32;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "retsurin cell value outside clue 0..max(cols,rows)/2 / ? or answer mark",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "retsurin border value outside line 0/1 or cross -1",
            );
        }
    }
}

/// `decodeConfig`: the `koburin_minesweeper` flag retsurin shares with koburin.
/// It is FileIO-only (retsurin URLs never write it), so the default absent state
/// stays out of the rule map instead of being recorded as `false`.
fn decode_config(r: &mut FileReader, b: &mut Board) {
    if r.peek_line() == Some("m") {
        r.read_line();
        b.rules.insert("koburin_minesweeper", true);
    }
}

/// `encodeConfig`: the `koburin_minesweeper` flag retsurin shares with koburin.
fn encode_config(w: &mut FileWriter, b: &Board) {
    w.encode_config_flag(b, "m", "koburin_minesweeper");
}

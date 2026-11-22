//! Koburin (pzprjs `variety/yajilin.js`, pid `koburin`). The shared script
//! also serves `yajilin`, `yajilin-regions`, `lixloop`, `retsurin` and
//! `heyajirimisaki`, which override `Encode`/`FileIO` and are separate pids.
//!
//! `Cell@koburin#2` sets minnum 0 / maxnum 4, and the URL flags select the
//! rule configs in the order `o`, `m`, `b`.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Koburin;

struct Koburin;

impl Genre for Koburin {
    fn pid(&self) -> &'static str {
        "koburin"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[
            ('o', "yajilin_out"),
            ('m', "koburin_minesweeper"),
            ('b', "disptype_yajilin"),
        ]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_4cell(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_4cell(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "o", "yajilin_out");
        r.decode_config_flag(b, "m", "koburin_minesweeper");
        r.decode_config_flag(b, "b", "disptype_yajilin");
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "o", "yajilin_out");
        w.encode_config_flag(b, "m", "koburin_minesweeper");
        w.encode_config_flag(b, "b", "disptype_yajilin");
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=4).contains(&c.qnum))
                || !(0..=1).contains(&c.qans)
                || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "koburin cell value outside clue 0-4 / ? / empty or shade 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "koburin border value outside line 0/1, cross -1",
            );
        }
    }
}

//! Arukone (pzprjs `variety/numlin.js`, pid `arukone`). Number Link with
//! letter clues and an optional "every cell must be used" rule. The sibling
//! `numlin` stores plain numbers instead.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Arukone;

struct Arukone;

impl Genre for Arukone {
    fn pid(&self) -> &'static str {
        "arukone"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('e', "dontpassallcell")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_number16(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_number16(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_config(r, b);
        decode_cell_qnum_letter(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_config(w, b);
        encode_cell_qnum_letter(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=52).contains(&c.qnum))
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "arukone cell value outside clue 1-52 / ?",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "arukone border value outside line 0/1 or cross -1",
            );
        }
    }
}

/// `decodeConfig`: arukone always reads one `passallcell` / `dontpassallcell`
/// line (`dontpassallcell` is also the URL flag `e`).
fn decode_config(r: &mut FileReader, b: &mut Board) {
    let passallcell = r.read_line() == Some("passallcell");
    b.rules.insert("dontpassallcell", !passallcell);
}

/// `encodeConfig`: arukone always writes the `passallcell` line.
fn encode_config(w: &mut FileWriter, b: &Board) {
    let dontpass = b.rules.get("dontpassallcell").copied().unwrap_or(false);
    w.write_line(if dontpass {
        "dontpassallcell"
    } else {
        "passallcell"
    });
}

/// `decodeCellQnum_letter`: `A`-`Z` are 1-26, `a`-`z` are 27-52, `-` is `?`.
fn decode_cell_qnum_letter(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "-" {
            cell.qnum = -2;
        } else if ("A"..="Z").contains(&ca) {
            cell.qnum = jsnum::parse_int(ca, 36).unwrap_or(0) as i32 - 9;
        } else if ("a"..="z").contains(&ca) {
            cell.qnum = jsnum::parse_int(ca, 36).unwrap_or(0) as i32 - 9 + 26;
        }
        Ok(())
    })
}

/// `encodeCellQnum_letter`: `A`-`Z` for 1-26, `a`-`z` for 27-52, `-` for `?`.
fn encode_cell_qnum_letter(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| match cell.qnum {
        n if (1..=26).contains(&n) => format!("{} ", jsnum::to_radix(n as u32 + 9, 36).to_uppercase()),
        n if (27..=52).contains(&n) => format!("{} ", jsnum::to_radix(n as u32 - 17, 36)),
        -2 => "- ".into(),
        _ => ". ".into(),
    });
}

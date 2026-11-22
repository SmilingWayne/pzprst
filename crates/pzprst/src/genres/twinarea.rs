//! Twin Area (pzprjs `variety/hanare.js`, pid `twinarea`). The script is shared
//! with `hanare` and `putteria`; `twinarea` adds the ice (`ques` 6) URL layer
//! and stores the ice flag as a `-` prefix on the FileIO question number.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Twinarea;

struct Twinarea;

impl Genre for Twinarea {
    fn pid(&self) -> &'static str {
        "twinarea"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, body, d);
        let rest = encode::decode_number16(b, &rest, d);
        encode::decode_ice(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}{}",
            encode::encode_border(b),
            encode::encode_number16(b),
            encode::encode_ice(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_border_ques(r, b, d)?;
        decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_border_ques(w, b);
        encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !matches!(c.ques, 0 | 6) || !(c.qnum == -1 || c.qnum == -2 || (1..=999).contains(&c.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "twinarea cell value outside empty / ? / 1..999, or ice flag outside 0/6",
            );
        }
        if b.borders.iter().any(|e| !(0..=1).contains(&e.ques)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "twinarea border value outside wall 0/1",
            );
        }
    }
}

/// `decodeCellQnum` (`FileIO@twinarea`): a leading `-` is the ice flag, and only
/// positive numbers become the question number.
fn decode_cell_qnum(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let mut bad = Vec::new();
    r.decode_cell(b, d, |cell, ca| {
        let ca = match ca.strip_prefix('-') {
            Some(rest) => {
                cell.ques = 6;
                rest
            }
            None => ca,
        };
        if ca != "." {
            match jsnum::integer(ca) {
                Some(n) if n > 0 => cell.qnum = n,
                Some(_) => {}
                None => bad.push(ca.to_string()),
            }
        }
        Ok(())
    })?;
    for ca in bad {
        d.push(
            DiagnosticCode::UnexpectedChar,
            format!("question number {ca:?} is not a number; cell left empty"),
        );
    }
    Ok(())
}

/// `encodeCellQnum` (`FileIO@twinarea`): `-` ice marker, then the number.
fn encode_cell_qnum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        let mut ca = String::new();
        if cell.ques == 6 {
            ca.push('-');
        }
        if cell.qnum != -1 {
            ca.push_str(&cell.qnum.to_string());
        }
        if ca.is_empty() {
            ca.push('.');
        }
        format!("{ca} ")
    });
}

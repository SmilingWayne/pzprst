//! Nuri-uzu (pzprjs `variety/tentaisho.js`, pid `nuriuzu`).
//!
//! Dots live on cells, borders and crosses, but nuriuzu's `Dot.setDot` refuses
//! every non-zero value on a cross, so cross dots never survive a decode. The
//! URL stores the dots only; the `nuriuzu_connect` flag is pflag `c` and a
//! `c` line in FileIO.

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &NuriUzu;

struct NuriUzu;

/// `Dot@nuriuzu.setDot`: a cross ignores every non-zero dot value.
fn set_dot(dot: &mut crate::Obj, group: Group, val: i32) {
    if group == Group::Cross && val != 0 {
        return;
    }
    dot.qnum = val;
}

/// `decodeConfig`: the `nuriuzu_connect` flag line.
fn decode_config(r: &mut FileReader, b: &mut Board) {
    r.decode_config_flag(b, "c", "nuriuzu_connect");
}

/// `encodeConfig`: the `nuriuzu_connect` flag line.
fn encode_config(w: &mut FileWriter, b: &Board) {
    w.encode_config_flag(b, "c", "nuriuzu_connect");
}

impl Genre for NuriUzu {
    fn pid(&self) -> &'static str {
        "nuriuzu"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('c', "nuriuzu_connect")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_dot(b, body, d, set_dot)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_dot(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_config(r, b);
        fileio::decode_dot_file(r, b, d, set_dot)?;
        fileio::decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_config(w, b);
        fileio::encode_dot_file(w, b);
        fileio::encode_cell_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let dots = || b.cells.iter().chain(&b.borders).chain(&b.crosses);
        if dots().any(|dot| !matches!(dot.qnum, -2..=9) || !matches!(dot.ques, 0 | 7)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "nuriuzu dot value outside empty / ? / 1-9, or X mark outside 0/7",
            );
        }
        if dots().any(|dot| dot.qnum == -2 || dot.ques == 7) {
            d.push(
                DiagnosticCode::LossyUrl,
                "nuriuzu '?' dots and X marks are dropped from the URL",
            );
        }
    }
}

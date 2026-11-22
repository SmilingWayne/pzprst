//! The Longest (pzprjs `variety/longest.js`, pid `longest`).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Longest;

struct Longest;

impl Genre for Longest {
    fn pid(&self) -> &'static str {
        "longest"
    }

    fn hasborder(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        encode::decode_outer_border(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode::encode_outer_border(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_border(b, d, |border, ca| {
            if ca == "2" {
                border.ques = 0;
                border.qans = 1;
            } else if ca == "-1" {
                border.ques = 0;
                border.qsub = 1;
            } else if ca == "1" {
                border.ques = 1;
            } else {
                border.ques = 0;
            }
            Ok(())
        })
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_border(b, |border| {
            if border.qans == 1 {
                "2 ".into()
            } else if border.ques == 1 {
                "1 ".into()
            } else if border.qsub == 1 {
                "-1 ".into()
            } else {
                "0 ".into()
            }
        });
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.borders.iter().any(|e| {
            !matches!(e.ques, 0 | 1) || !matches!(e.qans, 0 | 1) || !matches!(e.qsub, 0 | 1)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "longest border value outside ques/qans/qsub 0 or 1",
            );
        }
    }
}

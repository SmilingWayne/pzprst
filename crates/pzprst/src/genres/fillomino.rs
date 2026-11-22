//! Fillomino (pzprjs `variety/fillomino.js`, pid `fillomino`). Cells hold one
//! number, inner borders are walls (`ques`). `Cell.maxnum` is `cols * rows`, or 3
//! when the `t`/`fillomino_tri` flag is set. FileIO adds the answer number /
//! sub-mark layer per cell and the border answer layer, whose `X`/`-X` tokens
//! mark wall answers.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Fillomino;

struct Fillomino;

impl Genre for Fillomino {
    fn pid(&self) -> &'static str {
        "fillomino"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('t', "fillomino_tri")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_number16(b, body, d);
        // pzprjs always calls `decodeBorder`, but `encodeBorderIfPresent` omits the
        // layer when no border is a wall, so an empty remainder means "no walls",
        // not a border body that ends too early.
        if rest.is_empty() {
            return rest;
        }
        encode::decode_border(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_number16(b),
            encode_border_if_present(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "t", "fillomino_tri");
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)?;
        decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "t", "fillomino_tri");
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum`: 3 with the triangular rule, otherwise cols * rows.
        let maxnum = if b.rules.get("fillomino_tri").copied().unwrap_or(false) {
            3
        } else {
            (b.cols() * b.rows()) as i32
        };
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || !(0..=1).contains(&c.qcmp)
                || !(0..=4).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("fillomino cell value outside clue 1-{maxnum}/? or the answer layers"),
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.ques) || !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "fillomino border value outside wall 0/1 or the answer marks",
            );
        }
    }
}

/// `encodeBorderIfPresent`: write the border wall layer only when some border is
/// a wall.
fn encode_border_if_present(b: &Board) -> String {
    if b.borders.iter().any(|e| e.ques == 1) {
        encode::encode_border(b)
    } else {
        String::new()
    }
}

/// `decodeBorderAns`: `-X`/`X` mark a wall answer, `1`/`2`/`-1` the usual marks.
fn decode_border_ans(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_border(b, d, |border, ca| {
        match ca {
            "-X" => {
                border.qsub = 1;
                border.ques = 1;
            }
            "X" => border.ques = 1,
            "2" => {
                border.qans = 1;
                border.qsub = 1;
            }
            "1" => border.qans = 1,
            "-1" => border.qsub = 1,
            _ => {}
        }
        Ok(())
    })
}

/// `encodeBorderAns`.
fn encode_border_ans(w: &mut FileWriter, b: &Board) {
    w.encode_border(b, |border| {
        if border.ques == 1 && border.qsub == 1 {
            "-X ".into()
        } else if border.ques == 1 {
            "X ".into()
        } else if border.qans == 1 && border.qsub == 1 {
            "2 ".into()
        } else if border.qans == 1 {
            "1 ".into()
        } else if border.qsub == 1 {
            "-1 ".into()
        } else {
            "0 ".into()
        }
    });
}

//! Tetrominous (pzprjs `variety/fillomino.js`, pid `tetrominous`).
//!
//! A tetromino-packing fillomino. Each cell holds a clue 0–4 (an index into
//! pzprjs `Cell@tetrominous.letters` `"ILOST"`), `?` (`qnum` -2) or `*`
//! (`ques` 7, a cell no piece covers). `hasborder` is 1 and the URL's border
//! layer is the shared `decodeBorder`/`encodeBorder` wall layer, written only
//! when some border is a wall (`encodeBorderIfPresent`).
//!
//! pzprjs `FileIO@pentominous,tetrominous` also stores the `fillomino_tri`
//! config as a `t` line. `Encode@pentominous,tetrominous` replaces `encodePzpr`
//! without setting `outpflag`, so that config never reaches a URL (and a `t`
//! pflag in an input URL is ignored); pzprst keeps it in `Board.rules` from
//! FileIO and does not declare it as a URL flag.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Tetrominous;

/// pzprjs config key stored by the `FileIO` `t` line.
const TRI: &str = "fillomino_tri";

struct Tetrominous;

impl Genre for Tetrominous {
    fn pid(&self) -> &'static str {
        "tetrominous"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let length = b.cells.len();
        let rest = encode::generic_decode_number16(length, body, d, |c, val| {
            if val == 12 {
                b.cells[c].ques = 7;
            } else {
                b.cells[c].qnum = val;
            }
        });
        if rest.is_empty() {
            // `encodeBorderIfPresent` leaves the wall layer out when no border
            // is a wall, so an empty rest is a complete body, not truncation.
            return rest;
        }
        encode::decode_border(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let cells = encode::generic_encode_number16(b.cells.len(), |c| {
            if b.cells[c].ques == 7 {
                12
            } else {
                b.cells[c].qnum
            }
        });
        Ok(format!("{cells}{}", encode_border_if_present(b)))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        if r.peek_line() == Some("t") {
            r.read_line();
            b.rules.insert(TRI, true);
        }
        decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)?;
        decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        if b.rules.get(TRI).copied().unwrap_or(false) {
            w.write_line("t");
        }
        encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| !(c.ques == 0 || c.ques == 7) || !(-2..=4).contains(&c.qnum))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tetrominous cell value outside clue 0-4, ? or * (ques 7)",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.ques) || !(0..=1).contains(&e.qans))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tetrominous border value outside wall 0/1, line 0/1",
            );
        }
        if b.borders.iter().any(|e| !(0..=1).contains(&e.qsub)) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "tetrominous border value outside cross 0/1",
            );
        }
    }
}

/// `encodeBorderIfPresent`: the URL border layer, only when some border is a wall.
fn encode_border_if_present(b: &Board) -> String {
    if b.borders.iter().any(|border| border.ques == 1) {
        encode::encode_border(b)
    } else {
        String::new()
    }
}

/// `decodeCellQnum` (`FileIO@pentominous,tetrominous`): `*` is `ques` 7, `-` is
/// an unknown clue, `.` is empty.
fn decode_cell_qnum(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        cell.ques = 0;
        if ca == "*" {
            cell.ques = 7;
        } else if ca == "-" {
            cell.qnum = -2;
        } else if ca != "." {
            cell.qnum = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQnum` (`FileIO@pentominous,tetrominous`).
fn encode_cell_qnum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.ques == 7 {
            "* ".into()
        } else if cell.qnum == -2 {
            "- ".into()
        } else if cell.qnum >= 0 {
            format!("{} ", cell.qnum)
        } else {
            ". ".into()
        }
    });
}

/// `decodeBorderAns` (`FileIO` override in `variety/fillomino.js`): `X`/`-X`
/// mark a wall, with or without the cross.
fn decode_border_ans(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_border(b, d, |border, ca| {
        if ca == "-X" {
            border.qsub = 1;
            border.ques = 1;
        } else if ca == "X" {
            border.ques = 1;
        } else if ca == "2" {
            border.qans = 1;
            border.qsub = 1;
        } else if ca == "1" {
            border.qans = 1;
        } else if ca == "-1" {
            border.qsub = 1;
        }
        Ok(())
    })
}

/// `encodeBorderAns` (`FileIO` override in `variety/fillomino.js`).
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

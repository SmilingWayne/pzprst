//! Number City (pzprjs `variety/fillomino.js`, pid `numcity`). The other
//! fillomino-family pids override `Encode`/`FileIO` in that script and are not
//! modelled here.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Numcity;

struct Numcity;

impl Genre for Numcity {
    fn pid(&self) -> &'static str {
        "numcity"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('t', "fillomino_tri")]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        // Encode@fillomino: decodeNumber16, decodeBorder, then the `t` config
        // (read by the URL parser from the pflag). pzprjs's decodeBorder is a
        // no-op on an empty body, which is how encodeBorderIfPresent drops an
        // all-empty wall layer.
        let rest = encode::decode_number16(b, body, d);
        if rest.is_empty() {
            return rest;
        }
        encode::decode_border(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        // Encode@fillomino: encodeNumber16, then encodeBorderIfPresent.
        let mut cm = encode::encode_number16(b);
        cm += &encode_border_if_present(b);
        Ok(cm)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        // FileIO@fillomino, non-wafusuma/topo/snakepit branch.
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
        // pzprjs computes maxnum from the cell's room size; pzprst models no
        // rooms, so the whole board is the largest possible room (a sound
        // upper bound for the per-room formula).
        let maxnum = numcity_maxnum(b.cols() * b.rows());
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (1..=maxnum).contains(&c.qnum))
                || c.ques != 0
                || !(0..=1).contains(&c.qans)
                || !(0..=4).contains(&c.qsub)
                || !(0..=1).contains(&c.qcmp)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "numcity cell value outside clue 1..maxnum, ? or answer mark",
            );
        }
        if b.borders.iter().any(|e| {
            !(0..=1).contains(&e.ques) || !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "numcity border value outside wall 0/1, answer 0/1 or cross 0/1",
            );
        }
    }
}

/// `encodeBorderIfPresent`: the URL border layer only when some border is a wall.
fn encode_border_if_present(b: &Board) -> String {
    if b.borders.iter().any(|border| border.ques == 1) {
        encode::encode_border(b)
    } else {
        String::new()
    }
}

/// `decodeBorderAns`: `-X`/`X` are border walls, `2`/`1`/`-1` the answer mark
/// and the cross of the fillomino border layer.
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

/// `encodeBorderAns`: `-X`/`X` for walls, `2`/`1`/`-1` for the answer mark and
/// the cross, `0` for an empty border.
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

/// pzprjs `Cell@numcity` maxnum: the largest `n` with `n(n+1)/2 <= size`
/// (`floor(sqrt(size * 2 + 0.25) - 0.5)`, `size` being the room area).
fn numcity_maxnum(size: usize) -> i32 {
    let mut n = 0i32;
    while (n + 1) * (n + 2) / 2 <= size as i32 {
        n += 1;
    }
    n
}

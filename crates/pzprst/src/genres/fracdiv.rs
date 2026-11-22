//! Fractional Division / 分数分割 (pzprjs `variety/fracdiv.js`, pid `fracdiv`).
//!
//! A cell is either a plain clue (`ques` 0, `qnum` 1–9), a shaded cell (`ques`
//! 7) or a fraction cell (`ques` 51) carrying numerator `qnum` and denominator
//! `qnum2`. pzprjs's `decodePzpr` contains a stray `cell.qnum === -1;`
//! (comparison, not assignment) that leaves a `?` numerator at -2; it is
//! reproduced as a no-op.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Fracdiv;

struct Fracdiv;

impl Genre for Fracdiv {
    fn pid(&self) -> &'static str {
        "fracdiv"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let n = b.cells.len();
        let cells = &mut b.cells;
        encode::generic_decode_number16(2 * n, body, d, |c, val| {
            let cell = &mut cells[c >> 1];
            if c & 1 == 1 {
                // Denominator.
                if val == -2 {
                    if cell.qnum == 0 {
                        // The "0 ." pair is how pzprjs stores a shaded cell.
                        cell.ques = 7;
                        cell.qnum = -1;
                    } else {
                        cell.ques = 0;
                    }
                } else {
                    cell.qnum2 = val;
                    // pzprjs writes `if (cell.qnum === -2) { cell.qnum === -1; }`
                    // here: both sides are comparisons, so nothing happens.
                }
            } else {
                cell.qnum = val;
                cell.ques = 51;
            }
        })
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let cells = &b.cells;
        Ok(encode::generic_encode_number16(2 * cells.len(), |c| {
            let cell = &cells[c >> 1];
            if c & 1 == 1 {
                // Denominator: only a fraction cell stores one.
                if cell.ques != 51 {
                    if cell.qnum == -1 && cell.ques == 0 {
                        -1
                    } else {
                        -2
                    }
                } else if cell.qnum2 == -2 {
                    -1
                } else {
                    cell.qnum2
                }
            } else if cell.ques == 51 && cell.qnum == -1 {
                -2
            } else if cell.ques == 7 {
                // `!cell.isValid()` (`ques === 7`) encodes as 0, the numerator of
                // the "0 ." shaded-cell pair.
                0
            } else {
                cell.qnum
            }
        }))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |cell, ca| {
            if ca == "#" {
                cell.ques = 7;
            } else if let Some(num) = ca.strip_prefix('d') {
                cell.qnum = fileio::int(num)?;
            } else if ca != "." {
                let mut tokens = ca.split(',');
                cell.ques = 51;
                cell.qnum = fileio::int(tokens.next().unwrap_or(""))?;
                cell.qnum2 = fileio::int(
                    tokens
                        .next()
                        .ok_or_else(|| format!("expected \"qnum,qnum2\", found {ca:?}"))?,
                )?;
            }
            Ok(())
        })?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |cell| {
            if cell.ques == 7 {
                "# ".into()
            } else if cell.ques == 51 {
                format!("{},{} ", cell.qnum, cell.qnum2)
            } else if cell.qnum != -1 {
                format!("d{} ", cell.qnum)
            } else {
                ". ".into()
            }
        });
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let bad = b.cells.iter().any(|c| {
            let (min, max) = if c.ques == 51 { (0, 999) } else { (1, 9) };
            !matches!(c.ques, 0 | 7 | 51)
                || !(c.qnum == -1 || c.qnum == -2 || (min..=max).contains(&c.qnum))
                || !(c.qnum2 == -1 || c.qnum2 == -2 || (min..=max).contains(&c.qnum2))
        });
        if bad {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "fracdiv cell value outside plain clue 1-9, fraction 0-999 or empty/?",
            );
        }
        if b.cells
            .iter()
            .any(|c| c.ques == 51 && (c.qnum == -1 || c.qnum2 == -2))
        {
            d.push(
                DiagnosticCode::LossyUrl,
                "an empty numerator or a negative denominator is dropped from a fracdiv URL",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "fracdiv border value outside answer 0/1 or cross 0/1",
            );
        }
    }
}

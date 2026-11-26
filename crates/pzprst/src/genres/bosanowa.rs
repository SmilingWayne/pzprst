//! Bosanowa: an irregular playable area, clues and difference marks.
use super::Genre;
use crate::error::{DiagnosticCode, Diagnostics, Error, Result};
use crate::fileio::{FileReader, FileWriter};
use crate::{encode, fileio, jsnum, Board};
pub(crate) static GENRE: &dyn Genre = &Bosanowa;
struct Bosanowa;
fn bounds(b: &Board) -> Result<(usize, usize, usize, usize)> {
    let (mut x1, mut y1, mut x2, mut y2) = (b.cols(), b.rows(), 0, 0);
    let mut found = false;
    for y in 0..b.rows() {
        for x in 0..b.cols() {
            if b.cell(y, x).ques != 7 {
                found = true;
                x1 = x1.min(x);
                y1 = y1.min(y);
                x2 = x2.max(x);
                y2 = y2.max(y);
            }
        }
    }
    if !found {
        return Err(Error::InvalidSize("Bosanowa has no playable cells".into()));
    }
    Ok((x1, y1, x2, y2))
}
impl Genre for Bosanowa {
    fn pid(&self) -> &'static str {
        "bosanowa"
    }
    fn hasborder(&self) -> u8 {
        2
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('h', "bosanowa_hex"), ('t', "bosanowa_ten")]
    }
    fn init_board(&self, b: &mut Board) {
        for border in &mut b.borders {
            border.qsub = -1;
        }
    }
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_empty(b, body, d);
        let rest = encode::decode_number16(b, &rest, d);
        self.validate(b, d);
        rest
    }
    fn decode_url_with_flags(
        &self,
        b: &mut Board,
        body: &str,
        flags: &str,
        d: &mut Diagnostics,
    ) -> String {
        // pzprjs gives h precedence when both display flags are present.
        if flags.contains('h') {
            b.rules.insert("bosanowa_ten", false);
        }
        self.decode_url(b, body, d)
    }
    fn url_dimensions(&self, b: &Board) -> Result<(usize, usize)> {
        let (x, y, x2, y2) = bounds(b)?;
        Ok((x2 - x + 1, y2 - y + 1))
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        let (x1, y1, x2, y2) = bounds(b)?;
        let cols = x2 - x1 + 1;
        let rows = y2 - y1 + 1;
        let idx = |i: usize| (y1 + i / cols) * b.cols() + x1 + i % cols;
        let mut out =
            encode::generic_encode_binary(cols * rows, |i| b.cells[idx(i)].ques == 7, false);
        let mut count = 0u32;
        for i in 0..cols * rows {
            let q = b.cells[idx(i)].qnum;
            let s = if q == -2 {
                ".".into()
            } else if (0..16).contains(&q) {
                jsnum::to_radix(q as u32, 16)
            } else if (16..256).contains(&q) {
                format!("-{:02x}", q)
            } else {
                count += 1;
                String::new()
            };
            if count == 0 {
                out.push_str(&s)
            } else if !s.is_empty() || count == 20 {
                out.push_str(&jsnum::to_radix(15 + count, 36));
                out.push_str(&s);
                count = 0;
            }
        }
        if count > 0 {
            out.push_str(&jsnum::to_radix(15 + count, 36));
        }
        Ok(out)
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |c, s| {
            c.ques = if s == "." { 7 } else { 0 };
            if s != "0" && s != "." {
                c.qnum = fileio::int(s)?;
            }
            Ok(())
        })?;
        r.decode_cell(b, d, |c, s| {
            if s != "0" && s != "." {
                c.anum = fileio::int(s)?;
            }
            Ok(())
        })?;
        r.decode_inner_border(b, d, |e, s| {
            if s != "." {
                e.qsub = fileio::int(s)?;
            }
            Ok(())
        })
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |c| {
            format!(
                "{} ",
                if c.ques == 7 {
                    ".".into()
                } else if c.qnum >= 0 {
                    c.qnum.to_string()
                } else {
                    "0".into()
                }
            )
        });
        w.encode_cell(b, |c| {
            format!(
                "{} ",
                if c.ques == 7 || c.qnum != -1 {
                    ".".into()
                } else if c.anum >= 0 {
                    c.anum.to_string()
                } else {
                    "0".into()
                }
            )
        });
        w.encode_inner_border(b, |e| {
            format!(
                "{} ",
                if e.qsub != -1 {
                    e.qsub.to_string()
                } else {
                    ".".into()
                }
            )
        });
        Ok(())
    }
    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| c.qnum >= 256) {
            d.push(
                DiagnosticCode::LossyUrl,
                "Bosanowa URL drops clues above 255",
            );
        }
        if bounds(b).is_ok_and(|(x, y, x2, y2)| {
            x != 0 || y != 0 || x2 + 1 != b.cols() || y2 + 1 != b.rows()
        }) {
            d.push(DiagnosticCode::LossyUrl, "Bosanowa URL crops empty margins");
        }
    }
}

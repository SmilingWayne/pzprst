//! Japanese Sums: ordered outside sums, indicator and cell numbers.
use super::Genre;
use crate::error::{Diagnostics, Result};
use crate::fileio::{FileReader, FileWriter};
use crate::{encode, fileio, jsnum, Board, Group, Transform};
pub(crate) static GENRE: &dyn Genre = &JapaneseSums;
struct JapaneseSums;
impl Genre for JapaneseSums {
    fn pid(&self) -> &'static str {
        "japanesesums"
    }
    fn hasborder(&self) -> u8 {
        0
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn hasexcell(&self) -> u8 {
        1
    }
    fn excell_rows(&self, _: usize, rows: usize) -> usize {
        (rows + 1) / 2
    }
    fn excell_cols(&self, cols: usize, _: usize) -> usize {
        (cols + 1) / 2
    }
    fn subnumbers(&self) -> bool {
        true
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn init_board(&self, b: &mut Board) {
        b.indicator = Some(4);
    }
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let (n, rest) = body.split_once('/').unwrap_or((body, ""));
        b.indicator = Some(jsnum::integer(n).unwrap_or(0));
        let rest = encode::decode_number16_excell(b, rest, d);
        if rest.is_empty() {
            rest
        } else {
            encode::decode_number16(b, &rest, d)
        }
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut s = format!(
            "{}/{}",
            b.indicator.unwrap_or(4),
            encode::encode_number16_excell_flushed(b)
        );
        if b.cells.iter().any(|c| c.qnum != -1) {
            s.push_str(&encode::encode_number16(b));
        }
        Ok(s)
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        b.indicator = Some(r.read_line().and_then(jsnum::integer).unwrap_or(0));
        fileio::decode_cell_excell(r, b, d, |o, g, s| {
            if s == "." {
                return Ok(());
            }
            if g == Group::ExCell {
                o.qnum = fileio::int(s)?;
            } else if let Some(n) = s.strip_prefix('q') {
                o.qnum = fileio::int(n)?;
            } else {
                let s = fileio::set_cell_snum(o, s)?;
                match s {
                    "+" => o.qsub = 1,
                    "-" => o.qans = 1,
                    "." => {}
                    _ => o.anum = fileio::int(s)?,
                }
            }
            Ok(())
        })
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.write_line(&b.indicator.unwrap_or(4).to_string());
        fileio::encode_cell_excell(w, b, |o, g| {
            let mut s = if g == Group::ExCell {
                if o.qnum != -1 {
                    o.qnum.to_string()
                } else {
                    ".".into()
                }
            } else if o.qnum != -1 {
                format!("q{}", o.qnum)
            } else if o.anum != -1 {
                o.anum.to_string()
            } else if o.qsub == 1 {
                "+".into()
            } else if o.qans == 1 {
                "-".into()
            } else {
                ".".into()
            };
            if g == Group::Cell && o.qnum == -1 && o.anum == -1 {
                s.push_str(&fileio::get_cell_snum(o));
            }
            format!("{s} ")
        });
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        super::g_nonogram::GENRE.adjust_board_data(t, b);
    }
}

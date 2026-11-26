//! Isowatari: cluster size, circles, holes and player shading.
use super::Genre;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{FileReader, FileWriter};
use crate::{encode, fileio, jsnum, Board};
pub(crate) static GENRE: &dyn Genre = &Isowatari;
struct Isowatari;
impl Genre for Isowatari {
    fn pid(&self) -> &'static str {
        "isowatari"
    }
    fn hasborder(&self) -> u8 {
        2
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn subnumbers(&self) -> bool {
        true
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn init_board(&self, b: &mut Board) {
        b.indicator = Some(1);
    }
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let (count, rest) = body.split_once('/').unwrap_or((body, ""));
        b.indicator = Some(jsnum::integer(count).unwrap_or(0));
        let rest = encode::decode_circle(b, rest, d);
        encode::decode_empty(b, &rest, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}/{}{}",
            b.indicator.unwrap_or(1),
            encode::encode_circle(b)?,
            encode::encode_empty(b)
        ))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        b.indicator = Some(r.read_line().and_then(jsnum::integer).unwrap_or(0));
        r.decode_cell(b, d, |c, s| {
            c.ques = 0;
            match s {
                "*" => c.ques = 7,
                "-" => c.qnum = -2,
                "." => {}
                _ => c.qnum = fileio::int(s)?,
            };
            Ok(())
        })?;
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.write_line(&b.indicator.unwrap_or(1).to_string());
        w.encode_cell(b, |c| {
            format!(
                "{} ",
                if c.ques == 7 {
                    "*".into()
                } else if c.qnum == -2 {
                    "-".into()
                } else if c.qnum >= 0 {
                    c.qnum.to_string()
                } else {
                    ".".into()
                }
            )
        });
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        fileio::encode_cell_ans(w, b);
        Ok(())
    }
    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| c.qnum == -2) {
            d.push(
                DiagnosticCode::LossyUrl,
                "unknown circle cannot be represented in URL",
            );
        }
    }
}

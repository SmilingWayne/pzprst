//! Statue Park: circle clues, shading and a reusable ordered piece bank.
use super::Genre;
use crate::error::{DiagnosticCode, Diagnostics, Error, Result};
use crate::fileio::{FileReader, FileWriter};
use crate::{bank, encode, fileio, Board};
pub(crate) static GENRE: &dyn Genre = &StatuePark;
struct StatuePark;
const P: &[&str] = &[
    "337k", "15v", "24as", "24bo", "23fg", "337i", "23rg", "334u", "335s", "33bk", "24bk", "337o",
];
const T: &[&str] = &["14u", "23bg", "22u", "23f", "23eg"];
const D: &[&str] = &[
    "14u", "14u", "23bg", "23bg", "22u", "22u", "23f", "23f", "23eg", "23eg",
];
fn presets() -> &'static [(&'static str, &'static [&'static str])] {
    &[("p", P), ("t", T), ("d", D), ("z", &[])]
}
impl Genre for StatuePark {
    fn pid(&self) -> &'static str {
        "statuepark"
    }
    fn hasborder(&self) -> u8 {
        0
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn init_board(&self, b: &mut Board) {
        b.bank = bank::from_preset(presets(), "p");
    }
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = if body.starts_with('/') {
            body.to_string()
        } else {
            encode::decode_circle(b, body, d)
        };
        bank::decode_url(&rest, &mut b.bank, presets(), d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let bank = b
            .bank
            .as_ref()
            .ok_or_else(|| Error::ValueOutOfRange("Statue Park requires a piece bank".into()))?;
        Ok(format!(
            "{}{}",
            encode::encode_circle(b)?,
            bank::encode_url(bank, presets())?
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        let fallback = b.bank.clone().unwrap_or_default();
        b.bank = Some(bank::decode_fileio(r, &fallback, presets())?);
        fileio::decode_cell_qnum(r, b, d)?;
        r.decode_cell(b, d, |c, s| {
            match s {
                "X" => {
                    c.ques = 7;
                    c.qsub = 1
                }
                "x" => c.ques = 7,
                "#" => c.qans = 1,
                "+" => c.qsub = 1,
                _ => {}
            }
            Ok(())
        })?;
        bank::decode_marks(r, b.bank.as_mut().expect("bank initialized above"));
        Ok(())
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        let bank = b
            .bank
            .as_ref()
            .ok_or_else(|| Error::ValueOutOfRange("Statue Park requires a piece bank".into()))?;
        bank::encode_fileio(w, bank, presets())?;
        fileio::encode_cell_qnum(w, b);
        w.encode_cell(b, |c| {
            format!(
                "{} ",
                if c.ques == 7 {
                    if c.qsub != 0 {
                        "X"
                    } else {
                        "x"
                    }
                } else if c.qans != 0 {
                    "#"
                } else if c.qsub != 0 {
                    "+"
                } else {
                    "."
                }
            )
        });
        bank::encode_marks(w, bank);
        Ok(())
    }
    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| c.ques == 7 || c.qnum == -2) {
            d.push(
                DiagnosticCode::LossyUrl,
                "holes or unknown circles cannot be stored in the puzzle URL",
            );
        }
    }
}

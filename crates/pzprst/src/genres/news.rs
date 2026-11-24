//! Toichika rooms and direction-valued clues and answers.
use super::{g_yajikazu as arrows, Genre};
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "news"
    }
    fn hasborder(&self) -> u8 {
        1
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn subnumbers(&self) -> bool { true }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[('n', "tren_new")]
    }
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, s, d);
        let chars: Vec<char> = rest.chars().collect();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() {
            let ch = chars[i];
            match ch {
                '1'..='4' => b.cells[c].qnum = ch as i32 - '0' as i32,
                '.' => b.cells[c].qnum = -2,
                _ => {
                    if let Some(n) = jsnum::digit(ch, 36) {
                        c = c.saturating_add(n.saturating_sub(5) as usize);
                    } else {
                        d.push(
                            DiagnosticCode::UnexpectedChar,
                            format!("unexpected news character {ch:?}"),
                        );
                    }
                }
            }
            c += 1;
            if c >= b.cells.len() {
                break;
            }
            i += 1;
        }
        chars.iter().skip(i + 1).collect()
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut out = encode::encode_border(b);
        let mut count = 0;
        for c in &b.cells {
            let s = match c.qnum {
                -2 => ".".into(),
                1..=4 => c.qnum.to_string(),
                _ => {
                    count += 1;
                    String::new()
                }
            };
            if count == 0 {
                out.push_str(&s);
            } else if !s.is_empty() || count == 31 {
                out.push_str(&jsnum::to_radix(count + 4, 36));
                out.push_str(&s);
                count = 0;
            }
        }
        if count > 0 {
            out.push_str(&jsnum::to_radix(count + 4, 36));
        }
        Ok(out)
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_config_flag(b, "n", "tren_new");
        fileio::decode_area_room(r, b, d)?;
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_config_flag(b, "n", "tren_new");
        fileio::encode_area_room(w, b);
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data_before(&self, t: crate::Transform, b: &mut Board) {
        arrows::adjust_cell_numbers(b, t);
    }
}

//! Nurimaze room borders, circle/triangle clues and start/goal addresses.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
fn set_address(b: &mut Board, idx: usize, start: bool) {
    for c in &mut b.cells {
        if start {
            c.qnum2 = -1;
        } else {
            c.qnum3 = -1;
        }
    }
    if start {
        b.cells[idx].qnum2 = 1;
    } else {
        b.cells[idx].qnum3 = 1;
    }
}
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "nurimaze"
    }
    fn hasborder(&self) -> u8 {
        1
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn init_board(&self, b: &mut Board) {
        b.cells[0].qnum2 = 1;
        let last = b.cells.len() - 1;
        b.cells[last].qnum3 = 1;
    }
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, s, d);
        let chars: Vec<char> = rest.chars().collect();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() {
            match chars[i] {
                '1' => set_address(b, c, true),
                '2' => set_address(b, c, false),
                '3' => b.cells[c].ques = 41,
                '4' => b.cells[c].ques = 42,
                '5'..='9' | 'a'..='z' => c += jsnum::digit(chars[i], 36).unwrap() as usize - 5,
                _ => {}
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
            let s = if c.qnum2 == 1 {
                "1"
            } else if c.qnum3 == 1 {
                "2"
            } else if c.ques == 41 {
                "3"
            } else if c.ques == 42 {
                "4"
            } else {
                count += 1;
                ""
            };
            if count == 0 {
                out.push_str(s);
            } else if !s.is_empty() || count == 31 {
                out.push_str(&jsnum::to_radix(count + 4, 36));
                out.push_str(s);
                count = 0;
            }
        }
        if count > 0 {
            out.push_str(&jsnum::to_radix(count + 4, 36));
        }
        Ok(out)
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_area_room(r, b, d)?;
        let (mut i, mut start, mut goal) = (0usize, None, None);
        r.decode_cell(b, d, |c, s| {
            match s {
                "s" => start = Some(i),
                "g" => goal = Some(i),
                "o" => c.ques = 41,
                "t" => c.ques = 42,
                _ => {}
            };
            i += 1;
            Ok(())
        })?;
        if let Some(i) = start {
            set_address(b, i, true);
        }
        if let Some(i) = goal {
            set_address(b, i, false);
        }
        fileio::decode_cell_ans(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_area_room(w, b);
        w.encode_cell(b, |c| {
            if c.qnum2 == 1 {
                "s "
            } else if c.qnum3 == 1 {
                "g "
            } else if c.ques == 41 {
                "o "
            } else if c.ques == 42 {
                "t "
            } else {
                ". "
            }
            .into()
        });
        fileio::encode_cell_ans(w, b);
        fileio::encode_border_line_if_present(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
}

//! Makaro rooms exclude black arrow cells; adjacent boundaries are implicit.
use super::{g_yajikazu as arrows, Genre};
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;

/// `encodeBorder_makaro`: encode implicit walls around black cells as walls.
pub(super) fn encode_block_borders(b: &Board) -> String {
    let mut copy = b.clone();
    for i in 0..b.borders.len() {
        let (x, y) = b.geom.border_pos(i);
        let sides = if x % 2 == 0 {
            [(x - 1, y), (x + 1, y)]
        } else {
            [(x, y - 1), (x, y + 1)]
        };
        if sides
            .iter()
            .any(|&(x, y)| b.obj(Group::Cell, x, y).is_some_and(|c| c.ques == 1))
        {
            copy.borders[i].ques = 1;
        }
    }
    encode::encode_border(&copy)
}

impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "makaro"
    }
    fn hasborder(&self) -> u8 {
        1
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn subnumbers(&self) -> bool {
        true
    }
    fn room_cell_excluded(&self, c: &crate::Obj) -> bool {
        c.ques == 1
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_border(b, s, d);
        let chars: Vec<char> = rest.chars().collect();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() {
            let ch = chars[i];
            match ch {
                '0'..='9' => b.cells[c].qnum = ch as i32 - '0' as i32 + 1,
                '-' => {
                    if let Some(n) = jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 10) {
                        b.cells[c].qnum = n as i32 + 1;
                    }
                    i += 2;
                }
                'a'..='e' => {
                    b.cells[c].ques = 1;
                    b.cells[c].qdir = jsnum::digit(ch, 36).unwrap() as i32 - 10;
                }
                'g'..='z' => c += jsnum::digit(ch, 36).unwrap() as usize - 16,
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
        let mut out = encode_block_borders(b);
        let mut count = 0;
        for c in &b.cells {
            let s = if (1..11).contains(&c.qnum) {
                (c.qnum - 1).to_string()
            } else if (11..100).contains(&c.qnum) {
                format!("-{}", c.qnum - 1)
            } else if c.ques == 1 {
                jsnum::to_radix((c.qdir + 10) as u32, 36)
            } else {
                count += 1;
                String::new()
            };
            if count == 0 {
                out.push_str(&s);
            } else if !s.is_empty() || count == 20 {
                out.push_str(&jsnum::to_radix(count + 15, 36));
                out.push_str(&s);
                count = 0;
            }
        }
        if count > 0 {
            out.push_str(&jsnum::to_radix(count + 15, 36));
        }
        Ok(out)
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_area_room(r, b, d)?;
        r.decode_cell(b, d, |c, s| {
            match s {
                "t" | "b" | "l" | "r" | "#" => {
                    c.ques = 1;
                    c.qdir = match s {
                        "t" => 1,
                        "b" => 2,
                        "l" => 3,
                        "r" => 4,
                        _ => 0,
                    };
                }
                "-" => c.qnum = -2,
                "." => {}
                _ => c.qnum = fileio::int(s)?,
            }
            Ok(())
        })?;
        fileio::decode_cell_anumsub(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_area_room(w, b);
        w.encode_cell(b, |c| {
            let s = if c.ques == 1 {
                match c.qdir {
                    1 => "t",
                    2 => "b",
                    3 => "l",
                    4 => "r",
                    _ => "#",
                }
                .into()
            } else if c.qnum >= 0 {
                c.qnum.to_string()
            } else if c.qnum == -2 {
                "-".into()
            } else {
                ".".into()
            };
            format!("{s} ")
        });
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data_before(&self, t: crate::Transform, b: &mut Board) {
        arrows::adjust_cell_qdirs(b, t);
    }
}

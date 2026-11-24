//! Dosufuwa rooms exclude blocked cells, marked as `#` in the room grid.
use super::{g_makaro, Genre};
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "dosufuwa"
    }
    fn hasborder(&self) -> u8 {
        1
    }
    fn hascross(&self) -> u8 {
        2
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
            if ch.is_ascii_digit() || ch.is_ascii_lowercase() {
                c += jsnum::digit(ch, 36).unwrap() as usize;
                if let Some(cell) = b.cells.get_mut(c) {
                    cell.ques = 1;
                }
            } else if ch == '.' {
                c += 35;
            }
            c += 1;
            if c >= b.cells.len() {
                i += 1;
                break;
            }
            i += 1;
        }
        chars.iter().skip(i).collect()
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut out = g_makaro::encode_block_borders(b);
        let mut n = 0;
        for c in &b.cells {
            if c.ques == 1 {
                out.push_str(&jsnum::to_radix(n, 36));
                n = 0;
            } else {
                n += 1;
                if n == 36 {
                    out.push('.');
                    n = 0;
                }
            }
        }
        Ok(out)
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.read_line();
        let ids = r.item_list(b.rows() as i32);
        for y in 0..b.rows() {
            for x in 0..b.cols() {
                let i = y * b.cols() + x;
                if ids.get(i).is_some_and(|v| v == "#") {
                    b.cells[i].ques = 1;
                }
                for (nx, ny) in [(x + 1, y), (x, y + 1)] {
                    if nx < b.cols() && ny < b.rows() {
                        let j = ny * b.cols() + nx;
                        let wall = ids.get(i) != ids.get(j);
                        b.obj_mut(Group::Border, (x + nx + 1) as i32, (y + ny + 1) as i32)
                            .unwrap()
                            .ques = i32::from(wall);
                    }
                }
            }
        }
        fileio::decode_cell_qanssub(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        let rooms = crate::rooms::components(b);
        let mut ids = vec![None; b.cells.len()];
        for (i, room) in rooms.iter().enumerate() {
            for &c in room {
                ids[c] = Some(i);
            }
        }
        w.write_line(&rooms.len().to_string());
        for row in ids.chunks(b.cols()) {
            let s = row
                .iter()
                .map(|x| x.map_or_else(|| "#".to_string(), |v| v.to_string()))
                .collect::<Vec<_>>()
                .join(" ")
                + " ";
            w.write_line(&s);
        }
        fileio::encode_cell_qanssub(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
}

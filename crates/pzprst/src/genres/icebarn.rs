//! Icebarn ice cells, directed borders, and entrance/exit addresses.
use super::{g_icelom as address, g_slalom as answers, g_yajikazu as arrows, Genre};
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{FileReader, FileWriter};
use crate::jsnum;
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
fn inner_count(b: &Board) -> usize {
    2 * b.cols() * b.rows() - b.cols() - b.rows()
}
fn decode_ice_old(b: &mut Board, s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut c = 0;
    let mut i = 0;
    while i < chars.len() {
        let n = jsnum::digit(chars[i], 32).unwrap_or(0);
        for bit in [8, 4, 2, 1] {
            if c < b.cells.len() {
                b.cells[c].ques = if n & bit != 0 { 6 } else { 0 };
                c += 1;
            }
        }
        if c >= b.cells.len() {
            break;
        }
        i += 1;
    }
    chars.iter().skip(i + 1).collect()
}
fn decode_arrows_old1(b: &mut Board, s: &str) -> String {
    let input: String = s.chars().skip(1).collect();
    let parts: Vec<&str> = input.split('/').collect();
    for (pass, part) in parts.iter().take(4).enumerate() {
        if part.is_empty() {
            continue;
        }
        for value in part.split('+') {
            if let Some(id) = jsnum::integer(value).filter(|n| *n >= 0) {
                let x = id as usize % b.cols();
                let y = id as usize / b.cols();
                if y < b.rows() {
                    let (bx, by) = if pass < 2 {
                        (2 * x as i32 + 1, 2 * y as i32 + 2)
                    } else {
                        (2 * x as i32 + 2, 2 * y as i32 + 1)
                    };
                    if let Some(edge) = b.obj_mut(crate::Group::Border, bx, by) {
                        edge.qdir = [1, 2, 3, 4][pass];
                    }
                }
            }
        }
    }
    format!(
        "/{}",
        parts.iter().skip(4).copied().collect::<Vec<_>>().join("/")
    )
}
fn decode_arrows_old2(b: &mut Board, s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut pos = 0;
    let limits = [(b.cols() - 1) * b.rows(), inner_count(b)];
    let mut id = 0;
    for (pass, limit) in limits.into_iter().enumerate() {
        while pos < chars.len() {
            let ch = chars[pos];
            pos += 1;
            if let Some(n) = ch.to_digit(10) {
                if let Some(index) = b.border_by_id(id) {
                    b.borders[index].qdir = if pass == 0 {
                        if n % 2 == 0 {
                            3
                        } else {
                            4
                        }
                    } else if n % 2 == 0 {
                        1
                    } else {
                        2
                    };
                }
                id += (n as usize >> 1) + 1;
            } else if ch.is_ascii_lowercase() {
                id += jsnum::digit(ch, 36).unwrap() as usize - 9;
            } else {
                id += 1;
            }
            if id >= limit {
                break;
            }
        }
    }
    chars.iter().skip(pos).collect()
}
fn decode_arrows(b: &mut Board, s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = inner_count(b);
    if n == 0 {
        return s.into();
    }
    let mut pos = 0;
    for pass in 0..2 {
        let mut id = 0;
        while pos < chars.len() {
            let ch = chars[pos];
            pos += 1;
            if ch == 'z' {
                id += 35;
            } else if let Some(v) = jsnum::digit(ch, 36) {
                id += v as usize;
                if id < n {
                    let idx = b.border_by_id(id).unwrap();
                    let (x, _) = b.geom.pzprjs_border_pos(id).unwrap();
                    b.borders[idx].qdir = match (x % 2 == 0, pass) {
                        (true, 0) => 3,
                        (true, _) => 4,
                        (false, 0) => 1,
                        _ => 2,
                    };
                }
                id += 1;
            } else {
                pos -= 1;
                break;
            }
            if id >= n {
                break;
            }
        }
    }
    chars.iter().skip(pos).collect()
}
fn encode_arrows(b: &Board) -> String {
    let n = inner_count(b);
    let mut out = String::new();
    for pass in 0..2 {
        let mut count = 0;
        for id in 0..n {
            let dir = b.borders[b.border_by_id(id).unwrap()].qdir;
            let matched = if pass == 0 {
                dir == 1 || dir == 3
            } else {
                dir == 2 || dir == 4
            };
            if matched {
                out.push_str(&jsnum::to_radix(count, 36));
                count = 0;
            } else {
                count += 1;
                if count >= 35 {
                    out.push('z');
                    count = 0;
                }
            }
        }
        if count > 0 {
            out.push_str(&jsnum::to_radix(count, 36));
        }
    }
    out
}
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "icebarn"
    }
    fn hasborder(&self) -> u8 {
        2
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn hasexcell(&self) -> u8 {
        2
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn input_flags(&self) -> &'static str {
        "cx"
    }
    fn init_board(&self, b: &mut Board) {
        address::Icelom("icelom").init_board(b);
    }
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_ice(b, s, d);
        let rest = decode_arrows(b, &rest);
        address::decode_inout(b, &rest, d);
        String::new()
    }
    fn decode_url_with_format(
        &self,
        b: &mut Board,
        s: &str,
        flags: &str,
        legacy: bool,
        d: &mut Diagnostics,
    ) -> String {
        if !legacy {
            return self.decode_url(b, s, d);
        }
        if flags.contains('c') {
            let parts: Vec<&str> = s.split('/').collect();
            let reordered = format!(
                "{}/{}/{}",
                parts.get(2).copied().unwrap_or(""),
                parts.first().copied().unwrap_or(""),
                parts.get(1).copied().unwrap_or("")
            );
            let rest = encode::decode_ice(b, &reordered, d);
            let rest = decode_arrows_old2(b, &rest);
            address::decode_inout(b, &rest, d);
        } else {
            let rest = decode_ice_old(b, s);
            let rest = decode_arrows_old1(b, &rest);
            address::decode_inout(b, &rest, d);
        }
        String::new()
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}{}",
            encode::encode_ice(b),
            encode_arrows(b),
            address::encode_inout(b)
        ))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        let i = r.read_line().and_then(jsnum::integer).unwrap_or(0).max(0) as usize;
        let o = r.read_line().and_then(jsnum::integer).unwrap_or(0).max(0) as usize;
        address::load_address(b, i, true, d);
        address::load_address(b, o, false, d);
        r.decode_cell(b, d, |c, s| {
            if s == "1" {
                c.ques = 6;
            }
            Ok(())
        })?;
        let geom = b.geom;
        let mut n = 0;
        r.decode_border(b, d, |edge, s| {
            let (x, _) = geom.border_pos(n);
            n += 1;
            edge.qdir = match (x % 2 == 0, s) {
                (true, "1") => 3,
                (true, "2") => 4,
                (false, "1") => 1,
                (false, "2") => 2,
                _ => edge.qdir,
            };
            Ok(())
        })?;
        answers::decode_border_arrow_ans(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.write_line(&address::id_for_index(b, address::marker(b, true)).to_string());
        w.write_line(&address::id_for_index(b, address::marker(b, false)).to_string());
        w.encode_cell(b, |c| {
            if c.ques == 6 {
                "1 ".into()
            } else {
                "0 ".into()
            }
        });
        w.encode_border(b, |e| {
            match e.qdir {
                1 | 3 => "1 ",
                2 | 4 => "2 ",
                _ => "0 ",
            }
            .into()
        });
        answers::encode_border_arrow_ans(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data_before(&self, t: crate::Transform, b: &mut Board) {
        arrows::adjust_border_qdirs(b, t);
    }
}

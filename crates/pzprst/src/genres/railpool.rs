//! Railpool number lists, rooms, blocked cells and loop answers.
use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "railpool"
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
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        let chars: Vec<char> = s.chars().collect();
        let (mut c, mut i, mut off) = (0usize, 0usize, false);
        while i < chars.len() {
            let mut ch = chars[i];
            if ch.is_ascii_digit() || ('a'..='j').contains(&ch) {
                let mut vals = Vec::new();
                while ch.is_ascii_digit() || ('a'..='j').contains(&ch) {
                    let mut cur = 0;
                    while ('a'..='j').contains(&ch) {
                        cur = (cur + jsnum::digit(ch, 36).unwrap() as i32 - 10) * 10;
                        i += 1;
                        ch = chars.get(i).copied().unwrap_or('\0');
                    }
                    let digit = jsnum::digit(ch, 10).unwrap_or(0) as i32;
                    cur += digit;
                    vals.push(if cur == 0 { -2 } else { digit });
                    i += 1;
                    ch = chars.get(i).copied().unwrap_or('\0');
                }
                b.cells[c].qnums = vals;
                i = i.saturating_sub(1);
                c += 1;
                off = true;
            } else if ('k'..='z').contains(&ch) {
                c += jsnum::digit(ch, 36).unwrap() as usize - 19;
                if off {
                    c -= 1;
                }
                off = false;
            }
            if c >= b.cells.len() || ch == '/' {
                break;
            }
            i += 1;
        }
        let mut rest: String = chars.iter().skip(i + 1).collect();
        if rest.starts_with('/') {
            rest.remove(0);
        }
        let rest = encode::decode_border(b, &rest, d);
        encode::decode_empty(b, &rest, d)
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut out = String::new();
        let mut count = 0;
        for cell in &b.cells {
            let mut s = String::new();
            if !cell.qnums.is_empty() {
                for &n in &cell.qnums {
                    let mut n = n;
                    let mut chars = Vec::new();
                    while n > 0 {
                        let digit = n % 10;
                        n /= 10;
                        chars.push(jsnum::to_radix(
                            (if n < 10 { digit } else { digit + 10 }) as u32,
                            36,
                        ));
                    }
                    if chars.is_empty() {
                        s.push('0');
                    } else {
                        for ch in chars.iter().rev() {
                            s.push_str(ch);
                        }
                    }
                }
            } else {
                count += 1;
            }
            if count == 0 {
                out.push_str(&s);
                count = 1;
            } else if !s.is_empty() || count == 16 {
                out.push_str(&jsnum::to_radix(count + 19, 36));
                out.push_str(&s);
                count = if s.is_empty() { 0 } else { 1 };
            }
        }
        if count > 1 {
            out.push_str(&jsnum::to_radix(count + 19, 36));
        } else {
            out.push('/');
        }
        out.push_str(&encode::encode_border(b));
        out.push_str(&encode::encode_empty(b));
        Ok(out)
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        r.decode_cell(b, d, |c, s| {
            if s != "." {
                c.qnums = s
                    .split(',')
                    .map(|x| if x == "-" { Ok(-2) } else { fileio::int(x) })
                    .collect::<std::result::Result<Vec<_>, _>>()?;
            }
            Ok(())
        })?;
        r.decode_cell(b, d, |c, s| {
            if s == "*" {
                c.ques = 7;
            }
            Ok(())
        })?;
        fileio::decode_area_room(r, b, d)?;
        fileio::decode_border_line(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |c| {
            if c.qnums.is_empty() {
                ". ".into()
            } else {
                format!(
                    "{} ",
                    c.qnums
                        .iter()
                        .map(|n| if *n >= 0 { n.to_string() } else { "-".into() })
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
        });
        w.encode_cell(b, |c| {
            if c.ques == 7 {
                "* ".into()
            } else {
                ". ".into()
            }
        });
        fileio::encode_area_room(w, b);
        fileio::encode_border_line(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
}

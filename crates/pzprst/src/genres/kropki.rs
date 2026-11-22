//! Kropki (pzprjs `variety/minarism.js`, pid `kropki`). Dots live on borders:
//! the URL stores their 1/2 values three per base-27 character, FileIO also
//! stores the minarism-style `a`/`b` direction marks and `?` (`qnum` -2).

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Kropki;

/// pzprjs `Position.UP` / `DN` / `LT` / `RT`.
const UP: i32 = 1;
const DN: i32 = 2;
const LT: i32 = 3;
const RT: i32 = 4;

struct Kropki;

impl Genre for Kropki {
    fn pid(&self) -> &'static str {
        "kropki"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_circle_kropki(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_circle_kropki(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_border_kropki(r, b, d)?;
        fileio::decode_cell_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_border_kropki(w, b);
        fileio::encode_cell_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let maxnum = b.cols().max(b.rows()) as i32;
        if b.cells
            .iter()
            .any(|c| !(c.anum == -1 || (1..=maxnum).contains(&c.anum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kropki cell answer outside empty / 1..max(cols, rows)",
            );
        }
        if b.borders
            .iter()
            .any(|e| !matches!(e.qnum, -2 | -1 | 1 | 2) || !(0..=RT).contains(&e.qdir))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kropki border value outside empty / ? / dot 1-2, or direction outside 0-4",
            );
        }
        if b.borders.iter().any(|e| e.qnum == -2 || e.qdir != 0) {
            d.push(
                DiagnosticCode::LossyUrl,
                "kropki border question marks and direction marks are dropped from the URL",
            );
        }
    }
}

/// `decodeCircle_kropki`: three border dots per base-27 character.
fn decode_circle_kropki(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.borders.len();
    let tri = [9, 3, 1];
    let pos = (n + 2) / 3;
    let pos = pos.min(chars.len());
    let mut c = 0usize;
    for &ca in &chars[..pos] {
        let num = match jsnum::digit(ca, 27) {
            Some(v) => v as i32,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("unexpected character {ca:?} decodes as three empty borders"),
                );
                0
            }
        };
        for &t in &tri {
            if c < n {
                let val = (num / t) % 3;
                if val > 0 {
                    b.borders[c].qnum = val;
                }
                c += 1;
            }
        }
    }
    chars[pos..].iter().collect()
}

/// `encodeCircle_kropki`: three border dots per base-27 character.
fn encode_circle_kropki(b: &Board) -> String {
    let tri = [9u32, 3, 1];
    let mut cm = String::new();
    let (mut num, mut pass) = (0usize, 0u32);
    for border in &b.borders {
        if border.qnum > 0 {
            pass += border.qnum as u32 * tri[num];
        }
        num += 1;
        if num == 3 {
            cm.push_str(&jsnum::to_radix(pass, 27));
            num = 0;
            pass = 0;
        }
    }
    if num > 0 {
        cm.push_str(&jsnum::to_radix(pass, 27));
    }
    cm
}

/// `decodeBorder` in `FileIO@kropki`: `a`/`b` are direction marks, `.` is `?`
/// and every other value goes to the border dot number.
fn decode_border_kropki(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    let vertical = (b.cols() - 1) * b.rows();
    let mut idx = 0usize;
    let mut bad = Vec::new();
    r.decode_border(b, d, |border, ca| {
        let horz = idx >= vertical;
        idx += 1;
        if ca == "a" {
            border.qdir = if horz { UP } else { LT };
        } else if ca == "b" {
            border.qdir = if horz { DN } else { RT };
        } else if ca == "." {
            border.qnum = -2;
        } else if ca != "0" {
            match jsnum::integer(ca) {
                Some(n) => border.qnum = n,
                None => bad.push(ca.to_string()),
            }
        }
        Ok(())
    })?;
    for ca in bad {
        d.push(
            DiagnosticCode::UnexpectedChar,
            format!("border dot {ca:?} is not a number; border left empty"),
        );
    }
    Ok(())
}

/// `encodeBorder` in `FileIO@kropki`: direction first, then `?`, then the dot.
fn encode_border_kropki(w: &mut FileWriter, b: &Board) {
    w.encode_border(b, |border| {
        if border.qdir == UP || border.qdir == LT {
            "a ".into()
        } else if border.qdir == DN || border.qdir == RT {
            "b ".into()
        } else if border.qnum == -2 {
            ". ".into()
        } else if border.qnum != -1 {
            format!("{} ", border.qnum)
        } else {
            "0 ".into()
        }
    });
}

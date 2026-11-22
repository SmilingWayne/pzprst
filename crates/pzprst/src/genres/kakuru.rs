//! Kakuru (pzprjs `variety/kakuru.js`, pid `kakuru`). The script is shared
//! with `numrope` and `sananko`; `kakuru` has no border layer and no config
//! flags. Question numbers go up to 44 in edit mode; answers are 1–9.

use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Kakuru;

struct Kakuru;

impl Genre for Kakuru {
    fn pid(&self) -> &'static str {
        "kakuru"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        decode_kakuru(b, body, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(encode_kakuru(b))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_cell_qnum(r, b, d)?;
        decode_cell_anum(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_qnum(w, b);
        encode_cell_anum(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=44).contains(&c.qnum))
                || !matches!(c.ques, 0 | 1)
                || !(c.anum == -1 || (1..=9).contains(&c.anum))
                || !(0..=2).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "kakuru cell value outside empty / ? / 0..44, shading 0/1, answer 1..9 or sub mark 0..2",
            );
        }
    }
}

/// `decodeKakuru`: `+` is a shaded clue cell, `k`–`z` skip cells, `_` is `?`
/// and every other value goes through `decval`.
fn decode_kakuru(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() && c < n {
        let ca = chars[i];
        match ca {
            '+' => b.cells[c].ques = 1,
            'k'..='z' => c += jsnum::digit(ca, 36).unwrap() as usize - 19,
            '.' => {}
            _ => b.cells[c].qnum = decval(ca),
        }
        c += 1;
        i += 1;
        if c >= n {
            break;
        }
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    chars[i.min(chars.len())..].iter().collect()
}

/// `encodeKakuru`: numbers and `+` cells, `.` or `count + 18` in base 36 for blanks.
fn encode_kakuru(b: &Board) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in &b.cells {
        let mut pstr = String::new();
        if cell.qnum != -1 {
            pstr = encval(cell.qnum);
        } else if cell.ques == 1 {
            pstr = "+".into();
        } else {
            count += 1;
        }

        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 17 {
            if count == 1 {
                cm.push('.');
            } else {
                cm.push_str(&jsnum::to_radix(count + 18, 36));
            }
            cm.push_str(&pstr);
            count = 0;
        }
    }
    match count {
        1 => cm.push('.'),
        n if n > 1 => cm.push_str(&jsnum::to_radix(n + 18, 36)),
        _ => {}
    }
    cm
}

/// `decval`: `_` is `?`, `0`–`9` / `a`–`j` are 0–19, `A`–`Z` are 20–45.
fn decval(ca: char) -> i32 {
    if ca == '_' {
        -2
    } else if ca.is_ascii_digit() {
        ca as i32 - '0' as i32
    } else if ('a'..='j').contains(&ca) {
        10 + ca as i32 - 'a' as i32
    } else if ca.is_ascii_uppercase() {
        10 + jsnum::digit(ca, 36).unwrap() as i32
    } else {
        -1
    }
}

/// `encval`: `?` is `_`, 1–19 are lowercase base 36, 20–45 uppercase, the rest `0`.
fn encval(val: i32) -> String {
    if val == -2 {
        "_".into()
    } else if (1..=19).contains(&val) {
        jsnum::to_radix(val as u32, 36)
    } else if (20..=45).contains(&val) {
        jsnum::to_radix((val - 10) as u32, 36).to_uppercase()
    } else {
        "0".into()
    }
}

/// `FileIO.decodeData` question layer: `b` is a shaded cell, `?` is unknown.
fn decode_cell_qnum(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let mut bad = Vec::new();
    r.decode_cell(b, d, |cell, ca| {
        if ca == "b" {
            cell.ques = 1;
        } else if ca == "?" {
            cell.qnum = -2;
        } else if ca != "." {
            match jsnum::integer(ca) {
                Some(n) => cell.qnum = n,
                None => bad.push(ca.to_string()),
            }
        }
        Ok(())
    })?;
    for ca in bad {
        d.push(
            DiagnosticCode::UnexpectedChar,
            format!("question number {ca:?} is not a number; cell left empty"),
        );
    }
    Ok(())
}

/// `FileIO.encodeData` question layer.
fn encode_cell_qnum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        let ca = if cell.qnum == -2 {
            "?".to_string()
        } else if cell.qnum >= 0 {
            cell.qnum.to_string()
        } else if cell.ques == 1 {
            "b".into()
        } else {
            ".".into()
        };
        format!("{ca} ")
    });
}

/// `FileIO.decodeData` answer layer: `+` / `-` sub marks, `0` and `.` empty.
/// pzprjs also splits `[...]` snum arrays here; `Obj` has no `snum`.
fn decode_cell_anum(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    let mut bad = Vec::new();
    r.decode_cell(b, d, |cell, ca| {
        if ca == "+" {
            cell.qsub = 1;
        } else if ca == "-" {
            cell.qsub = 2;
        } else if ca != "." && ca != "0" {
            match jsnum::integer(ca) {
                Some(n) => cell.anum = n,
                None => bad.push(ca.to_string()),
            }
        }
        Ok(())
    })?;
    for ca in bad {
        d.push(
            DiagnosticCode::UnexpectedChar,
            format!("answer {ca:?} is not a number; cell left empty"),
        );
    }
    Ok(())
}

/// `FileIO.encodeData` answer layer; snum arrays are empty, so nothing is appended.
fn encode_cell_anum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        let ca = if cell.ques != 1 && cell.qnum == -1 {
            if cell.anum != -1 {
                cell.anum.to_string()
            } else if cell.qsub == 1 {
                "+".into()
            } else if cell.qsub == 2 {
                "-".into()
            } else {
                "0".into()
            }
        } else {
            ".".into()
        };
        format!("{ca} ")
    });
}

//! Yosenabe (pzprjs `variety/yosenabe.js`, pid `yosenabe`). A clue is `qnum`
//! or, when `qnum` is empty, a `qnum2` secondary number; `ques` 6 marks an ice
//! cell. The URL is the base-32 ice layer (`decodeIce` / `encodeIce`) followed
//! by the `decodeNumber16_yosenabe` clue layer; FileIO stores ice as an `i`
//! prefix, clues as `o`-prefixed or bare numbers, and a second cell layer
//! packing `qsub` and `qcmp` (`filever` 1).

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Error, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;

pub(crate) static GENRE: &dyn Genre = &Yosenabe;

struct Yosenabe;

impl Genre for Yosenabe {
    fn pid(&self) -> &'static str {
        "yosenabe"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_ice(b, body, d);
        decode_number16_yosenabe(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}",
            encode::encode_ice(b),
            encode_number16_yosenabe(b)
        ))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        match r.filever {
            1 => {
                decode_cell_yosenabe(r, b, d)?;
                fileio::decode_border_line(r, b, d)?;
                decode_cell_qsub_qcmp(r, b, d)
            }
            0 => {
                decode_cell_yosenabe(r, b, d)?;
                fileio::decode_border_line(r, b, d)
            }
            v => Err(Error::UnsupportedFileVersion(format!(
                "yosenabe pzprv3.{v}"
            ))),
        }
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.filever = 1;
        encode_cell_yosenabe(w, b);
        fileio::encode_border_line(w, b);
        encode_cell_qsub_qcmp(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        let mut bad_value = false;
        let mut lossy_qnum = false;
        let mut lossy_qnum2 = false;
        for cell in &b.cells {
            if !(cell.ques == 0 || cell.ques == 6)
                || !(cell.qnum == -1 || cell.qnum == -2 || (1..=999).contains(&cell.qnum))
                || !(cell.qnum2 == -1 || cell.qnum2 == -2 || (1..=999).contains(&cell.qnum2))
                || !(0..=15).contains(&cell.qsub)
                || cell.qcmp < 0
            {
                bad_value = true;
            }
            if cell.qnum >= 256 {
                lossy_qnum = true;
            }
            if cell.qnum2 >= 256 || (cell.qnum2 != -1 && cell.qnum != -1) {
                lossy_qnum2 = true;
            }
        }
        if bad_value {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "yosenabe cell value outside ice 0/6, clue 1-999/?, sub mark 0-15 or a negative done mark",
            );
        }
        if lossy_qnum {
            d.push(
                DiagnosticCode::LossyUrl,
                "cell numbers of 256 or more are dropped from a yosenabe URL",
            );
        }
        if lossy_qnum2 {
            d.push(
                DiagnosticCode::LossyUrl,
                "a second cell number is dropped from a yosenabe URL when the first one is present",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.line) || !(e.qsub == 0 || e.qsub == 2))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "yosenabe border value outside line 0/1, cross -1",
            );
        }
    }
}

/// `decodeNumber16_yosenabe`: hex clues (`-` escapes 16–255), `.` for a `?`,
/// `i`/`g` for a `qnum2` below 256, `h` for `qnum2` = `?`, and `j`–`z` blank
/// run markers that skip `digit - 19` cells.
fn decode_number16_yosenabe(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        let Some(cell) = b.cells.get_mut(c) else {
            break;
        };
        if ca.is_ascii_digit() || ('a'..='f').contains(&ca) {
            cell.qnum = ca.to_digit(16).unwrap() as i32;
        } else if ca == '-' {
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qn) => cell.qnum = qn as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal digits after '-' leave the cell empty",
                ),
            }
            i += 2;
        } else if ca == '.' {
            cell.qnum = -2;
        } else if ca == 'i' {
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 1), 16) {
                Some(qd) => cell.qnum2 = qd as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal digit after 'i' leaves the second number empty",
                ),
            }
            i += 1;
        } else if ca == 'g' {
            match jsnum::parse_int(&jsnum::substr(&chars, i + 1, 2), 16) {
                Some(qd) => cell.qnum2 = qd as i32,
                None => d.push(
                    DiagnosticCode::UnexpectedChar,
                    "no hexadecimal digits after 'g' leave the second number empty",
                ),
            }
            i += 2;
        } else if ca == 'h' {
            cell.qnum2 = -2;
        } else if ('j'..='z').contains(&ca) {
            c += jsnum::digit(ca, 36).unwrap() as usize - 19;
        } else {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} is skipped"),
            );
        }
        c += 1;
        if c >= n {
            break;
        }
        i += 1;
    }
    if c < n {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    chars[(i + 1).min(chars.len())..].iter().collect()
}

/// `encodeNumber16_yosenabe`: `qnum` first (`-` for 16–255), then `qnum2`
/// (`i`/`g`/`h`), with `18 + count` in base 36 for blank runs, flushed after
/// 17 blanks like pzprjs.
fn encode_number16_yosenabe(b: &Board) -> String {
    let mut count = 0u32;
    let mut cm = String::new();
    for cell in &b.cells {
        let (qn, qd) = (cell.qnum, cell.qnum2);
        let mut pstr = String::new();
        if qn == -2 {
            pstr = ".".into();
        } else if (0..16).contains(&qn) {
            pstr = jsnum::to_radix(qn as u32, 16);
        } else if (16..256).contains(&qn) {
            pstr = format!("-{}", jsnum::to_radix(qn as u32, 16));
        } else if qd == -2 {
            pstr = "h".into();
        } else if (0..16).contains(&qd) {
            pstr = format!("i{}", jsnum::to_radix(qd as u32, 16));
        } else if (16..256).contains(&qd) {
            pstr = format!("g{}", jsnum::to_radix(qd as u32, 16));
        } else {
            count += 1;
        }
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 17 {
            cm.push_str(&jsnum::to_radix(18 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(18 + count, 36));
    }
    cm
}

/// `FileIO@yosenabe.decodeData`'s cell layer: `i` marks ice, `o` starts the
/// primary number (bare `o` is `?`), and any other non-`.` token is `qnum2`.
fn decode_cell_yosenabe(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        let mut ca = ca;
        if let Some(rest) = ca.strip_prefix('i') {
            cell.ques = 6;
            ca = rest;
        }
        if let Some(rest) = ca.strip_prefix('o') {
            cell.qnum = if rest.is_empty() {
                -2
            } else {
                fileio::int(rest)?
            };
        } else if !ca.is_empty() && ca != "." {
            cell.qnum2 = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `FileIO@yosenabe.encodeData`'s cell layer.
fn encode_cell_yosenabe(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        let mut ca = String::new();
        if cell.ques == 6 {
            ca.push('i');
        }
        if cell.qnum != -1 {
            ca.push('o');
            if cell.qnum >= 0 {
                ca.push_str(&cell.qnum.to_string());
            }
        } else if cell.qnum2 > 0 {
            ca.push_str(&cell.qnum2.to_string());
        }
        if ca.is_empty() {
            ca.push('.');
        }
        ca.push(' ');
        ca
    });
}

/// `decodeCellQsubQcmp`: `qsub` in the low four bits, `qcmp` in the high bits;
/// a `0` token leaves both empty.
fn decode_cell_qsub_qcmp(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca != "0" {
            let value = fileio::int(ca)?;
            cell.qsub = value & 0x0f;
            cell.qcmp = value >> 4;
        }
        Ok(())
    })
}

/// `encodeCellQsubQcmp`.
fn encode_cell_qsub_qcmp(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| format!("{} ", cell.qsub + (cell.qcmp << 4)));
}

//! URL body codecs (port of pzprjs `variety-common/Encode.js`).
//!
//! Decoders return the unconsumed rest of the body, like pzprjs `outbstr`.

use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Error, Result};
use crate::jsnum;

/// `decode4Cell`: cell numbers 0–4 with run-length blanks.
pub fn decode_4cell(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let mut c = 0usize;
    let mut i = 0usize;
    while i < chars.len() {
        let ca = chars[i];
        match ca {
            '0'..='4' => b.cells[c].qnum = ca as i32 - '0' as i32,
            '5'..='9' => {
                b.cells[c].qnum = ca as i32 - '5' as i32;
                c += 1;
            }
            'a'..='e' => {
                b.cells[c].qnum = ca as i32 - 'a' as i32;
                c += 2;
            }
            'g'..='z' => c += jsnum::digit(ca, 36).unwrap() as usize - 16,
            '.' => b.cells[c].qnum = -2,
            _ => d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} skips one cell"),
            ),
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
    chars.iter().skip(i + 1).collect()
}

/// `encode4Cell`.
pub fn encode_4cell(b: &Board) -> String {
    let cells = &b.cells;
    let mut count = 0u32;
    let mut cm = String::new();
    let mut c = 0usize;
    while c < cells.len() {
        let mut pstr = String::new();
        let qn = cells[c].qnum;
        if qn >= 0 {
            if c + 1 < cells.len() && cells[c + 1].qnum != -1 {
                pstr = jsnum::to_radix(qn as u32, 16);
            } else if c + 2 < cells.len() && cells[c + 2].qnum != -1 {
                pstr = jsnum::to_radix(5 + qn as u32, 16);
                c += 1;
            } else {
                pstr = jsnum::to_radix(10 + qn as u32, 16);
                c += 2;
            }
        } else if qn == -2 {
            pstr = ".".into();
        } else {
            count += 1;
        }

        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 20 {
            cm.push_str(&jsnum::to_radix(count + 15, 36));
            cm.push_str(&pstr);
            count = 0;
        }
        c += 1;
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(count + 15, 36));
    }
    cm
}

/// `genericDecodeTriple`: three base-3 values per base-27 character.
pub fn decode_triple(
    length: usize,
    bstr: &str,
    d: &mut Diagnostics,
    mut set: impl FnMut(usize, i32),
) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let pos = if chars.is_empty() {
        0
    } else {
        ((length + 2) / 3).min(chars.len())
    };
    if pos < (length + 2) / 3 && !chars.is_empty() {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    let mut c = 0usize;
    for &ch in &chars[..pos] {
        let ca = match jsnum::digit(ch, 27) {
            Some(v) => v as i32,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("unexpected character {ch:?} decodes as empty"),
                );
                0
            }
        };
        for tri in [9, 3, 1] {
            let val = (ca / tri) % 3;
            if val > 0 {
                if c < length {
                    set(c, val);
                } else {
                    d.push(
                        DiagnosticCode::TrailingData,
                        "non-zero padding after the last cell",
                    );
                }
            }
            c += 1;
        }
    }
    chars[pos..].iter().collect()
}

/// `genericEncodeTriple`; values must be 0–2 (pzprjs throws otherwise).
pub fn encode_triple(length: usize, mut get: impl FnMut(usize) -> i32) -> Result<String> {
    let mut cm = String::new();
    let (mut num, mut pass) = (0, 0u32);
    let tri = [9, 3, 1];
    for c in 0..length {
        let value = get(c);
        if !(0..=2).contains(&value) {
            return Err(Error::ValueOutOfRange(format!(
                "value {value} at index {c} cannot be URL-encoded"
            )));
        }
        pass += value as u32 * tri[num];
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
    Ok(cm)
}

/// `decodeCircle`: cell qnum 1/2.
pub fn decode_circle(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let n = b.cells.len();
    let cells = &mut b.cells;
    decode_triple(n, bstr, d, |c, val| cells[c].qnum = val)
}

/// `encodeCircle`: `max(0, qnum)` per cell.
pub fn encode_circle(b: &Board) -> Result<String> {
    encode_triple(b.cells.len(), |c| b.cells[c].qnum.max(0))
}

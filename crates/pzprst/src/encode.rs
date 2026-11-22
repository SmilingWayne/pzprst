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

/// `decodeOuterBorder`: border `ques` bits, five per base-32 digit, in pzprjs
/// border order (inner borders first, the outer frame last).
pub fn decode_outer_border(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let total = b.borders.len();
    let inner = 2 * b.cols() * b.rows() - b.cols() - b.rows();
    let pos = if chars.is_empty() {
        0
    } else {
        ((total + 4) / 5).min(chars.len())
    };
    let twi = [16, 8, 4, 2, 1];
    let mut id = 0usize;
    for &ch in &chars[..pos] {
        let ca = match jsnum::parse_int(&ch.to_string(), 32) {
            Some(v) => v as i32,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("unexpected character {ch:?} decodes as empty"),
                );
                0
            }
        };
        for t in twi {
            if id >= total {
                break;
            }
            if let Some(i) = b.border_by_id(id) {
                b.borders[i].ques = if ca & t != 0 { 1 } else { 0 };
            }
            id += 1;
        }
    }
    if pos * 5 < inner {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last inner border",
        );
    }
    chars[pos..].iter().collect()
}

/// `encodeOuterBorder`: border `ques` bits, five per base-32 digit. Every pzprjs
/// user of this encoder defines `isLineNG` as `ques == 1`; the outer frame is
/// written only when one of its marks is set.
pub fn encode_outer_border(b: &Board) -> String {
    let (c, r) = (b.cols(), b.rows());
    let total = b.borders.len();
    let inside = (2 * c * r - c - r).min(total);
    let hasoutside =
        (inside..total).any(|id| b.border_by_id(id).is_some_and(|i| b.borders[i].ques == 1));
    let max = if hasoutside { total } else { inside };
    let twi = [16, 8, 4, 2, 1];
    let mut cm = String::new();
    let (mut num, mut pass) = (0usize, 0u32);
    for id in 0..max {
        if let Some(i) = b.border_by_id(id) {
            if b.borders[i].ques == 1 {
                pass += twi[num];
            }
        }
        num += 1;
        if num == 5 {
            cm.push_str(&jsnum::to_radix(pass, 32));
            num = 0;
            pass = 0;
        }
    }
    if num > 0 {
        cm.push_str(&jsnum::to_radix(pass, 32));
    }
    cm
}

/// `readNumber16`: one value written as a lowercase hex digit, a `-`/`+`/`=`/`@`
/// (or `%`)/`*`/`$` escape, or `.` (-2). Returns the value and the length of the
/// text pzprjs would consume; `Some(-1)` means no value (the caller skips one
/// character) and `None` is JavaScript's NaN (a truncated or non-hex escape).
pub fn read_number16(chars: &[char], i: usize) -> (Option<i32>, usize) {
    let Some(&ca) = chars.get(i) else {
        return (Some(-1), 0);
    };
    // parseInt(substr(i + 1, len), 16), plus the escape's offset.
    let escape = |len: usize, offset: i32| {
        jsnum::parse_int(&jsnum::substr(chars, i + 1, len), 16).map(|v| v as i32 + offset)
    };
    match ca {
        '0'..='9' | 'a'..='f' => (Some(jsnum::digit(ca, 16).unwrap() as i32), 1),
        '-' => (escape(2, 0), 3),
        '+' => (escape(3, 0), 4),
        '=' => (escape(3, 4096), 4),
        '%' | '@' => (escape(3, 8192), 4),
        '*' => (escape(4, 12240), 5),
        '$' => (escape(5, 77776), 6),
        '.' => (Some(-2), 1),
        _ => (Some(-1), 0),
    }
}

/// `writeNumber16`: the text of one value, `""` for values pzprjs cannot store
/// (every negative except -2). Takes `i64` because pzprjs passes any JS number
/// while pzprst's properties are `i32`; `2 * qnum + ques` must not overflow.
pub fn write_number16(qn: i64) -> String {
    match qn {
        -2 => ".".into(),
        0..=15 => jsnum::to_radix(qn as u32, 16),
        16..=255 => format!("-{}", jsnum::to_radix(qn as u32, 16)),
        256..=4095 => format!("+{}", jsnum::to_radix(qn as u32, 16)),
        4096..=8191 => format!("={:0>3}", jsnum::to_radix((qn - 4096) as u32, 16)),
        8192..=12239 => format!("@{:0>3}", jsnum::to_radix((qn - 8192) as u32, 16)),
        12240..=77775 => format!("*{:0>4}", jsnum::to_radix((qn - 12240) as u32, 16)),
        _ if qn >= 77776 => format!("${:0>5}", jsnum::to_radix((qn - 77776) as u32, 16)),
        _ => String::new(),
    }
}

/// `decodeNumber16`: cell numbers, one hex digit, a `.` for `?`, or a
/// `-`/`+`/`=`/`@`/`*`/`$` hex escape; `g`–`z` skip cells.
pub fn decode_number16(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let cells = &mut b.cells;
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() && c < n {
        let ca = chars[i];
        let (val, used) = read_number16(&chars, i);
        match val {
            Some(v) if v != -1 => {
                cells[c].qnum = v;
                i += used;
                c += 1;
            }
            Some(_) => {
                if ('g'..='z').contains(&ca) {
                    c += jsnum::digit(ca, 36).unwrap() as usize - 15;
                    i += 1;
                } else {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("unexpected character {ca:?} is skipped"),
                    );
                    i += 1;
                }
            }
            None => {
                // pzprjs stores JavaScript's NaN here; keep the empty default.
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("incomplete hex escape at offset {i} leaves cell {c} empty"),
                );
                i += used;
                c += 1;
            }
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

/// `encodeNumber16`: cell qnums, with base-36 run lengths for empty cells.
pub fn encode_number16(b: &Board) -> String {
    let mut count = 0u32;
    let mut cm = String::new();
    for cell in &b.cells {
        let pstr = write_number16(i64::from(cell.qnum));
        if pstr.is_empty() {
            count += 1;
        }
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 20 {
            cm.push_str(&jsnum::to_radix(15 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(15 + count, 36));
    }
    cm
}

/// `decodeIce`: `ques = 6` cells, five bits per base-32 character (`decodeBinary`).
pub fn decode_ice(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let twi = [16, 8, 4, 2, 1];
    let mut c = 0usize;
    let mut i = 0usize;
    while i < chars.len() {
        let ca = chars[i];
        let num = match jsnum::digit(ca, 32) {
            Some(v) => v,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("character {ca:?} decodes as an all-zero ice digit"),
                );
                0
            }
        };
        for &bit in &twi {
            if c < n {
                if num & bit != 0 {
                    b.cells[c].ques = 6;
                }
                c += 1;
            }
        }
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

/// `encodeIce`: `ques == 6` cells, five bits per base-32 character (`encodeBinary`).
pub fn encode_ice(b: &Board) -> String {
    let twi = [16, 8, 4, 2, 1];
    let mut cm = String::new();
    let mut num = 0usize;
    let mut pass = 0u32;
    for cell in &b.cells {
        if cell.ques == 6 {
            pass += twi[num];
        }
        num += 1;
        if num == 5 {
            cm.push_str(&jsnum::to_radix(pass, 32));
            num = 0;
            pass = 0;
        }
    }
    if num > 0 {
        cm.push_str(&jsnum::to_radix(pass, 32));
    }
    cm
}

/// `decodeNumber10`: cell clues `0`–`9` and `?` (`.`), blanks as base-36 run markers.
pub fn decode_number10(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let n = b.cells.len();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() {
        let ca = chars[i];
        match ca {
            '.' => b.cells[c].qnum = -2,
            '0'..='9' => b.cells[c].qnum = ca as i32 - '0' as i32,
            'a'..='z' => c += jsnum::digit(ca, 36).unwrap() as usize - 10,
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

/// `encodeNumber10`: one character per cell, blanks as base-36 run markers.
/// pzprjs throws for a clue of 10 or more; `maybeEncodeNumber10` is inlined.
pub fn encode_number10(b: &Board) -> Result<String> {
    let cells = &b.cells;
    let mut cm = String::new();
    let mut count = 0u32;
    for cell in cells {
        let qn = cell.qnum;
        let mut pstr = String::new();
        if qn == -2 {
            pstr.push('.');
        } else if (0..10).contains(&qn) {
            pstr = qn.to_string();
        } else if qn >= 10 {
            return Err(Error::ValueOutOfRange(
                "grid contains a number higher than 9".into(),
            ));
        } else {
            count += 1;
        }

        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 26 {
            cm.push_str(&jsnum::to_radix(9 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(9 + count, 36));
    }
    Ok(cm)
}

/// `decodeNumber10or16`: `decodeNumber10`, or `decodeNumber16` after a leading
/// `-` (base 10 is the backwards-compatible default, base 16 the full-range form).
pub fn decode_number10or16(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let mut chars = bstr.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    if first == '-' {
        number10or16::decode_number16(b, chars.as_str(), d)
    } else {
        number10or16::decode_number10(b, bstr, d)
    }
}

/// `encodeNumber10or16`: `maybeEncodeNumber10` when every clue fits, otherwise
/// `-` followed by `encodeNumber16`.
pub fn encode_number10or16(b: &Board) -> String {
    let cm = number10or16::maybe_encode_number10(b);
    if !cm.is_empty() {
        cm
    } else {
        let mut out = String::from("-");
        out.push_str(&number10or16::encode_number16(b));
        out
    }
}

/// Helpers for `decodeNumber10or16` and `encodeNumber10or16`: one-to-one ports
/// of the pzprjs number functions they call. They stay private to these two
/// codecs; promoting them to shared codecs is a separate change.
mod number10or16 {
    use crate::board::Board;
    use crate::error::{DiagnosticCode, Diagnostics};
    use crate::jsnum;

    /// What pzprjs `readNumber16` reports: a value, a value that is `NaN` in
    /// JavaScript, or no number at this position (pzprjs `[-1, 0]`).
    enum ReadNumber16 {
        Value(i32),
        Nan,
        None,
    }

    /// Port of pzprjs `readNumber16`: one hex digit, a `-`/`+`/`=`/`%`/`@`/
    /// `*`/`$` escape, `.` for `?`, or no number. Also returns the digits
    /// consumed, which can run past the end of a truncated body like `substr`.
    fn read_number16(chars: &[char], i: usize) -> (ReadNumber16, usize) {
        let ca = chars[i];
        if ca.is_ascii_digit() {
            return (ReadNumber16::Value(ca as i32 - '0' as i32), 1);
        }
        if ('a'..='f').contains(&ca) {
            return (ReadNumber16::Value(ca as i32 - 'a' as i32 + 10), 1);
        }
        let (digits, offset) = match ca {
            '-' => (2, 0),
            '+' => (3, 0),
            '=' => (3, 4096),
            '%' | '@' => (3, 8192),
            '*' => (4, 12240),
            '$' => (5, 77776),
            '.' => return (ReadNumber16::Value(-2), 1),
            _ => return (ReadNumber16::None, 0),
        };
        let body = jsnum::substr(chars, i + 1, digits);
        match jsnum::parse_int(&body, 16) {
            Some(v) => (ReadNumber16::Value(v as i32 + offset), digits + 1),
            None => (ReadNumber16::Nan, digits + 1),
        }
    }

    /// Port of pzprjs `genericDecodeNumber16`.
    pub(super) fn generic_decode_number16(
        length: usize,
        bstr: &str,
        d: &mut Diagnostics,
        mut set: impl FnMut(usize, i32),
    ) -> String {
        let chars: Vec<char> = bstr.chars().collect();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() && c < length {
            let ca = chars[i];
            match read_number16(&chars, i) {
                // pzprjs `readNumber16` uses -1 as its "no number" sentinel.
                (ReadNumber16::Value(-1), _) => {
                    if ('g'..='z').contains(&ca) {
                        c += jsnum::digit(ca, 36).unwrap() as usize - 15;
                    } else {
                        d.push(
                            DiagnosticCode::UnexpectedChar,
                            format!("unexpected character {ca:?} is skipped"),
                        );
                    }
                    i += 1;
                }
                (ReadNumber16::Value(val), used) => {
                    set(c, val);
                    i += used;
                    c += 1;
                }
                (ReadNumber16::Nan, used) => {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        "number escape is not hexadecimal; the cell stays empty",
                    );
                    i += used;
                    c += 1;
                }
                (ReadNumber16::None, _) => {
                    if ('g'..='z').contains(&ca) {
                        c += jsnum::digit(ca, 36).unwrap() as usize - 15;
                    } else {
                        d.push(
                            DiagnosticCode::UnexpectedChar,
                            format!("unexpected character {ca:?} is skipped"),
                        );
                    }
                    i += 1;
                }
            }
        }
        if c < length {
            d.push(
                DiagnosticCode::TruncatedBody,
                "body ends before the last cell",
            );
        }
        chars.iter().skip(i).collect()
    }

    /// Port of pzprjs `decodeNumber16`.
    pub(super) fn decode_number16(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
        let length = b.cells.len();
        let cells = &mut b.cells;
        generic_decode_number16(length, bstr, d, |c, val| cells[c].qnum = val)
    }

    /// Port of pzprjs `decodeNumber10`.
    pub(super) fn decode_number10(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
        let chars: Vec<char> = bstr.chars().collect();
        let n = b.cells.len();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() {
            let ca = chars[i];
            match ca {
                '.' => b.cells[c].qnum = -2,
                '0'..='9' => b.cells[c].qnum = ca as i32 - '0' as i32,
                'a'..='z' => c += jsnum::digit(ca, 36).unwrap() as usize - 10,
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

    /// Port of pzprjs `maybeEncodeNumber10`; empty when a value needs base 16.
    pub(super) fn maybe_encode_number10(b: &Board) -> String {
        let mut cm = String::new();
        let mut count = 0u32;
        for cell in &b.cells {
            let qn = cell.qnum;
            let mut pstr = String::new();
            if qn == -2 {
                pstr.push('.');
            } else if (0..10).contains(&qn) {
                pstr = qn.to_string();
            } else if qn >= 10 {
                return String::new();
            } else {
                count += 1;
            }
            if count == 0 {
                cm.push_str(&pstr);
            } else if !pstr.is_empty() || count == 26 {
                cm.push_str(&jsnum::to_radix(9 + count, 36));
                cm.push_str(&pstr);
                count = 0;
            }
        }
        if count > 0 {
            cm.push_str(&jsnum::to_radix(9 + count, 36));
        }
        cm
    }

    /// Port of pzprjs `writeNumber16`; empty for blanks (and for negative values
    /// other than `?`).
    fn write_number16(qn: i32) -> String {
        if qn == -2 {
            ".".into()
        } else if (0..16).contains(&qn) {
            jsnum::to_radix(qn as u32, 16)
        } else if (16..256).contains(&qn) {
            format!("-{}", jsnum::to_radix(qn as u32, 16))
        } else if (256..4096).contains(&qn) {
            format!("+{}", jsnum::to_radix(qn as u32, 16))
        } else if (4096..8192).contains(&qn) {
            format!("={:0>3}", jsnum::to_radix((qn - 4096) as u32, 16))
        } else if (8192..12240).contains(&qn) {
            format!("@{:0>3}", jsnum::to_radix((qn - 8192) as u32, 16))
        } else if (12240..77776).contains(&qn) {
            format!("*{:0>4}", jsnum::to_radix((qn - 12240) as u32, 16))
        } else if qn >= 77776 {
            format!("${:0>5}", jsnum::to_radix((qn - 77776) as u32, 16))
        } else {
            String::new()
        }
    }

    /// Port of pzprjs `genericEncodeNumber16`.
    pub(super) fn generic_encode_number16(
        length: usize,
        mut get: impl FnMut(usize) -> i32,
    ) -> String {
        let mut cm = String::new();
        let mut count = 0u32;
        for c in 0..length {
            let pstr = write_number16(get(c));
            if pstr.is_empty() {
                count += 1;
            }
            if count == 0 {
                cm.push_str(&pstr);
            } else if !pstr.is_empty() || count == 20 {
                cm.push_str(&jsnum::to_radix(15 + count, 36));
                cm.push_str(&pstr);
                count = 0;
            }
        }
        if count > 0 {
            cm.push_str(&jsnum::to_radix(15 + count, 36));
        }
        cm
    }

    /// Port of pzprjs `encodeNumber16`.
    pub(super) fn encode_number16(b: &Board) -> String {
        generic_encode_number16(b.cells.len(), |c| b.cells[c].qnum)
    }
}

// ----- border walls (shared by every genre with a border.ques layer) -------

/// `decodeBorder`: border walls as two base-32 runs, five 5-bit groups per
/// character. Ids are pzprjs border ids: inner vertical borders first, then the
/// inner horizontal ones (only `inside` borders, so the outer frame of a
/// hasborder-2 board is not part of this layer).
pub fn decode_border(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let (cols, rows) = (b.cols(), b.rows());
    let vertical = (cols - 1) * rows;
    let horizontal = cols * (rows - 1);
    let (pos1, pos2) = if chars.is_empty() {
        (0, 0)
    } else {
        let pos1 = ((vertical + 4) / 5).min(chars.len());
        let pos2 = ((horizontal + 4) / 5 + pos1).min(chars.len());
        (pos1, pos2)
    };
    if chars.len() < (vertical + 4) / 5 + (horizontal + 4) / 5 {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last border group",
        );
    }
    let twi = [16, 8, 4, 2, 1];
    let mut id = 0usize;
    for &ca in &chars[..pos1] {
        let ca = border_bits(ca, d);
        for t in twi {
            if id < vertical {
                if let Some(index) = b.border_by_id(id) {
                    b.borders[index].ques = if ca & t != 0 { 1 } else { 0 };
                }
                id += 1;
            }
        }
    }
    let mut id = vertical;
    for &ca in &chars[pos1..pos2] {
        let ca = border_bits(ca, d);
        for t in twi {
            if id < vertical + horizontal {
                if let Some(index) = b.border_by_id(id) {
                    b.borders[index].ques = if ca & t != 0 { 1 } else { 0 };
                }
                id += 1;
            }
        }
    }
    // pzprjs calls `bd.roommgr.rebuild()` here; pzprst models no rooms, so no-op.
    chars[pos2..].iter().collect()
}

/// `encodeBorder`: inner border walls, two base-32 runs (vertical, horizontal).
pub fn encode_border(b: &Board) -> String {
    let (cols, rows) = (b.cols(), b.rows());
    let vertical = (cols - 1) * rows;
    let horizontal = cols * (rows - 1);
    let twi = [16, 8, 4, 2, 1];
    let mut cm = String::new();
    for range in [0..vertical, vertical..vertical + horizontal] {
        let mut num = 0usize;
        let mut pass = 0i64;
        for id in range {
            let ques = b.border_by_id(id).map_or(0, |i| b.borders[i].ques);
            pass += i64::from(ques) * i64::from(twi[num]);
            num += 1;
            if num == 5 {
                cm.push_str(&base32(pass));
                num = 0;
                pass = 0;
            }
        }
        if num > 0 {
            cm.push_str(&base32(pass));
        }
    }
    cm
}

/// pzprjs `(pass).toString(32)` for the border-group accumulator.
fn base32(pass: i64) -> String {
    let mut digits = Vec::new();
    let mut n = pass.unsigned_abs();
    loop {
        digits.push(std::char::from_digit((n % 32) as u32, 32).expect("base-32 digit"));
        n /= 32;
        if n == 0 {
            break;
        }
    }
    let digits: String = digits.iter().rev().collect();
    if pass < 0 {
        format!("-{digits}")
    } else {
        digits
    }
}

/// pzprjs `parseInt(ca, 32)` for one border group: `NaN` clears the five walls.
fn border_bits(ca: char, d: &mut Diagnostics) -> u32 {
    match jsnum::digit(ca, 32) {
        Some(v) => v,
        None => {
            d.push(
                DiagnosticCode::UnexpectedChar,
                format!("unexpected character {ca:?} clears five borders"),
            );
            0
        }
    }
}
/// `genericDecodeBinary`: five boolean values per base-32 character.
pub(crate) fn generic_decode_binary(
    length: usize,
    bstr: &str,
    d: &mut Diagnostics,
    mut set: impl FnMut(usize, bool),
) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let twi = [16, 8, 4, 2, 1];
    let mut c = 0usize;
    let mut i = 0usize;
    while i < chars.len() {
        let ca = chars[i];
        let num = match jsnum::digit(ca, 32) {
            Some(v) => v,
            None => {
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("unexpected character {ca:?} decodes as empty"),
                );
                0
            }
        };
        for bit in twi {
            if c < length {
                set(c, num & bit != 0);
                c += 1;
            }
        }
        if c >= length {
            break;
        }
        i += 1;
    }
    if c < length && !chars.is_empty() {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    chars[(i + 1).min(chars.len())..].iter().collect()
}

/// `genericEncodeBinary`: five boolean values per base-32 character; with
/// `skipnone` an all-false board encodes as an empty body.
pub(crate) fn generic_encode_binary(
    length: usize,
    mut get: impl FnMut(usize) -> bool,
    skipnone: bool,
) -> String {
    let twi = [16u32, 8, 4, 2, 1];
    let mut cm = String::new();
    let (mut num, mut pass) = (0usize, 0u32);
    let mut found = false;
    for c in 0..length {
        if get(c) {
            pass += twi[num];
            found = true;
        }
        num += 1;
        if num == 5 {
            cm.push_str(&jsnum::to_radix(pass, 32));
            num = 0;
            pass = 0;
        }
    }
    if num > 0 {
        cm.push_str(&jsnum::to_radix(pass, 32));
    }
    if found || !skipnone {
        cm
    } else {
        String::new()
    }
}

/// `decodeBinary`: `cell[prop] = val` for every cell whose bit is set; `set`
/// receives exactly those cells.
pub fn decode_binary(
    b: &mut Board,
    bstr: &str,
    d: &mut Diagnostics,
    mut set: impl FnMut(&mut crate::Obj),
) -> String {
    let n = b.cells.len();
    let cells = &mut b.cells;
    generic_decode_binary(n, bstr, d, |c, on| {
        if on {
            set(&mut cells[c]);
        }
    })
}

/// `encodeBinary`: one bit per cell, `get` says whether the property holds;
/// with `skipnone` an all-false board encodes as an empty body.
pub fn encode_binary(
    b: &Board,
    mut get: impl FnMut(&crate::Obj) -> bool,
    skipnone: bool,
) -> String {
    let cells = &b.cells;
    generic_encode_binary(cells.len(), |c| get(&cells[c]), skipnone)
}

/// `decodeEmpty`: cell `ques` = 7 (a shaded cell), five cells per base-32 character.
pub fn decode_empty(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String {
    let n = b.cells.len();
    let cells = &mut b.cells;
    generic_decode_binary(n, bstr, d, |c, on| {
        if on {
            cells[c].ques = 7;
        }
    })
}

/// `encodeEmpty`: one bit per cell, set when `ques` is 7; empty when none is set.
pub fn encode_empty(b: &Board) -> String {
    generic_encode_binary(b.cells.len(), |c| b.cells[c].ques == 7, true)
}

enum Number16 {
    /// `readNumber16` found no number (`[-1, 0]`).
    None,
    /// `parseInt` is `NaN`; carries the characters JS consumes.
    NaN(usize),
    /// The number and the characters consumed.
    Value(i32, usize),
}

/// `readNumber16`: one cell number, hex digits or an escape prefix.
fn generic_read_number16(chars: &[char], i: usize) -> Number16 {
    let ca = chars[i];
    let parse = |start: usize, len: usize, add: i32| match jsnum::parse_int(
        &jsnum::substr(chars, start, len),
        16,
    ) {
        Some(v) => Number16::Value(v as i32 + add, len + 1),
        None => Number16::NaN(len + 1),
    };
    if ca.is_ascii_digit() || ('a'..='f').contains(&ca) {
        Number16::Value(ca.to_digit(16).unwrap() as i32, 1)
    } else if ca == '-' {
        parse(i + 1, 2, 0)
    } else if ca == '+' {
        parse(i + 1, 3, 0)
    } else if ca == '=' {
        parse(i + 1, 3, 4096)
    } else if ca == '%' || ca == '@' {
        parse(i + 1, 3, 8192)
    } else if ca == '*' {
        parse(i + 1, 4, 12240)
    } else if ca == '$' {
        parse(i + 1, 5, 77776)
    } else if ca == '.' {
        Number16::Value(-2, 1)
    } else {
        Number16::None
    }
}

/// `genericDecodeNumber16`: cell numbers with run-length blanks.
pub fn generic_decode_number16(
    length: usize,
    bstr: &str,
    d: &mut Diagnostics,
    mut set: impl FnMut(usize, i32),
) -> String {
    let chars: Vec<char> = bstr.chars().collect();
    let (mut c, mut i) = (0usize, 0usize);
    while i < chars.len() && c < length {
        let ca = chars[i];
        match generic_read_number16(&chars, i) {
            // `readNumber16` reports -1 for "no number", so an escape that
            // parses to -1 (e.g. `--1`) falls into pzprjs's skip branch and
            // consumes its first character only.
            Number16::Value(-1, _) => {
                if ('g'..='z').contains(&ca) {
                    c += jsnum::digit(ca, 36).unwrap() as usize - 15;
                } else {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("unexpected character {ca:?} is skipped"),
                    );
                }
                i += 1;
            }
            Number16::Value(val, used) => {
                set(c, val);
                i += used;
                c += 1;
            }
            Number16::NaN(used) => {
                // pzprjs stores NaN here; pzprst leaves the object untouched
                // but still consumes the characters and the cell.
                d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("number after {ca:?} is not hexadecimal; cell left empty"),
                );
                i += used;
                c += 1;
            }
            Number16::None => {
                if ('g'..='z').contains(&ca) {
                    c += jsnum::digit(ca, 36).unwrap() as usize - 15;
                } else {
                    d.push(
                        DiagnosticCode::UnexpectedChar,
                        format!("unexpected character {ca:?} is skipped"),
                    );
                }
                i += 1;
            }
        }
    }
    if c < length {
        d.push(
            DiagnosticCode::TruncatedBody,
            "body ends before the last cell",
        );
    }
    chars[i.min(chars.len())..].iter().collect()
}

/// `writeNumber16`: one cell number as URL characters (`""` for empty).
fn generic_write_number16(qn: i32) -> String {
    fn pad(n: i32, width: usize) -> String {
        format!("{:0>width$}", jsnum::to_radix(n as u32, 16), width = width)
    }
    if qn == -2 {
        ".".into()
    } else if (0..16).contains(&qn) {
        jsnum::to_radix(qn as u32, 16)
    } else if (16..256).contains(&qn) {
        format!("-{}", jsnum::to_radix(qn as u32, 16))
    } else if (256..4096).contains(&qn) {
        format!("+{}", jsnum::to_radix(qn as u32, 16))
    } else if (4096..8192).contains(&qn) {
        format!("={}", pad(qn - 4096, 3))
    } else if (8192..12240).contains(&qn) {
        format!("@{}", pad(qn - 8192, 3))
    } else if (12240..77776).contains(&qn) {
        format!("*{}", pad(qn - 12240, 4))
    } else if qn >= 77776 {
        format!("${}", pad(qn - 77776, 5))
    } else {
        String::new()
    }
}

/// `genericEncodeNumber16`: cell numbers with run-length blanks.
pub fn generic_encode_number16(length: usize, mut get: impl FnMut(usize) -> i32) -> String {
    let mut cm = String::new();
    let mut count = 0u32;
    for c in 0..length {
        let pstr = generic_write_number16(get(c));
        if pstr.is_empty() {
            count += 1;
        }
        if count == 0 {
            cm.push_str(&pstr);
        } else if !pstr.is_empty() || count == 20 {
            cm.push_str(&jsnum::to_radix(15 + count, 36));
            cm.push_str(&pstr);
            count = 0;
        }
    }
    if count > 0 {
        cm.push_str(&jsnum::to_radix(15 + count, 36));
    }
    cm
}

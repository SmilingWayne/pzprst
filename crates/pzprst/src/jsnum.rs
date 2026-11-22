//! The few JavaScript number conversions pzprjs relies on.
#![allow(dead_code)] // a toolkit for genre ports; not every helper has a caller yet

/// `Number(s)` for strings (unary `+s`).
pub fn number(s: &str) -> f64 {
    let t = s.trim_matches(|c: char| c.is_whitespace() || c == '\u{feff}');
    if t.is_empty() {
        return 0.0;
    }
    for (prefix, radix) in [
        ("0x", 16),
        ("0X", 16),
        ("0o", 8),
        ("0O", 8),
        ("0b", 2),
        ("0B", 2),
    ] {
        if let Some(rest) = t.strip_prefix(prefix) {
            if rest.is_empty() || !rest.chars().all(|c| c.is_digit(radix)) {
                return f64::NAN;
            }
            return rest.chars().fold(0.0, |acc, c| {
                acc * radix as f64 + c.to_digit(radix).unwrap() as f64
            });
        }
    }
    let unsigned = t.strip_prefix(['+', '-']).unwrap_or(t);
    if unsigned == "Infinity" {
        return if t.starts_with('-') {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }
    if !is_decimal_literal(unsigned) {
        return f64::NAN;
    }
    t.parse().unwrap_or(f64::NAN)
}

fn is_decimal_literal(s: &str) -> bool {
    let b = s.as_bytes();
    let mut i = 0;
    let mut digits = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
        digits += 1;
    }
    if i < b.len() && b[i] == b'.' {
        i += 1;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return false;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let start = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == start {
            return false;
        }
    }
    i == b.len()
}

/// `isNaN(s)` for strings.
pub fn is_nan(s: &str) -> bool {
    number(s).is_nan()
}

/// `Number(s)` restricted to values that are exact `i32` integers.
pub fn integer(s: &str) -> Option<i32> {
    let n = number(s);
    (n.is_finite() && n.fract() == 0.0 && n >= i32::MIN as f64 && n <= i32::MAX as f64)
        .then_some(n as i32)
}

/// `parseInt(c, radix)` for a single character (case-insensitive like JS).
pub fn digit(c: char, radix: u32) -> Option<u32> {
    c.to_digit(radix)
}

/// `parseInt(s, radix)`: optional whitespace and sign, then the longest digit prefix.
/// `None` is JavaScript's `NaN` (no digits).
pub fn parse_int(s: &str, radix: u32) -> Option<i64> {
    let t = s.trim_start();
    let (neg, t) = match t.as_bytes().first() {
        Some(b'-') => (true, &t[1..]),
        Some(b'+') => (false, &t[1..]),
        _ => (false, t),
    };
    let t = if radix == 16 {
        t.strip_prefix("0x").or(t.strip_prefix("0X")).unwrap_or(t)
    } else {
        t
    };
    let mut n: i64 = 0;
    let mut any = false;
    for c in t.chars() {
        match c.to_digit(radix) {
            Some(v) => {
                n = n.saturating_mul(radix as i64).saturating_add(v as i64);
                any = true;
            }
            None => break,
        }
    }
    any.then_some(if neg { -n } else { n })
}

/// `s.substr(start, len)` on a body already split into chars.
pub fn substr(chars: &[char], start: usize, len: usize) -> String {
    chars.iter().skip(start).take(len).collect()
}

/// `n.toString(radix)` for non-negative integers.
pub fn to_radix(mut n: u32, radix: u32) -> String {
    if n == 0 {
        return "0".into();
    }
    let mut out = Vec::new();
    while n > 0 {
        out.push(std::char::from_digit(n % radix, radix).unwrap());
        n /= radix;
    }
    out.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_matches_js() {
        assert_eq!(number(""), 0.0);
        assert_eq!(number(" 3 "), 3.0);
        assert_eq!(number("1e1"), 10.0);
        assert_eq!(number("0x10"), 16.0);
        assert!(number("f").is_nan());
        assert!(number("3a").is_nan());
        assert!(number("inf").is_nan());
        assert!(number("nan").is_nan());
        assert_eq!(number("-1"), -1.0);
        assert_eq!(number(".5"), 0.5);
        assert!(number(".").is_nan());
        assert!(number("-").is_nan());
    }

    #[test]
    fn radix() {
        assert_eq!(to_radix(26, 27), "q");
        assert_eq!(to_radix(35, 36), "z");
        assert_eq!(to_radix(16, 16), "10");
        assert_eq!(digit('A', 27), Some(10));
        assert_eq!(digit('r', 27), None);
    }

    #[test]
    fn parse_int_matches_js() {
        assert_eq!(parse_int("1f", 16), Some(31));
        assert_eq!(parse_int("1g", 16), Some(1));
        assert_eq!(parse_int("g", 16), None);
        assert_eq!(parse_int("", 16), None);
        assert_eq!(parse_int(" -A", 16), Some(-10));
        assert_eq!(parse_int("0x1f", 16), Some(31));
        assert_eq!(parse_int("z", 36), Some(35));
        assert_eq!(substr(&['a', 'b', 'c'], 1, 5), "bc");
        assert_eq!(substr(&['a'], 3, 2), "");
    }
}

//! Tapa Loop's multi-number clues and loop answer.
use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
pub(crate) static GENRE: &dyn Genre = &Puzzle;
struct Puzzle;
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        "tapaloop"
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
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let chars: Vec<char> = body.chars().collect();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() {
            let ch = chars[i];
            match ch {
                '0'..='8' => b.cells[c].qnums = vec![ch as i32 - '0' as i32],
                '.' => b.cells[c].qnums = vec![-2],
                'a'..='f' | '+' | '-' => {
                    let (offset, len, radix, digits) = match ch {
                        '+' => (1, 2, 7, 3),
                        '-' => (1, 2, 6, 4),
                        _ => (0, 2, 8, 2),
                    };
                    let v = jsnum::parse_int(&jsnum::substr(&chars, i + offset, len), 36)
                        .unwrap_or(0) as i32;
                    let mut n = v - if offset == 0 { 360 } else { 36 };
                    let mut q = vec![0; digits];
                    for k in (0..digits).rev() {
                        q[k] = n % radix;
                        n /= radix;
                    }
                    q[0] += n * radix;
                    for x in &mut q {
                        if *x == 0 {
                            *x = -2;
                        }
                    }
                    if offset != 0 || v >= 360 {
                        b.cells[c].qnums = q;
                    }
                    i += if offset == 0 { 1 } else { 2 };
                }
                'g'..='z' => c += jsnum::digit(ch, 36).unwrap() as usize - 16,
                _ => d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("unexpected tapaloop character {ch:?}"),
                ),
            }
            c += 1;
            if c >= b.cells.len() {
                break;
            }
            i += 1;
        }
        if c < b.cells.len() {
            d.push(
                DiagnosticCode::TruncatedBody,
                "tapaloop body ends before the last cell",
            );
        }
        chars.iter().skip(i + 1).collect()
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut out = String::new();
        let mut count = 0;
        for cell in &b.cells {
            let q = &cell.qnums;
            let p = |i: usize| q[i].max(0) as u32;
            let s = match q.len() {
                1 => {
                    if q[0] == -2 {
                        ".".into()
                    } else {
                        q[0].to_string()
                    }
                }
                2 => jsnum::to_radix(360 + p(0) * 8 + p(1), 36),
                3 => format!("+{}", jsnum::to_radix(36 + p(0) * 49 + p(1) * 7 + p(2), 36)),
                4 => format!(
                    "-{}",
                    jsnum::to_radix(36 + p(0) * 216 + p(1) * 36 + p(2) * 6 + p(3), 36)
                ),
                _ => {
                    count += 1;
                    String::new()
                }
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
        r.decode_cell(b, d, |c, s| {
            if s != "." {
                c.qnums = s
                    .split(',')
                    .map(|x| {
                        if x == "-" {
                            Ok(-2)
                        } else {
                            jsnum::integer(x).ok_or_else(|| format!("expected integer: {x:?}"))
                        }
                    })
                    .collect::<std::result::Result<Vec<_>, _>>()?;
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)?;
        fileio::decode_cell_qsub(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |c| {
            let s = if c.qnums.is_empty() {
                ".".into()
            } else {
                c.qnums
                    .iter()
                    .map(|n| if *n >= 0 { n.to_string() } else { "-".into() })
                    .collect::<Vec<_>>()
                    .join(",")
            };
            format!("{s} ")
        });
        fileio::encode_border_line(w, b);
        fileio::encode_cell_qsub(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
}

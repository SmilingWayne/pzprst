//! Tapa's ordered multi-number clues and shaded-cell answers.
use super::Genre;
use crate::board::Board;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
pub(crate) static GENRE: &dyn Genre = &Tapa;
struct Tapa;
impl Genre for Tapa {
    fn pid(&self) -> &'static str {
        "tapa"
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
                '9' => b.cells[c].qnums = vec![1, 1, 1, 1],
                '.' => b.cells[c].qnums = vec![-2],
                'a'..='f' => {
                    let mut n =
                        jsnum::parse_int(&jsnum::substr(&chars, i, 2), 36).unwrap_or(0) as i32;
                    let mut values = if (360..396).contains(&n) {
                        n -= 360;
                        vec![n / 6, n % 6]
                    } else if (396..460).contains(&n) {
                        n -= 396;
                        vec![n / 16, (n % 16) / 4, n % 4]
                    } else if (460..476).contains(&n) {
                        n -= 460;
                        vec![n / 8, (n % 8) / 4, (n % 4) / 2, n % 2]
                    } else {
                        vec![]
                    };
                    for value in &mut values {
                        if *value == 0 {
                            *value = -2;
                        }
                    }
                    b.cells[c].qnums = values;
                    i += 1;
                }
                'g'..='z' => c += jsnum::digit(ch, 36).unwrap() as usize - 16,
                _ => d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("unexpected tapa character {ch:?}"),
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
                "tapa body ends before the last cell",
            );
        }
        chars.iter().skip(i + 1).collect()
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut out = String::new();
        let mut count = 0;
        for cell in &b.cells {
            let q = &cell.qnums;
            let positive = |i: usize| q[i].max(0) as u32;
            let text = match q.len() {
                1 => {
                    if q[0] == -2 {
                        ".".into()
                    } else {
                        q[0].to_string()
                    }
                }
                2 => jsnum::to_radix(360 + positive(0) * 6 + positive(1), 36),
                3 => jsnum::to_radix(396 + positive(0) * 16 + positive(1) * 4 + positive(2), 36),
                4 if q.as_slice() == [1, 1, 1, 1] => "9".into(),
                4 => jsnum::to_radix(
                    460 + u32::from(q[0] > 0) * 8
                        + u32::from(q[1] > 0) * 4
                        + u32::from(q[2] > 0) * 2
                        + u32::from(q[3] > 0),
                    36,
                ),
                _ => {
                    count += 1;
                    String::new()
                }
            };
            if count == 0 {
                out.push_str(&text);
            } else if !text.is_empty() || count == 20 {
                out.push_str(&jsnum::to_radix(count + 15, 36));
                out.push_str(&text);
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
            match s {
                "#" => c.qans = 1,
                "+" => c.qsub = 1,
                "." => {}
                _ => {
                    c.qnums = s
                        .split(',')
                        .map(|v| {
                            if v == "-" {
                                Ok(-2)
                            } else {
                                jsnum::integer(v)
                                    .ok_or_else(|| format!("expected an integer, found {v:?}"))
                            }
                        })
                        .collect::<std::result::Result<Vec<_>, _>>()?;
                }
            }
            Ok(())
        })?;
        fileio::decode_border_line(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.encode_cell(b, |c| {
            let s = if !c.qnums.is_empty() {
                c.qnums
                    .iter()
                    .map(|n| if *n >= 0 { n.to_string() } else { "-".into() })
                    .collect::<Vec<_>>()
                    .join(",")
            } else if c.qans == 1 {
                "#".into()
            } else if c.qsub == 1 {
                "+".into()
            } else {
                ".".into()
            };
            format!("{s} ")
        });
        fileio::encode_border_line_if_present(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
}

//! Kouchoku and Angle Loop cross clues and arbitrary answer segments.
use super::Genre;
use crate::board::{Board, Segment};
use crate::error::{DiagnosticCode, Diagnostics, Error, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
pub(crate) static GENRE: &dyn Genre = &Puzzle("kouchoku");
pub(super) struct Puzzle(pub(super) &'static str);
impl Genre for Puzzle {
    fn pid(&self) -> &'static str {
        self.0
    }
    fn hasborder(&self) -> u8 {
        0
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        let chars: Vec<char> = s.chars().collect();
        let (mut c, mut i) = (0usize, 0usize);
        while i < chars.len() {
            let ch = chars[i];
            match ch {
                'a'..='z' => b.crosses[c].qnum = jsnum::digit(ch, 36).unwrap() as i32 - 9,
                '0'..='9' => c += jsnum::digit(ch, 36).unwrap() as usize,
                '.' => b.crosses[c].qnum = -2,
                _ => d.push(
                    DiagnosticCode::UnexpectedChar,
                    format!("unexpected cross clue {ch:?}"),
                ),
            }
            c += 1;
            if c >= b.crosses.len() {
                break;
            }
            i += 1;
        }
        if c < b.crosses.len() {
            d.push(DiagnosticCode::TruncatedBody, "cross clue body ends early");
        }
        chars.iter().skip(i + 1).collect()
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut out = String::new();
        let mut count = 0;
        for c in &b.crosses {
            let s = if c.qnum >= 0 {
                jsnum::to_radix((9 + c.qnum) as u32, 36)
            } else if c.qnum == -2 {
                ".".into()
            } else {
                count += 1;
                String::new()
            };
            if count == 0 {
                out.push_str(&s);
            } else if !s.is_empty() || count == 10 {
                out.push_str(&jsnum::to_radix(count - 1, 36));
                out.push_str(&s);
                count = 0;
            }
        }
        if count > 0 {
            out.push_str(&jsnum::to_radix(count - 1, 36));
        }
        Ok(out)
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cross_num(r, b, d)?;
        let n = r.read_line().and_then(jsnum::integer).unwrap_or(0).max(0) as usize;
        for _ in 0..n {
            let line = r.read_line().unwrap_or("");
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|x| {
                    fileio::int(x).map_err(|e| Error::InvalidFileIo {
                        line: 0,
                        message: e,
                    })
                })
                .collect::<Result<_>>()?;
            if nums.len() < 4 {
                return Err(Error::InvalidFileIo {
                    line: 0,
                    message: "segment needs four coordinates".into(),
                });
            }
            b.segments.push(Segment {
                start: (nums[0], nums[1]),
                end: (nums[2], nums[3]),
                origin: if nums.len() >= 6 {
                    Some((nums[4], nums[5]))
                } else {
                    None
                },
            });
        }
        Ok(())
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cross_num(w, b);
        w.write_line(&b.segments.len().to_string());
        for s in &b.segments {
            let mut line = format!("{} {} {} {}", s.start.0, s.start.1, s.end.0, s.end.1);
            if let Some((x, y)) = s.origin {
                line.push_str(&format!(" {x} {y}"));
            }
            w.write_line(&line);
        }
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
}

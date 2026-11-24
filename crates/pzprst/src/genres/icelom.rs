//! Icelom's ice and number layers plus entrance and exit addresses.
use super::{g_slalom as answers, g_yajikazu as arrows, Genre};
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::jsnum;
pub(crate) static GENRE: &dyn Genre = &Icelom("icelom");
pub(super) struct Icelom(pub(super) &'static str);

pub(super) fn marker(b: &Board, is_in: bool) -> usize {
    b.borders
        .iter()
        .position(|e| if is_in { e.qnum2 == 1 } else { e.qnum3 == 1 })
        .unwrap_or(0)
}
fn point_arrow(b: &mut Board, idx: usize, is_in: bool) {
    let (x, y) = b.geom.border_pos(idx);
    let (c, r) = (b.cols() as i32 * 2, b.rows() as i32 * 2);
    let direction = if y == r {
        if is_in {
            1
        } else {
            2
        }
    } else if y == 0 {
        if is_in {
            2
        } else {
            1
        }
    } else if x == c {
        if is_in {
            3
        } else {
            4
        }
    } else if x == 0 {
        if is_in {
            4
        } else {
            3
        }
    } else {
        0
    };
    if direction != 0 {
        b.borders[idx].qdir = direction;
    }
}
fn set_address(b: &mut Board, idx: usize, is_in: bool) {
    let own = marker(b, is_in);
    let other = marker(b, !is_in);
    if idx == other {
        b.borders[own].qdir = 0;
        b.borders[other].qdir = 0;
        if is_in {
            b.borders[own].qnum2 = -1;
            b.borders[other].qnum3 = -1;
            b.borders[other].qnum2 = 1;
            b.borders[own].qnum3 = 1;
        } else {
            b.borders[own].qnum3 = -1;
            b.borders[other].qnum2 = -1;
            b.borders[other].qnum3 = 1;
            b.borders[own].qnum2 = 1;
        }
        point_arrow(b, other, is_in);
        point_arrow(b, own, !is_in);
    } else if idx != own {
        b.borders[own].qdir = 0;
        if is_in {
            b.borders[own].qnum2 = -1;
            b.borders[idx].qnum2 = 1;
        } else {
            b.borders[own].qnum3 = -1;
            b.borders[idx].qnum3 = 1;
        }
        point_arrow(b, idx, is_in);
    }
}
pub(super) fn id_for_index(b: &Board, index: usize) -> usize {
    (0..b.borders.len())
        .find(|&id| b.border_by_id(id) == Some(index))
        .unwrap_or(0)
}
pub(super) fn load_address(b: &mut Board, id: usize, is_in: bool, d: &mut Diagnostics) {
    if let Some(index) = b.border_by_id(id) {
        set_address(b, index, is_in);
    } else {
        d.push(
            DiagnosticCode::ValueOutOfRange,
            format!("invalid entrance/exit border id {id}"),
        );
    }
}
pub(super) fn decode_inout(b: &mut Board, rest: &str, d: &mut Diagnostics) {
    let parts: Vec<&str> = rest.split('/').collect();
    let offset = 2 * b.cols() * b.rows() - b.cols() - b.rows();
    let i = parts.get(1).and_then(|s| jsnum::integer(s)).unwrap_or(0) as i64 + offset as i64;
    let o = parts.get(2).and_then(|s| jsnum::integer(s)).unwrap_or(0) as i64 + offset as i64;
    for (id, is_in) in [(i, true), (o, false)] {
        if let Ok(id) = usize::try_from(id) {
            load_address(b, id, is_in, d);
        } else {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("invalid entrance/exit border id {id}"),
            );
        }
    }
}
pub(super) fn encode_inout(b: &Board) -> String {
    let offset = 2 * b.cols() * b.rows() - b.cols() - b.rows();
    let i = id_for_index(b, marker(b, true)) as i64 - offset as i64;
    let o = id_for_index(b, marker(b, false)) as i64 - offset as i64;
    format!("/{i}/{o}")
}
impl Genre for Icelom {
    fn pid(&self) -> &'static str {
        self.0
    }
    fn hasborder(&self) -> u8 {
        2
    }
    fn hascross(&self) -> u8 {
        2
    }
    fn hasexcell(&self) -> u8 {
        2
    }
    fn input_flags(&self) -> &'static str {
        "a"
    }
    fn output_flags(&self) -> &'static str {
        if self.0 == "icelom" {
            "a"
        } else {
            ""
        }
    }
    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }
    fn init_board(&self, b: &mut Board) {
        let start = b.geom.border_index(1, 0).unwrap();
        let end = if b.cols() >= 3 {
            b.geom.border_index(5, 0).unwrap()
        } else {
            b.geom.border_index(1, 2 * b.rows() as i32).unwrap()
        };
        b.borders[start].qnum2 = 1;
        b.borders[end].qnum3 = 1;
        point_arrow(b, start, true);
        point_arrow(b, end, false);
    }
    fn decode_url(&self, b: &mut Board, s: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_ice(b, s, d);
        let rest = encode::decode_number16(b, &rest, d);
        decode_inout(b, &rest, d);
        String::new()
    }
    fn encode_url(&self, b: &Board) -> Result<String> {
        Ok(format!(
            "{}{}{}",
            encode::encode_ice(b),
            encode::encode_number16(b),
            encode_inout(b)
        ))
    }
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        let i = r.read_line().and_then(jsnum::integer).unwrap_or(0).max(0) as usize;
        let o = r.read_line().and_then(jsnum::integer).unwrap_or(0).max(0) as usize;
        load_address(b, i, true, d);
        load_address(b, o, false, d);
        r.read_line();
        r.decode_cell(b, d, |c, s| {
            let s = if let Some(rest) = s.strip_prefix('i') {
                c.ques = 6;
                rest
            } else {
                s
            };
            if !s.is_empty() && s != "." {
                c.qnum = if s == "?" { -2 } else { fileio::int(s)? };
            }
            Ok(())
        })?;
        answers::decode_border_arrow_ans(r, b, d)
    }
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        w.write_line(&id_for_index(b, marker(b, true)).to_string());
        w.write_line(&id_for_index(b, marker(b, false)).to_string());
        w.write_line(if self.0 == "icelom" {
            "allwhite"
        } else {
            "skipwhite"
        });
        w.encode_cell(b, |c| {
            let prefix = if c.ques == 6 { "i" } else { "" };
            let value = match c.qnum {
                -1 if prefix.is_empty() => ".",
                -1 => "",
                -2 => "?",
                _ => return format!("{prefix}{} ", c.qnum),
            };
            format!("{prefix}{value} ")
        });
        answers::encode_border_arrow_ans(w, b);
        Ok(())
    }
    fn validate(&self, _: &Board, _: &mut Diagnostics) {}
    fn adjust_board_data_before(&self, t: crate::Transform, b: &mut Board) {
        arrows::adjust_border_qdirs(b, t);
    }
}

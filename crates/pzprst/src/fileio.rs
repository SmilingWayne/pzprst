//! pzpr FileIO text (port of `parser.js` FileData, `puzzle/FileData.js` and
//! `variety-common/FileData.js`).

use crate::board::{Board, Group};
use crate::error::{DiagnosticCode, Diagnostics, Error, Result};
use crate::genres;
use crate::jsnum;
use crate::url::MAX_DIM;
use crate::variety;

/// `parseFile`'s normalization: `[\t\r]*\n` → `\n`, then `/` → `\n`.
fn normalize(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending = String::new();
    for ch in text.chars() {
        match ch {
            '\t' | '\r' => pending.push(ch),
            '\n' => {
                pending.clear();
                out.push('\n');
            }
            _ => {
                out.push_str(&pending);
                pending.clear();
                out.push(if ch == '/' { '\n' } else { ch });
            }
        }
    }
    out.push_str(&pending);
    out
}

pub fn looks_like_fileio(text: &str) -> bool {
    let norm = normalize(text.trim_start());
    let mut lines = norm.split('\n');
    let mut first = lines.next().unwrap_or("");
    if first.starts_with("v:") {
        first = lines.next().unwrap_or("");
    }
    first.starts_with("pzprv3")
}

pub(crate) struct FileReader {
    lines: Vec<String>,
    seek: usize,
    pub filever: u32,
}

impl FileReader {
    /// `readLine`.
    pub fn read_line(&mut self) -> Option<&str> {
        self.seek += 1;
        self.lines.get(self.seek - 1).map(|s| s.as_str())
    }

    pub fn peek_line(&self) -> Option<&str> {
        self.lines.get(self.seek).map(|s| s.as_str())
    }

    fn line_no(&self) -> usize {
        self.seek + 5
    }

    /// `getItemList`: read `rows` lines, split on spaces, flatten.
    fn item_list(&mut self, rows: i32) -> Vec<String> {
        let mut items = Vec::new();
        for _ in 0..rows.max(0) {
            match self.read_line() {
                Some(line) if !line.is_empty() => {
                    items.extend(line.split(' ').filter(|s| !s.is_empty()).map(String::from))
                }
                _ => continue,
            }
        }
        items
    }

    /// `decodeObj`: tokens assigned to positions row by row; positions without an
    /// object are skipped like pzprjs's null objects.
    fn decode_obj(
        &mut self,
        b: &mut Board,
        group: Group,
        (startbx, startby, endbx, endby): (i32, i32, i32, i32),
        d: &mut Diagnostics,
        mut func: impl FnMut(&mut crate::Obj, &str) -> std::result::Result<(), String>,
    ) -> Result<()> {
        let items = self.item_list((endby - startby) / 2 + 1);
        let per_row = if endbx < startbx {
            0
        } else {
            (endbx - startbx) / 2 + 1
        };
        let expected = per_row.max(0) as usize * ((endby - startby) / 2 + 1).max(0) as usize;
        match items.len().cmp(&expected) {
            std::cmp::Ordering::Less => d.push(
                DiagnosticCode::TruncatedBody,
                "FileIO section has fewer values than objects",
            ),
            std::cmp::Ordering::Greater => d.push(
                DiagnosticCode::TrailingData,
                "FileIO section has more values than objects",
            ),
            std::cmp::Ordering::Equal => {}
        }
        let (mut bx, mut by) = (startbx, startby);
        let line = self.line_no();
        for item in &items {
            if let Some(obj) = b.obj_mut(group, bx, by) {
                func(obj, item).map_err(|message| Error::InvalidFileIo { line, message })?;
            }
            bx += 2;
            if bx > endbx {
                bx = startbx;
                by += 2;
            }
            if by > endby {
                break;
            }
        }
        Ok(())
    }

    /// `decodeCell`: `decodeObj` over cells.
    pub fn decode_cell(
        &mut self,
        b: &mut Board,
        d: &mut Diagnostics,
        func: impl FnMut(&mut crate::Obj, &str) -> std::result::Result<(), String>,
    ) -> Result<()> {
        let (c, r) = (b.cols() as i32, b.rows() as i32);
        self.decode_obj(b, Group::Cell, (1, 1, 2 * c - 1, 2 * r - 1), d, func)
    }

    /// `decodeBorder`: `decodeObj` over borders in FileIO order.
    pub fn decode_border(
        &mut self,
        b: &mut Board,
        d: &mut Diagnostics,
        mut func: impl FnMut(&mut crate::Obj, &str) -> std::result::Result<(), String>,
    ) -> Result<()> {
        for range in border_ranges(b) {
            self.decode_obj(b, Group::Border, range, d, &mut func)?;
        }
        Ok(())
    }

    /// `decodeConfigFlag`: consume the line only if it equals `flag`.
    pub fn decode_config_flag(&mut self, b: &mut Board, flag: &str, key: &'static str) {
        let on = self.peek_line() == Some(flag);
        if on {
            self.read_line();
        }
        b.rules.insert(key, on);
    }
}

/// Border ranges in FileIO order (pzpr FILE_PZPR).
fn border_ranges(b: &Board) -> Vec<(i32, i32, i32, i32)> {
    let (c, r) = (b.cols() as i32, b.rows() as i32);
    match b.geom.hasborder {
        1 => vec![(2, 1, 2 * c - 2, 2 * r - 1), (1, 2, 2 * c - 1, 2 * r - 2)],
        2 => vec![(0, 1, 2 * c, 2 * r - 1), (1, 0, 2 * c - 1, 2 * r)],
        _ => vec![],
    }
}

pub(crate) struct FileWriter {
    pub data: String,
    pub filever: u32,
}

impl FileWriter {
    /// `writeLine`.
    pub fn write_line(&mut self, s: &str) {
        self.data.push_str(s);
        self.data.push('\n');
    }

    /// `encodeObj`.
    fn encode_obj(
        &mut self,
        b: &Board,
        group: Group,
        (sbx, sby, ebx, eby): (i32, i32, i32, i32),
        func: &mut impl FnMut(&crate::Obj) -> String,
    ) {
        let empty = crate::Obj::default();
        let mut by = sby;
        while by <= eby {
            let mut line = String::new();
            let mut bx = sbx;
            while bx <= ebx {
                line.push_str(&func(b.obj(group, bx, by).unwrap_or(&empty)));
                bx += 2;
            }
            self.write_line(&line);
            by += 2;
        }
    }

    /// `encodeCell`.
    pub fn encode_cell(&mut self, b: &Board, mut func: impl FnMut(&crate::Obj) -> String) {
        let (c, r) = (b.cols() as i32, b.rows() as i32);
        self.encode_obj(b, Group::Cell, (1, 1, 2 * c - 1, 2 * r - 1), &mut func);
    }

    /// `encodeBorder`.
    pub fn encode_border(&mut self, b: &Board, mut func: impl FnMut(&crate::Obj) -> String) {
        for range in border_ranges(b) {
            self.encode_obj(b, Group::Border, range, &mut func);
        }
    }

    /// `encodeConfigFlag`.
    pub fn encode_config_flag(&mut self, b: &Board, flag: &str, key: &str) {
        if b.rules.get(key).copied().unwrap_or(false) {
            self.write_line(flag);
        }
    }
}

/// pzprjs `+ca` for an integer token; the error becomes a malformed-FileIO error.
pub(crate) fn int(token: &str) -> std::result::Result<i32, String> {
    jsnum::integer(token).ok_or_else(|| format!("expected an integer, found {token:?}"))
}

// ----- common property codecs (variety-common/FileData.js) ---------------

/// `decodeCellQnum`.
pub(crate) fn decode_cell_qnum(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "-" {
            cell.qnum = -2;
        } else if ca != "." {
            cell.qnum = int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQnum`.
pub(crate) fn encode_cell_qnum(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| match cell.qnum {
        n if n >= 0 => format!("{n} "),
        -2 => "- ".into(),
        _ => ". ".into(),
    });
}

/// `decodeCellQsub`.
pub(crate) fn decode_cell_qsub(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca != "0" {
            cell.qsub = int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQsub`.
pub(crate) fn encode_cell_qsub(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qsub > 0 {
            format!("{} ", cell.qsub)
        } else {
            "0 ".into()
        }
    });
}

/// `decodeBorderLine`.
pub(crate) fn decode_border_line(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_border(b, d, |border, ca| {
        if ca == "-1" {
            border.qsub = 2;
        } else if ca != "0" {
            border.line = int(ca)?;
        }
        Ok(())
    })
}

/// `encodeBorderLine`.
pub(crate) fn encode_border_line(w: &mut FileWriter, b: &Board) {
    w.encode_border(b, |border| {
        if border.line > 0 {
            format!("{} ", border.line)
        } else if border.qsub == 2 {
            "-1 ".into()
        } else {
            "0 ".into()
        }
    });
}

// ----- file level ----------------------------------------------------------

pub(crate) fn decode(text: &str, d: &mut Diagnostics) -> Result<Board> {
    if text.trim_start().starts_with("<?xml") {
        return Err(Error::InvalidFileIo {
            line: 1,
            message: "pencilbox XML is not supported".into(),
        });
    }
    let norm = normalize(text);
    let mut lines: std::collections::VecDeque<&str> = norm.split('\n').collect();
    let bad = |line: usize, m: &str| Error::InvalidFileIo {
        line,
        message: m.into(),
    };

    let mut first = lines.pop_front().unwrap_or("");
    let mut variant = None;
    if let Some(v) = first.strip_prefix("v:") {
        variant = Some(v.to_string());
        first = lines.pop_front().unwrap_or("");
    }
    if !first.starts_with("pzprv3") {
        return Err(bad(1, "not a pzprv3 file"));
    }
    let filever = first
        .strip_prefix("pzprv3.")
        .map(|r| {
            r.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
        })
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap_or(u32::MAX))
        .unwrap_or(0);
    let pid_name = lines.pop_front().unwrap_or("");
    let pid = variety::to_pid(pid_name).ok_or_else(|| Error::UnknownPuzzle(pid_name.into()))?;
    let genre = genres::get(pid).ok_or_else(|| Error::UnsupportedPuzzle(pid.into()))?;
    let dim = |s: Option<&str>| match s.and_then(jsnum::integer) {
        Some(n) if n >= 1 && n as usize <= MAX_DIM => Ok(n as usize),
        _ => Err(Error::InvalidSize(format!(
            "FileIO size {:?}",
            s.unwrap_or("")
        ))),
    };
    let rows = dim(lines.pop_front())?;
    let cols = dim(lines.pop_front())?;

    let mut body = Vec::new();
    for line in lines.iter() {
        if line.starts_with("http://") {
            d.push(DiagnosticCode::TrailingData, "ignored URL line after data");
            break;
        }
        if line.contains("info:{") {
            d.push(
                DiagnosticCode::DroppedMetadata,
                "metadata/history block dropped",
            );
            break;
        }
        if line.contains("history:{") || line.contains("__HISTORY__") {
            d.push(DiagnosticCode::DroppedHistory, "operation history dropped");
            break;
        }
        body.push(line.to_string());
    }

    let mut board = Board::empty(genre, cols, rows)?;
    board.variant = variant;
    let mut reader = FileReader {
        lines: body,
        seek: 0,
        filever,
    };
    genre.decode_fileio(&mut reader, &mut board, d)?;
    if reader.peek_line().is_some_and(|l| l.contains("TrialData")) {
        d.push(
            DiagnosticCode::DroppedTrial,
            "trial (tentative) boards dropped",
        );
    } else if reader.lines[reader.seek.min(reader.lines.len())..]
        .iter()
        .any(|l| !l.trim().is_empty())
    {
        d.push(
            DiagnosticCode::TrailingData,
            "ignored lines after board data",
        );
    }
    Ok(board)
}

pub(crate) fn encode(b: &Board) -> Result<String> {
    let mut w = FileWriter {
        data: String::new(),
        filever: 0,
    };
    b.genre.encode_fileio(b, &mut w)?;
    let header = if w.filever == 0 {
        "pzprv3".to_string()
    } else {
        format!("pzprv3.{}", w.filever)
    };
    Ok(format!(
        "{header}\n{}\n{}\n{}\n{}",
        b.pid(),
        b.rows(),
        b.cols(),
        w.data
    ))
}

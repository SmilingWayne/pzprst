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
    pub(crate) fn item_list(&mut self, rows: i32) -> Vec<String> {
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

    /// `decodeCross`: `decodeObj` over the cross grid, or over the inner
    /// crosses when `is_inset`.
    pub fn decode_cross(
        &mut self,
        b: &mut Board,
        d: &mut Diagnostics,
        is_inset: bool,
        func: impl FnMut(&mut crate::Obj, &str) -> std::result::Result<(), String>,
    ) -> Result<()> {
        let inset = if is_inset { 2 } else { 0 };
        let (c, r) = (b.cols() as i32, b.rows() as i32);
        self.decode_obj(
            b,
            Group::Cross,
            (inset, inset, 2 * c - inset, 2 * r - inset),
            d,
            func,
        )
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

    /// `encodeCross`: `encodeObj` over the cross grid, or over the inner
    /// crosses when `is_inset`.
    pub fn encode_cross(
        &mut self,
        b: &Board,
        is_inset: bool,
        mut func: impl FnMut(&crate::Obj) -> String,
    ) {
        let inset = if is_inset { 2 } else { 0 };
        let (c, r) = (b.cols() as i32, b.rows() as i32);
        self.encode_obj(
            b,
            Group::Cross,
            (inset, inset, 2 * c - inset, 2 * r - inset),
            &mut func,
        );
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

/// `decodeAreaRoom`: convert a grid of room identifiers to question borders.
pub(crate) fn decode_area_room(
    r: &mut FileReader,
    b: &mut Board,
    _d: &mut Diagnostics,
) -> Result<()> {
    r.read_line();
    let ids = r.item_list(b.rows() as i32);
    for y in 0..b.rows() {
        for x in 0..b.cols() {
            let i = y * b.cols() + x;
            for (nx, ny) in [(x + 1, y), (x, y + 1)] {
                if nx >= b.cols() || ny >= b.rows() {
                    continue;
                }
                let j = ny * b.cols() + nx;
                let wall = ids.get(i) != ids.get(j);
                if let Some(border) =
                    b.obj_mut(Group::Border, (x + nx + 1) as i32, (y + ny + 1) as i32)
                {
                    border.ques = i32::from(wall);
                }
            }
        }
    }
    Ok(())
}

/// `encodeAreaRoom`: emit component count and row-major room identifiers.
pub(crate) fn encode_area_room(w: &mut FileWriter, b: &Board) {
    let rooms = crate::rooms::components(b);
    let mut ids = vec![None; b.cells.len()];
    for (id, room) in rooms.iter().enumerate() {
        for &i in room {
            ids[i] = Some(id);
        }
    }
    w.write_line(&rooms.len().to_string());
    for row in ids.chunks(b.cols()) {
        let mut line = String::new();
        for id in row {
            line.push_str(&format!(
                "{} ",
                id.map_or_else(|| ".".to_string(), |v| v.to_string())
            ));
        }
        w.write_line(&line);
    }
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

/// `decodeCellQnumb`: a clue number whose `?` is `5` (the number is drawn on a
/// dark background); `.` is empty.
pub(crate) fn decode_cell_qnumb(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "5" {
            cell.qnum = -2;
        } else if ca != "." {
            cell.qnum = int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQnumb`: a clue number whose `?` is `5`; `.` is empty.
pub(crate) fn encode_cell_qnumb(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| match cell.qnum {
        n if n >= 0 => format!("{n} "),
        -2 => "5 ".into(),
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
    let mut pid = variety::to_pid(pid_name).ok_or_else(|| Error::UnknownPuzzle(pid_name.into()))?;
    let dim = |s: Option<&str>| match s.and_then(jsnum::integer) {
        Some(n) if n >= 1 && n as usize <= MAX_DIM => Ok(n as usize),
        _ => Err(Error::InvalidSize(format!(
            "FileIO size {:?}",
            s.unwrap_or("")
        ))),
    };
    let rows = dim(lines.pop_front())?;
    let cols = if pid == "sudoku" {
        rows
    } else {
        dim(lines.pop_front())?
    };
    if pid == "bonsan"
        && lines
            .iter()
            .skip(2 * rows)
            .take(2 * rows - 1)
            .any(|s| s.contains('1'))
    {
        pid = "heyabon";
    }
    if pid == "icelom" && lines.get(2).is_some_and(|line| *line == "skipwhite") {
        pid = "icelom2";
    }
    let genre = genres::get(pid).ok_or_else(|| Error::UnsupportedPuzzle(pid.into()))?;

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
    if genre.room_clues() {
        crate::rooms::normalize_clues(&mut board);
    }
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
    let size = if b.pid() == "sudoku" {
        format!("{}", b.cols())
    } else {
        format!("{}\n{}", b.rows(), b.cols())
    };
    Ok(format!("{header}\n{}\n{size}\n{}", b.pid(), w.data))
}
/// `encodeBorderLineIfPresent`: write the border line layer only when some
/// border has a line or the cross mark.
pub(crate) fn encode_border_line_if_present(w: &mut FileWriter, b: &Board) {
    if b.borders
        .iter()
        .any(|border| border.line > 0 || border.qsub == 2)
    {
        encode_border_line(w, b);
    }
}

/// `decodeCellAns`: `#` shades the cell, `+` sets its background mark.
pub(crate) fn decode_cell_ans(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "#" {
            cell.qans = 1;
        } else if ca == "+" {
            cell.qsub = 1;
        }
        Ok(())
    })
}

/// `encodeCellAns`.
pub(crate) fn encode_cell_ans(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qans == 1 {
            "# ".into()
        } else if cell.qsub == 1 {
            "+ ".into()
        } else {
            ". ".into()
        }
    });
}

/// `decodeBorderAns`: `1` marks the border, `2` the border and the cross,
/// `-1` only the cross.
pub(crate) fn decode_border_ans(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_border(b, d, |border, ca| {
        if ca == "2" {
            border.qans = 1;
            border.qsub = 1;
        } else if ca == "1" {
            border.qans = 1;
        } else if ca == "-1" {
            border.qsub = 1;
        }
        Ok(())
    })
}

/// `encodeBorderAns`.
pub(crate) fn encode_border_ans(w: &mut FileWriter, b: &Board) {
    w.encode_border(b, |border| {
        if border.qans == 1 && border.qsub == 1 {
            "2 ".into()
        } else if border.qans == 1 {
            "1 ".into()
        } else if border.qsub == 1 {
            "-1 ".into()
        } else {
            "0 ".into()
        }
    });
}

/// `decodeCellQnumAns`: cell number, `?`, shading (`#`) or sub mark (`+`).
pub(crate) fn decode_cell_qnum_ans(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "#" {
            cell.qans = 1;
        } else if ca == "+" {
            cell.qsub = 1;
        } else if ca == "-" {
            cell.qnum = -2;
        } else if ca != "." {
            cell.qnum = int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQnumAns`: number / `?` first, then shading `#`, sub mark `+`, `.`.
pub(crate) fn encode_cell_qnum_ans(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        if cell.qnum >= 0 {
            format!("{} ", cell.qnum)
        } else if cell.qnum == -2 {
            "- ".into()
        } else if cell.qans == 1 {
            "# ".into()
        } else if cell.qsub == 1 {
            "+ ".into()
        } else {
            ". ".into()
        }
    });
}

/// `decodeBorderQues`: `1` marks a wall; every other token leaves `ques` alone.
pub(crate) fn decode_border_ques(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_border(b, d, |border, ca| {
        if ca == "1" {
            border.ques = 1;
        }
        Ok(())
    })
}

/// `encodeBorderQues`.
pub(crate) fn encode_border_ques(w: &mut FileWriter, b: &Board) {
    w.encode_border(b, |border| {
        if border.ques == 1 {
            "1 ".into()
        } else {
            "0 ".into()
        }
    });
}

/// `setCellSnum`: parse bracketed pencil marks.
pub(crate) fn set_cell_snum<'a>(
    cell: &mut crate::Obj,
    ca: &'a str,
) -> std::result::Result<&'a str, String> {
    if let Some((head, rest)) = ca.split_once('[') {
        let values = rest.split(']').next().unwrap_or(rest);
        let mut nums = vec![-1; 4];
        for (i, value) in values.split(',').enumerate() {
            if i >= nums.len() {
                nums.resize(i + 1, -1);
            }
            if !value.is_empty() {
                nums[i] = int(value)?;
            }
        }
        if nums.len() != 4 || nums.iter().any(|&v| v != -1) {
            cell.snum = nums;
        }
        Ok(head)
    } else {
        Ok(ca)
    }
}

pub(crate) fn get_cell_snum(cell: &crate::Obj) -> String {
    if cell.snum.is_empty() {
        return String::new();
    }
    let text = cell
        .snum
        .iter()
        .map(|v| {
            if *v == -1 {
                String::new()
            } else {
                v.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(",");
    if text == ",,," {
        String::new()
    } else {
        format!("[{text}]")
    }
}

/// `decodeCellSnum`: optional grid of pencil marks.
pub(crate) fn decode_cell_snum(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    if r.peek_line().map_or(true, |line| line.is_empty()) {
        return Ok(());
    }
    r.decode_cell(b, d, |cell, ca| {
        set_cell_snum(cell, ca)?;
        Ok(())
    })
}

/// `encodeCellSnum`: omit the layer if every cell is empty.
pub(crate) fn encode_cell_snum(w: &mut FileWriter, b: &Board) {
    if b.cells
        .iter()
        .any(|cell| cell.snum.iter().any(|&v| v != -1))
    {
        w.encode_cell(b, |cell| {
            let s = get_cell_snum(cell);
            if s.is_empty() {
                ". ".into()
            } else {
                format!("{s} ")
            }
        });
    }
}

/// `decodeCellAnumsub`: answer number, `cmp` mark and `sub` mark per cell.
pub(crate) fn decode_cell_anumsub(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    let has_subnumbers = b.genre.subnumbers();
    r.decode_cell(b, d, |cell, ca| {
        let ca = if has_subnumbers {
            set_cell_snum(cell, ca)?
        } else {
            ca
        };
        if ca == "c" {
            cell.qcmp = 1;
        } else if ca == "+" {
            cell.qsub = 1;
        } else if ca == "-" {
            cell.qsub = 2;
        } else if ca == "=" {
            cell.qsub = 3;
        } else if ca == "%" {
            cell.qsub = 4;
        } else if ca != "." {
            cell.anum = int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellAnumsub`: answer number, `cmp` mark and `sub` mark per cell.
pub(crate) fn encode_cell_anumsub(w: &mut FileWriter, b: &Board) {
    let has_subnumbers = b.genre.subnumbers();
    w.encode_cell(b, |cell| {
        let mut ca = if cell.anum != -1 {
            cell.anum.to_string()
        } else if cell.qcmp == 1 {
            "c".into()
        } else if cell.qsub == 1 {
            "+".into()
        } else if cell.qsub == 2 {
            "-".into()
        } else if cell.qsub == 3 {
            "=".into()
        } else if cell.qsub == 4 {
            "%".into()
        } else {
            ".".into()
        };
        if has_subnumbers && cell.anum == -1 && !cell.snum.is_empty() {
            ca.push_str(&get_cell_snum(cell));
        }
        format!("{ca} ")
    });
}

/// `decodeCellQanssub`: shading (`n`) and the four background marks (`+`, `-`,
/// `=`, `%`).
pub(crate) fn decode_cell_qanssub(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cell(b, d, |cell, ca| {
        if ca == "+" {
            cell.qsub = 1;
        } else if ca == "-" {
            cell.qsub = 2;
        } else if ca == "=" {
            cell.qsub = 3;
        } else if ca == "%" {
            cell.qsub = 4;
        } else if ca != "." {
            cell.qans = int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQanssub`: shading first, then the four background marks, then `.`.
pub(crate) fn encode_cell_qanssub(w: &mut FileWriter, b: &Board) {
    w.encode_cell(b, |cell| {
        let ca = if cell.qans != 0 {
            cell.qans.to_string()
        } else if cell.qsub == 1 {
            "+".into()
        } else if cell.qsub == 2 {
            "-".into()
        } else if cell.qsub == 3 {
            "=".into()
        } else if cell.qsub == 4 {
            "%".into()
        } else {
            ".".into()
        };
        format!("{ca} ")
    });
}

// ----- dot-grid codecs (variety-common/FileData.js) ------------------------

/// `decodeDotFile`: `2*rows-1` dot-grid lines (`1`–`9`, `-` for `?`, `X` for
/// `ques` 7) concatenated like pzprjs. `set` is pzprjs `Dot.setDot`.
pub(crate) fn decode_dot_file(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
    mut set: impl FnMut(&mut crate::Obj, Group, i32),
) -> Result<()> {
    let (cols, rows) = (b.cols(), b.rows());
    let dotsmax = (2 * cols - 1) * (2 * rows - 1);
    let mut data = String::new();
    for _ in 0..2 * rows - 1 {
        match r.read_line() {
            Some(line) => data.push_str(line),
            None => d.push(
                DiagnosticCode::TruncatedBody,
                "FileIO ends before the last dot line",
            ),
        }
    }
    let chars: Vec<char> = data.chars().collect();
    if chars.len() > dotsmax {
        d.push(
            DiagnosticCode::TrailingData,
            "dot data has more characters than dots",
        );
    }
    for (s, &ca) in chars.iter().take(dotsmax).enumerate() {
        let (bx, by) = crate::encode::dot_point(b, s);
        let group = crate::encode::dot_group(bx, by);
        if let Some(num) = ca.to_digit(10) {
            if (1..=9).contains(&num) {
                if let Some(dot) = b.obj_mut(group, bx, by) {
                    set(dot, group, num as i32);
                }
            }
        } else if ca == '-' {
            if let Some(dot) = b.obj_mut(group, bx, by) {
                set(dot, group, -2);
            }
        } else if ca == 'X' {
            if let Some(dot) = b.obj_mut(group, bx, by) {
                dot.ques = 7;
            }
        }
    }
    Ok(())
}

/// `encodeDotFile`: `2*rows-1` lines of `2*cols-1` dot characters; a `?` dot
/// is `-`, a piece with `ques` 7 is `X` (unless it has a number in 1–9).
pub(crate) fn encode_dot_file(w: &mut FileWriter, b: &Board) {
    let (cols, rows) = (b.cols(), b.rows());
    for by in 1..=2 * rows - 1 {
        let mut data = String::new();
        for bx in 1..=2 * cols - 1 {
            let ca = match b.dot(bx as i32, by as i32) {
                Some(dot) if (1..=9).contains(&dot.qnum) => {
                    char::from_digit(dot.qnum as u32, 10).unwrap()
                }
                Some(dot) if dot.ques == 7 => 'X',
                Some(dot) if dot.qnum == -2 => '-',
                _ => '.',
            };
            data.push(ca);
        }
        w.write_line(&data);
    }
}

/// `decodeCrossNum`.
pub(crate) fn decode_cross_num(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    r.decode_cross(b, d, false, |cross, ca| {
        if ca == "-" {
            cross.qnum = -2;
        } else if ca != "." {
            cross.qnum = int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCrossNum`.
pub(crate) fn encode_cross_num(w: &mut FileWriter, b: &Board) {
    w.encode_cross(b, false, |cross| match cross.qnum {
        n if n >= 0 => format!("{n} "),
        -2 => "- ".into(),
        _ => ". ".into(),
    });
}

// ----- cell/excell ("obj") grid codecs (puzzle/FileData.js) ----------------

/// pzprjs `Board.setminmax`: the `obj` rectangle `decodeCellExCell` and
/// `encodeCellExCell` walk, `(minbx + 1, minby + 1)` to
/// `(maxbx - 1, maxby - 1)`.
fn obj_rect(b: &Board) -> (i32, i32, i32, i32) {
    let (c, r) = (b.cols() as i32, b.rows() as i32);
    let exrows = b.geom.excell_rows as i32;
    let excols = b.geom.excell_cols as i32;
    let (minbx, minby) = if b.geom.hasexcell > 0 {
        (-2 * excols, -2 * exrows)
    } else {
        (0, 0)
    };
    let (maxbx, maxby) = if b.geom.hasexcell == 2 {
        (2 * c + 2, 2 * r + 2)
    } else {
        (2 * c, 2 * r)
    };
    (minbx + 1, minby + 1, maxbx - 1, maxby - 1)
}

/// pzprjs `getObjectPos("obj", bx, by)`: a cell inside the grid, an excell
/// outside it. Positions with no object at all (the `hasexcell` 1 top-left
/// corner) still get the excell group, like pzprjs's frozen `emptyexcell`.
fn obj_group(b: &Board, bx: i32, by: i32) -> Group {
    if b.geom.cell_index(bx, by).is_some() {
        Group::Cell
    } else {
        Group::ExCell
    }
}

/// `decodeCellExCell`: one token per cell or excell of pzprjs's combined `obj`
/// grid, in FileIO order. `func` receives the object, its group and the token.
pub(crate) fn decode_cell_excell(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
    mut func: impl FnMut(&mut crate::Obj, Group, &str) -> std::result::Result<(), String>,
) -> Result<()> {
    let (sbx, sby, ebx, eby) = obj_rect(b);
    let items = r.item_list((eby - sby) / 2 + 1);
    let per_row = if ebx < sbx { 0 } else { (ebx - sbx) / 2 + 1 };
    let expected = per_row.max(0) as usize * ((eby - sby) / 2 + 1).max(0) as usize;
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
    let (mut bx, mut by) = (sbx, sby);
    let line = r.line_no();
    let mut null = crate::Obj::default();
    for item in &items {
        let group = obj_group(b, bx, by);
        let result = match b.obj_at_mut(bx, by) {
            Some(obj) => func(obj, group, item),
            None => func(&mut null, group, item),
        };
        result.map_err(|message| Error::InvalidFileIo { line, message })?;
        bx += 2;
        if bx > ebx {
            bx = sbx;
            by += 2;
        }
        if by > eby {
            break;
        }
    }
    Ok(())
}

/// `encodeCellExCell`: one token per cell or excell of pzprjs's combined `obj`
/// grid, one FileIO line per object row. `func` receives the object and its
/// group.
pub(crate) fn encode_cell_excell(
    w: &mut FileWriter,
    b: &Board,
    mut func: impl FnMut(&crate::Obj, Group) -> String,
) {
    let (sbx, sby, ebx, eby) = obj_rect(b);
    let empty = crate::Obj::default();
    let mut by = sby;
    while by <= eby {
        let mut line = String::new();
        let mut bx = sbx;
        while bx <= ebx {
            let group = obj_group(b, bx, by);
            let obj = b.obj_at(bx, by).unwrap_or(&empty);
            line.push_str(&func(obj, group));
            bx += 2;
        }
        w.write_line(&line);
        by += 2;
    }
}

// ----- kakuro `[＼]` cell/excell layer (variety-common/FileData.js) ---------

/// `decodeCellQnum51`: one token per cell or excell of the combined `obj`
/// grid. A cell token is `qnum,qnum2` (and makes the cell a `[＼]`); the top
/// excell stores `qnum2`, the left one `qnum`, and the top-left corner is
/// ignored. `.` leaves the object alone.
pub(crate) fn decode_cell_qnum51(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    let (sbx, sby, ebx, eby) = obj_rect(b);
    // pzprjs's frozen `emptyexcell` receives tokens for positions with no
    // object (the `hasexcell` 2 ring corners); they are ignored.
    let mut real = Vec::new();
    let mut by = sby;
    while by <= eby {
        let mut bx = sbx;
        while bx <= ebx {
            real.push(b.obj_at(bx, by).is_some());
            bx += 2;
        }
        by += 2;
    }
    let mut pos = (sbx, sby);
    let mut idx = 0usize;
    decode_cell_excell(r, b, d, |obj, group, ca| {
        let (bx, by) = pos;
        pos = if bx + 2 > ebx {
            (sbx, by + 2)
        } else {
            (bx + 2, by)
        };
        let is_real = real.get(idx).copied().unwrap_or(false);
        idx += 1;
        if ca == "." || !is_real {
            return Ok(());
        }
        if group == Group::Cell {
            let mut parts = ca.split(',');
            let qnum = int(parts.next().unwrap_or(""))?;
            let qnum2 = match parts.next() {
                Some(token) => int(token)?,
                None => return Err(format!("expected \"qnum,qnum2\", found {ca:?}")),
            };
            // pzprjs `Cell.set51cell` clears the four direction numbers.
            obj.ques = 51;
            obj.qnum = qnum;
            obj.qnum2 = qnum2;
            obj.qnum3 = -1;
            obj.qnum4 = -1;
            obj.anum = -1;
        } else if bx != -1 && by == -1 {
            obj.qnum2 = int(ca)?;
        } else if bx == -1 && by != -1 {
            obj.qnum = int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellQnum51`: one token per cell or excell of the combined `obj`
/// grid. A `[＼]` cell writes `qnum,qnum2`, any other cell `.`, the top-left
/// corner `0`, the top excell its `qnum2` and the left one its `qnum`.
pub(crate) fn encode_cell_qnum51(w: &mut FileWriter, b: &Board) {
    let (sbx, sby, ebx, _eby) = obj_rect(b);
    let mut pos = (sbx, sby);
    encode_cell_excell(w, b, |obj, group| {
        let (bx, by) = pos;
        pos = if bx + 2 > ebx {
            (sbx, by + 2)
        } else {
            (bx + 2, by)
        };
        if group == Group::Cell {
            if obj.ques == 51 {
                format!("{},{} ", obj.qnum, obj.qnum2)
            } else {
                ". ".into()
            }
        } else if b.obj_at(bx, by).is_none() {
            // The frozen `emptyexcell` has null coordinates, so pzprjs's
            // `by === -1` test fails and its default `qnum` (-1) is written.
            "-1 ".into()
        } else if bx == -1 && by == -1 {
            "0 ".into()
        } else if by == -1 {
            format!("{} ", obj.qnum2)
        } else {
            format!("{} ", obj.qnum)
        }
    });
}

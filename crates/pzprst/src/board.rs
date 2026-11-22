//! Static pzpr board: objects addressed by pzprjs doubled coordinates `(bx, by)`.

use std::collections::BTreeMap;
use std::fmt;

use crate::error::{DiagnosticCode, Diagnostics, Error, Result};
use crate::genres::{self, Genre};
use crate::transform::Transform;
use crate::{fileio, state, url, variety};

/// Properties shared by every pzpr board object, with pzprjs defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Obj {
    pub qnum: i32,
    pub qnum2: i32,
    pub qnum3: i32,
    pub qnum4: i32,
    pub ques: i32,
    pub qdir: i32,
    pub anum: i32,
    pub qans: i32,
    pub qsub: i32,
    pub qcmp: i32,
    pub line: i32,
}

impl Default for Obj {
    fn default() -> Self {
        Self {
            qnum: -1,
            qnum2: -1,
            qnum3: -1,
            qnum4: -1,
            ques: 0,
            qdir: 0,
            anum: -1,
            qans: 0,
            qsub: 0,
            qcmp: 0,
            line: 0,
        }
    }
}

impl Obj {
    /// Puzzle-definition properties (what URLs encode and what pzpr-trace
    /// state matching compares): every `qnum*` slot, `ques` and `qdir`.
    pub fn question(&self) -> (i32, i32, i32, i32, i32, i32) {
        (
            self.qnum, self.qnum2, self.qnum3, self.qnum4, self.ques, self.qdir,
        )
    }

    pub fn clear_answer(&mut self) {
        let d = Obj::default();
        self.anum = d.anum;
        self.qans = d.qans;
        self.qsub = d.qsub;
        self.qcmp = d.qcmp;
        self.line = d.line;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Cell,
    Border,
    Cross,
    ExCell,
}

/// Board shape: `hasborder` is pzprjs's flag (0 none, 1 inner borders, 2 inner and outer).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Geometry {
    pub cols: usize,
    pub rows: usize,
    pub hasborder: u8,
    pub hascross: u8,
    /// pzprjs `Board.hasexcell` (0 none, 1 above/left, 2 all four sides).
    pub hasexcell: u8,
    /// pzprjs `Board.excellRows` / `excellCols` (default 1 each).
    pub excell_rows: usize,
    pub excell_cols: usize,
    /// pzprjs `hasexcell === 1`'s extra top-left excell (`emptyexcell.ques` 51).
    pub excell_corner: bool,
}

impl Geometry {
    fn c(&self) -> i32 {
        self.cols as i32
    }
    fn r(&self) -> i32 {
        self.rows as i32
    }

    fn cell_count(&self) -> usize {
        self.cols * self.rows
    }

    /// pzprjs `estimateSize("cross")`: a `(cols + 1) x (rows + 1)` grid at the
    /// even/even points; pzprst only allocates it when a genre uses crosses.
    pub fn cross_count(&self) -> usize {
        if self.hascross == 0 {
            0
        } else {
            (self.cols + 1) * (self.rows + 1)
        }
    }

    pub fn cross_index(&self, bx: i32, by: i32) -> Option<usize> {
        if bx & 1 != 0
            || by & 1 != 0
            || bx < 0
            || by < 0
            || bx > 2 * self.c()
            || by > 2 * self.r()
            || self.hascross == 0
        {
            return None;
        }
        Some((by >> 1) as usize * (self.cols + 1) + (bx >> 1) as usize)
    }

    /// pzprjs `estimateSize("excell")`.
    pub fn excell_count(&self) -> usize {
        match self.hasexcell {
            0 => 0,
            1 => {
                self.cols * self.excell_rows
                    + self.rows * self.excell_cols
                    + usize::from(self.excell_corner)
            }
            _ => 2 * (self.cols * self.excell_rows + self.rows * self.excell_cols),
        }
    }

    /// pzprjs `setposExCells`: the object's `(bx, by)` for excell id `i`.
    pub fn excell_pos(&self, i: usize) -> Option<(i32, i32)> {
        if i >= self.excell_count() {
            return None;
        }
        let exrows = self.excell_rows as i32;
        let excols = self.excell_cols as i32;
        let qc = (self.cols as i32) * exrows;
        let qr = (self.rows as i32) * excols;
        let i = i as i32;
        Some(match self.hasexcell {
            1 => {
                if i < qc {
                    ((i / exrows) * 2 + 1, (i % exrows) * -2 - 1)
                } else if i < qc + qr {
                    let j = i - qc;
                    ((j % excols) * -2 - 1, (j / excols) * 2 + 1)
                } else {
                    (-1, -1)
                }
            }
            _ => {
                if i < qc {
                    (i * 2 + 1, -1)
                } else if i < 2 * qc {
                    ((i - qc) * 2 + 1, 2 * qr + 1)
                } else if i < 2 * qc + qr {
                    (-1, (i - 2 * qc) * 2 + 1)
                } else {
                    (2 * qc + 1, (i - 2 * qc - qr) * 2 + 1)
                }
            }
        })
    }

    /// The operation sequence of a composed transform, in the order
    /// `tools/gen_fixtures.js` composes it (pzprjs applies one board operation
    /// at a time, so hooks and excell movement happen per operation).
    pub fn op_sequence(t: Transform) -> &'static [Transform] {
        match t {
            Transform::Identity => &[],
            Transform::Rot90 => &[Transform::Rot90],
            Transform::Rot180 => &[Transform::Rot90, Transform::Rot90],
            Transform::Rot270 => &[Transform::Rot270],
            Transform::FlipX => &[Transform::FlipX],
            Transform::FlipY => &[Transform::FlipY],
            Transform::Transpose => &[Transform::Rot270, Transform::FlipY],
            Transform::AntiTranspose => &[Transform::Rot90, Transform::FlipY],
        }
    }

    pub fn excell_index(&self, bx: i32, by: i32) -> Option<usize> {
        (0..self.excell_count()).find(|&i| self.excell_pos(i) == Some((bx, by)))
    }

    pub fn cross_pos(&self, i: usize) -> Option<(i32, i32)> {
        if i >= self.cross_count() {
            return None;
        }
        let (bx, by) = ((i % (self.cols + 1)) as i32, (i / (self.cols + 1)) as i32);
        Some((2 * bx, 2 * by))
    }

    pub fn cell_index(&self, bx: i32, by: i32) -> Option<usize> {
        if bx & 1 == 0
            || by & 1 == 0
            || bx < 1
            || by < 1
            || bx > 2 * self.c() - 1
            || by > 2 * self.r() - 1
        {
            return None;
        }
        Some((by >> 1) as usize * self.cols + (bx >> 1) as usize)
    }

    pub fn cell_pos(&self, i: usize) -> (i32, i32) {
        (
            (i % self.cols) as i32 * 2 + 1,
            (i / self.cols) as i32 * 2 + 1,
        )
    }

    /// Range of `bx` for vertical borders (`bx` even).
    fn vertical_bx(&self) -> (i32, i32) {
        if self.hasborder == 2 {
            (0, 2 * self.c())
        } else {
            (2, 2 * self.c() - 2)
        }
    }

    /// Range of `by` for horizontal borders (`by` even).
    fn horizontal_by(&self) -> (i32, i32) {
        if self.hasborder == 2 {
            (0, 2 * self.r())
        } else {
            (2, 2 * self.r() - 2)
        }
    }

    /// Vertical borders per row.
    fn vertical_per_row(&self) -> usize {
        if self.hasborder == 0 {
            return 0;
        }
        let (s, e) = self.vertical_bx();
        if e < s {
            0
        } else {
            ((e - s) / 2 + 1) as usize
        }
    }

    /// Rows of horizontal borders.
    fn horizontal_rows(&self) -> usize {
        if self.hasborder == 0 {
            return 0;
        }
        let (s, e) = self.horizontal_by();
        if e < s {
            0
        } else {
            ((e - s) / 2 + 1) as usize
        }
    }

    fn vertical_count(&self) -> usize {
        self.vertical_per_row() * self.rows
    }

    fn border_count(&self) -> usize {
        self.vertical_count() + self.horizontal_rows() * self.cols
    }

    /// Borders are ordered: vertical row-major, then horizontal row-major (FileIO order).
    pub fn border_index(&self, bx: i32, by: i32) -> Option<usize> {
        if self.hasborder == 0 {
            return None;
        }
        if bx & 1 == 0 && by & 1 == 1 {
            let (s, e) = self.vertical_bx();
            if bx < s || bx > e || by < 1 || by > 2 * self.r() - 1 {
                return None;
            }
            Some((by >> 1) as usize * self.vertical_per_row() + ((bx - s) / 2) as usize)
        } else if bx & 1 == 1 && by & 1 == 0 {
            let (s, e) = self.horizontal_by();
            if by < s || by > e || bx < 1 || bx > 2 * self.c() - 1 {
                return None;
            }
            Some(self.vertical_count() + ((by - s) / 2) as usize * self.cols + (bx >> 1) as usize)
        } else {
            None
        }
    }

    pub fn border_pos(&self, i: usize) -> (i32, i32) {
        let nv = self.vertical_count();
        if i < nv {
            let per = self.vertical_per_row();
            let (s, _) = self.vertical_bx();
            (s + 2 * (i % per) as i32, 2 * (i / per) as i32 + 1)
        } else {
            let j = i - nv;
            let (s, _) = self.horizontal_by();
            (
                2 * (j % self.cols) as i32 + 1,
                s + 2 * (j / self.cols) as i32,
            )
        }
    }

    #[allow(dead_code)] // for genre ports that index pzprjs border ids
    /// Position of pzprjs `board.border[id]` (`setposBorders`): inner vertical, inner
    /// horizontal, then for hasborder 2 the top, bottom, left and right frame.
    pub fn pzprjs_border_pos(&self, id: usize) -> Option<(i32, i32)> {
        let (c, r) = (self.cols, self.rows);
        let mut i = id;
        let inner_v = (c - 1) * r;
        if i < inner_v {
            return Some(((i % (c - 1)) as i32 * 2 + 2, (i / (c - 1)) as i32 * 2 + 1));
        }
        i -= inner_v;
        if i < c * (r - 1) {
            return Some(((i % c) as i32 * 2 + 1, (i / c) as i32 * 2 + 2));
        }
        i -= c * (r - 1);
        if self.hasborder != 2 {
            return None;
        }
        let (c2, r2) = (2 * self.c(), 2 * self.r());
        for (n, pos) in [
            (
                c,
                &(|k: usize| (k as i32 * 2 + 1, 0)) as &dyn Fn(usize) -> (i32, i32),
            ),
            (c, &|k: usize| (k as i32 * 2 + 1, r2)),
            (r, &|k: usize| (0, k as i32 * 2 + 1)),
            (r, &|k: usize| (c2, k as i32 * 2 + 1)),
        ] {
            if i < n {
                return Some(pos(i));
            }
            i -= n;
        }
        None
    }
}

/// Parsing options.
#[derive(Debug, Clone)]
pub struct Options {
    /// Turn any diagnostic into [`Error::Strict`].
    pub strict: bool,
    /// For trace URLs, read only `state` when a `trace` (history) parameter is present.
    pub ignore_trace: bool,
    /// Maximum inflated state size in bytes (pzpr-trace's limit by default).
    pub max_state_bytes: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            strict: false,
            ignore_trace: false,
            max_state_bytes: 1 << 20,
        }
    }
}

/// A static pzpr board: puzzle definition plus an optional answer snapshot.
#[derive(Clone)]
pub struct Board {
    pub(crate) genre: &'static dyn Genre,
    pub(crate) geom: Geometry,
    pub(crate) rules: BTreeMap<&'static str, bool>,
    pub(crate) variant: Option<String>,
    pub(crate) cells: Vec<Obj>,
    pub(crate) borders: Vec<Obj>,
    pub(crate) crosses: Vec<Obj>,
    pub(crate) excells: Vec<Obj>,
    pub(crate) diagnostics: Diagnostics,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Board")
            .field("pid", &self.pid())
            .field("cols", &self.geom.cols)
            .field("rows", &self.geom.rows)
            .field("rules", &self.rules)
            .field("variant", &self.variant)
            .finish_non_exhaustive()
    }
}

impl PartialEq for Board {
    /// Equality of board content; diagnostics are ignored.
    fn eq(&self, other: &Self) -> bool {
        self.pid() == other.pid()
            && self.geom == other.geom
            && self.rules == other.rules
            && self.variant == other.variant
            && self.cells == other.cells
            && self.borders == other.borders
            && self.crosses == other.crosses
            && self.excells == other.excells
    }
}

impl Board {
    /// An empty board of a supported puzzle type (pid or alias).
    pub fn new(puzzle: &str, cols: usize, rows: usize) -> Result<Self> {
        let pid = variety::to_pid(puzzle).ok_or_else(|| Error::UnknownPuzzle(puzzle.into()))?;
        let genre = genres::get(pid).ok_or_else(|| Error::UnsupportedPuzzle(pid.into()))?;
        Self::empty(genre, cols, rows)
    }

    pub(crate) fn empty(genre: &'static dyn Genre, cols: usize, rows: usize) -> Result<Self> {
        if cols == 0 || rows == 0 || cols > url::MAX_DIM || rows > url::MAX_DIM {
            return Err(Error::InvalidSize(format!("{cols}x{rows}")));
        }
        let geom = Geometry {
            cols,
            rows,
            hasborder: genre.hasborder(),
            hascross: genre.hascross(),
            hasexcell: genre.hasexcell(),
            excell_rows: genre.excell_rows(cols, rows),
            excell_cols: genre.excell_cols(cols, rows),
            excell_corner: genre.excell_corner(),
        };
        Ok(Self {
            genre,
            geom,
            rules: genre
                .rule_flags()
                .iter()
                .map(|(_, key)| (*key, false))
                .collect(),
            variant: None,
            cells: vec![Obj::default(); geom.cell_count()],
            borders: vec![Obj::default(); geom.border_count()],
            crosses: vec![Obj::default(); geom.cross_count()],
            excells: vec![Obj::default(); geom.excell_count()],
            diagnostics: Diagnostics::default(),
        })
    }

    // ----- parsing -------------------------------------------------------

    /// Read a puzzle URL (any host or a bare body). Answer data is never read from it.
    pub fn from_url(input: &str, opts: &Options) -> Result<Self> {
        let mut diags = Diagnostics::default();
        let parts = url::parse(input, &mut diags)?;
        if parts.param("state").is_some_and(|s| !s.is_empty()) {
            diags.push(
                DiagnosticCode::IgnoredState,
                "state parameter ignored; use from_trace_url",
            );
        }
        let mut board = Self::from_url_parts(&parts, &mut diags)?;
        board.diagnostics = diags.check_strict(opts.strict)?;
        Ok(board)
    }

    fn from_url_parts(parts: &url::UrlParts, diags: &mut Diagnostics) -> Result<Self> {
        let genre =
            genres::get(parts.pid).ok_or_else(|| Error::UnsupportedPuzzle(parts.pid.into()))?;
        let mut board = Self::empty(genre, parts.cols, parts.rows)?;
        board.variant = parts.variant.clone();
        if parts.body.is_empty() {
            if !parts.pflag.is_empty() {
                diags.push(
                    DiagnosticCode::UnknownFlag,
                    "flags are ignored without a body, as in pzprjs",
                );
            }
            return Ok(board);
        }
        for c in parts.pflag.chars() {
            if !genre.rule_flags().iter().any(|(f, _)| *f == c) {
                diags.push(
                    DiagnosticCode::UnknownFlag,
                    format!("unknown URL flag {c:?}"),
                );
            }
        }
        for (flag, key) in genre.rule_flags() {
            board.rules.insert(key, parts.pflag.contains(*flag));
        }
        let rest = genre.decode_url(&mut board, &parts.body, diags);
        if !rest.is_empty() {
            diags.push(
                DiagnosticCode::TrailingData,
                format!("ignored trailing URL data {rest:?}"),
            );
        }
        Ok(board)
    }

    /// Read pzpr FileIO text (newline- or `/`-separated).
    pub fn from_fileio(text: &str, opts: &Options) -> Result<Self> {
        let mut diags = Diagnostics::default();
        let mut board = fileio::decode(text, &mut diags)?;
        board.genre.validate(&board, &mut diags);
        board.diagnostics = diags.check_strict(opts.strict)?;
        Ok(board)
    }

    /// Read a pzpr-trace `state` value.
    pub fn from_state(s: &str, opts: &Options) -> Result<Self> {
        let text = state::decode(s, opts.max_state_bytes)?;
        Self::from_fileio(&text, opts)
    }

    /// Read a pzpr-trace URL `…?state=S&pid/cols/rows/body`; the state must match the puzzle.
    pub fn from_trace_url(input: &str, opts: &Options) -> Result<Self> {
        let mut diags = Diagnostics::default();
        let parts = url::parse(input, &mut diags)?;
        if parts.param("trace").is_some_and(|t| !t.is_empty()) {
            if !opts.ignore_trace {
                return Err(Error::TraceNotSupported);
            }
            diags.push(DiagnosticCode::IgnoredTrace, "trace parameter ignored");
        }
        let puzzle = Self::from_url_parts(&parts, &mut diags)?;
        let Some(s) = parts.param("state").filter(|s| !s.is_empty()) else {
            return Err(Error::StateDecode("URL has no state parameter".into()));
        };
        let text = state::decode(s, opts.max_state_bytes)?;
        let mut board = fileio::decode(&text, &mut diags)?;
        board.genre.validate(&board, &mut diags);
        state::check_match(&puzzle, &board)?;
        board.variant = puzzle.variant.clone();
        board.diagnostics = diags.check_strict(opts.strict)?;
        Ok(board)
    }

    /// Read any supported input: FileIO text, a trace URL (non-empty `state=`),
    /// a bare state (no `/`), or a puzzle URL.
    pub fn parse(input: &str, opts: &Options) -> Result<Self> {
        let input = input.trim();
        if fileio::looks_like_fileio(input) {
            return Self::from_fileio(input, opts);
        }
        if !input.contains('/') {
            return Self::from_state(input, opts);
        }
        let parts = url::parse(input, &mut Diagnostics::default())?;
        if parts.param("state").is_some_and(|s| !s.is_empty()) {
            Self::from_trace_url(input, opts)
        } else {
            Self::from_url(input, opts)
        }
    }

    // ----- output --------------------------------------------------------

    /// pzprjs `getURL(URL_PZPRV3)` body: `pid/[v:…/][flags/]cols/rows/data`.
    /// This is the canonical URL; any answer snapshot is not part of it.
    pub fn to_url(&self) -> Result<String> {
        let body = self.genre.encode_url(self)?;
        let pflag: String = self
            .genre
            .rule_flags()
            .iter()
            .filter(|(_, key)| self.rules.get(key).copied().unwrap_or(false))
            .map(|(f, _)| *f)
            .collect();
        let urlid = variety::urlid(self.pid()).unwrap_or(self.pid());
        Ok(url::generate(
            urlid,
            self.variant.as_deref(),
            &pflag,
            self.geom.cols,
            self.geom.rows,
            &body,
        ))
    }

    /// pzprjs `getFileData()` text. Like pzprjs, it does not store a `v:` variant.
    pub fn to_fileio(&self) -> Result<String> {
        fileio::encode(self)
    }

    /// pzpr-trace `state`: raw-DEFLATE FileIO, unpadded URL-safe base64.
    pub fn to_state(&self) -> Result<String> {
        Ok(state::encode(&self.to_fileio()?))
    }

    /// `https://pzpr-trace.net/p?state=…&<url>`.
    pub fn to_trace_url(&self, prefix: &str) -> Result<String> {
        Ok(format!(
            "{prefix}state={}&{}",
            self.to_state()?,
            self.to_url()?
        ))
    }

    // ----- symmetries ----------------------------------------------------

    /// pzprjs `turnflipGroup("excell", key, d)` over the operations of `t`.
    ///
    /// `hasexcell == 1` moves no excell on turns and mirrors the whole top
    /// block on `flipx` / the whole left block on `flipy` (the `d2` region in
    /// pzprjs `turnflipGroup`); `hasexcell == 2` maps the ring around the board.
    /// `excellRows`/`excellCols` may depend on the board size, so they are
    /// re-evaluated at each operation, like pzprjs `setminmax`/`setposAll`.
    fn transform_excells(&self, t: Transform, src: &[Obj]) -> Vec<Obj> {
        if self.geom.hasexcell == 0 {
            return src.to_vec();
        }
        let (mut cols, mut rows) = (self.geom.cols, self.geom.rows);
        let mut cur: Vec<Obj> = src.to_vec();
        for &op in Geometry::op_sequence(t) {
            let (nc, nr) = if op.swaps_axes() {
                (rows, cols)
            } else {
                (cols, rows)
            };
            // pzprjs `turnflipGroup` moves no excell at all on a turn when
            // `hasexcell == 1`: the object ids keep their values and only their
            // positions (and the count per side) change with the new dimensions.
            if self.geom.hasexcell == 1 && op.swaps_axes() {
                cols = nc;
                rows = nr;
                continue;
            }
            let from_geom = Geometry {
                cols,
                rows,
                excell_rows: self.genre.excell_rows(cols, rows),
                excell_cols: self.genre.excell_cols(cols, rows),
                ..self.geom
            };
            let to_geom = Geometry {
                cols: nc,
                rows: nr,
                excell_rows: self.genre.excell_rows(nc, nr),
                excell_cols: self.genre.excell_cols(nc, nr),
                ..self.geom
            };
            let mut next = vec![Obj::default(); to_geom.excell_count()];
            for (i, obj) in cur.iter().enumerate() {
                let Some((bx, by)) = from_geom.excell_pos(i) else {
                    continue;
                };
                let moved = if self.geom.hasexcell == 1 {
                    match op {
                        Transform::FlipX if by < 0 && bx >= 1 => Some(((2 * nc as i32) - bx, by)),
                        Transform::FlipY if bx < 0 && by >= 1 => Some((bx, (2 * nr as i32) - by)),
                        Transform::FlipX | Transform::FlipY => Some((bx, by)),
                        _ => Some((bx, by)),
                    }
                } else {
                    Some(match op {
                        Transform::FlipX => ((2 * nc as i32) - bx, by),
                        Transform::FlipY => (bx, (2 * nr as i32) - by),
                        Transform::Rot90 => ((2 * nc as i32) - by, bx),
                        Transform::Rot270 => (by, (2 * nr as i32) - bx),
                        _ => (bx, by),
                    })
                };
                if let Some((x, y)) = moved {
                    if let Some(j) = to_geom.excell_index(x, y) {
                        if let Some(slot) = next.get_mut(j) {
                            *slot = *obj;
                        }
                    }
                }
            }
            cur = next;
            cols = nc;
            rows = nr;
        }
        cur
    }

    /// pzprjs `turnflipGroup("excell", …)` for a genre-local copy of the excell
    /// values (a genre hook that has to replay the movement itself).
    pub fn move_excells(&self, t: Transform, src: &[Obj]) -> Vec<Obj> {
        self.transform_excells(t, src)
    }

    /// Apply a board symmetry; puzzle and snapshot move together.
    pub fn transform(&self, t: Transform) -> Board {
        let (c, r) = (self.geom.cols, self.geom.rows);
        let (nc, nr) = if t.swaps_axes() { (r, c) } else { (c, r) };
        let mut out = self.clone();
        self.genre.adjust_board_data_before(t, &mut out);
        // The hook edited `out`; the coordinate map has to move *those* objects,
        // not the untouched originals (pzprjs adjusts the data, then moves it).
        let src_cells = out.cells.clone();
        let src_borders = out.borders.clone();
        let src_crosses = out.crosses.clone();
        out.geom = Geometry {
            cols: nc,
            rows: nr,
            ..self.geom
        };
        let map = |bx: i32, by: i32| t.map_point(bx, by, c as i32, r as i32, nc as i32, nr as i32);
        for (i, obj) in src_cells.iter().enumerate() {
            let (bx, by) = self.geom.cell_pos(i);
            let (x, y) = map(bx, by);
            out.cells[out.geom.cell_index(x, y).expect("cell maps to cell")] = *obj;
        }
        for (i, obj) in src_borders.iter().enumerate() {
            let (bx, by) = self.geom.border_pos(i);
            let (x, y) = map(bx, by);
            out.borders[out.geom.border_index(x, y).expect("border maps to border")] = *obj;
        }
        for (i, obj) in src_crosses.iter().enumerate() {
            let (bx, by) = self.geom.cross_pos(i).expect("cross maps to cross");
            let (x, y) = map(bx, by);
            out.crosses[out.geom.cross_index(x, y).expect("cross maps to cross")] = *obj;
        }
        // pzprjs `BoardExec.turnflipGroup("excell", …)` does not move excells by
        // the generic coordinate map: `hasexcell == 1` leaves them in place for
        // turns and only reverses the top row / left column for flips, while
        // `hasexcell == 2` maps them on the ring around the board. The
        // per-genre `adjustExCellTopLeft` hooks then redistribute the values.
        out.excells = self.transform_excells(t, &out.excells);
        out.geom.excell_rows = out.genre.excell_rows(nc, nr);
        out.geom.excell_cols = out.genre.excell_cols(nc, nr);
        self.genre.adjust_board_data(t, &mut out);
        out
    }

    /// Symmetry-aware duplicate key over the genre's allowed transforms.
    pub fn dedupe_key(&self) -> Result<DedupeKey> {
        let mut best: Option<(String, Transform)> = None;
        for &t in self.genre.symmetries() {
            let u = self.transform(t).to_url()?;
            if best
                .as_ref()
                .map_or(true, |(b, _)| u.as_bytes() < b.as_bytes())
            {
                best = Some((u, t));
            }
        }
        let (representative, transform) = best.expect("identity is always allowed");
        Ok(DedupeKey {
            key: format!("{DEDUPE_POLICY}:{representative}"),
            representative,
            transform,
        })
    }

    // ----- accessors -----------------------------------------------------

    pub fn pid(&self) -> &'static str {
        self.genre.pid()
    }
    pub fn cols(&self) -> usize {
        self.geom.cols
    }
    pub fn rows(&self) -> usize {
        self.geom.rows
    }
    pub fn variant(&self) -> Option<&str> {
        self.variant.as_deref()
    }
    /// Rule settings keyed by pzprjs config name (e.g. `slither_full`).
    pub fn rules(&self) -> &BTreeMap<&'static str, bool> {
        &self.rules
    }
    pub fn set_rule(&mut self, key: &str, value: bool) -> Result<()> {
        let key = self
            .rules
            .keys()
            .copied()
            .find(|k| *k == key)
            .ok_or_else(|| Error::UnknownRule(format!("{key:?} for {}", self.pid())))?;
        self.rules.insert(key, value);
        Ok(())
    }
    /// Diagnostics collected while reading this board.
    pub fn diagnostics(&self) -> &[crate::Diagnostic] {
        self.diagnostics.as_slice()
    }

    pub fn cell(&self, row: usize, col: usize) -> &Obj {
        &self.cells[row * self.geom.cols + col]
    }
    pub fn cell_mut(&mut self, row: usize, col: usize) -> &mut Obj {
        &mut self.cells[row * self.geom.cols + col]
    }
    #[allow(dead_code)] // for genre ports that index pzprjs border ids
    /// Index into `borders` of pzprjs `board.border[id]`. The orders differ when
    /// hasborder is 2: pzprjs numbers the outer frame last, `borders` uses FileIO order.
    pub(crate) fn border_by_id(&self, id: usize) -> Option<usize> {
        let (bx, by) = self.geom.pzprjs_border_pos(id)?;
        self.geom.border_index(bx, by)
    }
    pub fn obj(&self, group: Group, bx: i32, by: i32) -> Option<&Obj> {
        match group {
            Group::Cell => self.geom.cell_index(bx, by).map(|i| &self.cells[i]),
            Group::Border => self.geom.border_index(bx, by).map(|i| &self.borders[i]),
            Group::Cross => self.geom.cross_index(bx, by).map(|i| &self.crosses[i]),
            Group::ExCell => self
                .geom
                .excell_index(bx, by)
                .and_then(|i| self.excells.get(i)),
        }
    }
    pub fn obj_mut(&mut self, group: Group, bx: i32, by: i32) -> Option<&mut Obj> {
        match group {
            Group::Cell => self.geom.cell_index(bx, by).map(|i| &mut self.cells[i]),
            Group::Border => self.geom.border_index(bx, by).map(|i| &mut self.borders[i]),
            Group::Cross => self.geom.cross_index(bx, by).map(|i| &mut self.crosses[i]),
            Group::ExCell => self
                .geom
                .excell_index(bx, by)
                .and_then(|i| self.excells.get_mut(i)),
        }
    }

    /// pzprjs `getObjectPos("obj", bx, by)`: cell or excell at a doubled point.
    pub fn obj_at(&self, bx: i32, by: i32) -> Option<&Obj> {
        if let Some(i) = self.geom.cell_index(bx, by) {
            return Some(&self.cells[i]);
        }
        self.geom
            .excell_index(bx, by)
            .and_then(|i| self.excells.get(i))
    }
    pub fn obj_at_mut(&mut self, bx: i32, by: i32) -> Option<&mut Obj> {
        if let Some(i) = self.geom.cell_index(bx, by) {
            return Some(&mut self.cells[i]);
        }
        let i = self.geom.excell_index(bx, by)?;
        self.excells.get_mut(i)
    }

    /// pzprjs `bd.getDot(bx, by)`: the object under a dot-grid point, by parity.
    /// `hascross` gates the cross squares, `hasborder` the outer frame.
    pub fn dot(&self, bx: i32, by: i32) -> Option<&Obj> {
        self.dot_group(bx, by).and_then(|g| self.obj(g, bx, by))
    }
    pub fn dot_mut(&mut self, bx: i32, by: i32) -> Option<&mut Obj> {
        let g = self.dot_group(bx, by)?;
        self.obj_mut(g, bx, by)
    }
    fn dot_group(&self, bx: i32, by: i32) -> Option<Group> {
        match (bx & 1, by & 1) {
            (1, 1) => Some(Group::Cell),
            (0, 1) | (1, 0) => Some(Group::Border),
            (0, 0) => Some(Group::Cross),
            _ => None,
        }
    }

    /// Whether any answer-side property is set.
    pub fn has_snapshot(&self) -> bool {
        let d = Obj::default();
        self.cells
            .iter()
            .chain(&self.borders)
            .chain(&self.crosses)
            .chain(&self.excells)
            .any(|o| {
                o.anum != d.anum
                    || o.qans != d.qans
                    || o.qsub != d.qsub
                    || o.qcmp != d.qcmp
                    || o.line != d.line
            })
    }

    pub fn clear_snapshot(&mut self) {
        self.cells
            .iter_mut()
            .chain(self.borders.iter_mut())
            .chain(self.crosses.iter_mut())
            .chain(self.excells.iter_mut())
            .for_each(Obj::clear_answer);
    }
}

pub const DEDUPE_POLICY: &str = "d4v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DedupeKey {
    /// `d4v1:<representative>`.
    pub key: String,
    /// Smallest canonical URL over the allowed symmetries.
    pub representative: String,
    /// The transform with `board.transform(t).to_url() == representative`.
    pub transform: Transform,
}

#[cfg(test)]
mod tests {
    use super::Geometry;

    #[test]
    fn pzprjs_border_ids_cover_every_border_once() {
        for hasborder in [1, 2] {
            for (cols, rows) in [(1, 1), (1, 3), (4, 1), (3, 2), (5, 4)] {
                let g = Geometry {
                    cols,
                    rows,
                    hasborder,
                    hascross: 0,
                    hasexcell: 0,
                    excell_rows: 1,
                    excell_cols: 1,
                    excell_corner: false,
                };
                let mut seen: Vec<usize> = (0..g.border_count())
                    .map(|id| {
                        let (bx, by) = g.pzprjs_border_pos(id).unwrap();
                        g.border_index(bx, by).unwrap()
                    })
                    .collect();
                if hasborder == 1 {
                    assert!(seen.iter().enumerate().all(|(i, &j)| i == j));
                }
                seen.sort_unstable();
                assert_eq!(seen, (0..g.border_count()).collect::<Vec<_>>());
                assert_eq!(g.pzprjs_border_pos(g.border_count()), None);
            }
        }
    }
}

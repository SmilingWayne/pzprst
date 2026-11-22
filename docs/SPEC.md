# pzprst — Specification and Development Plan

Status: v0.1 spec (2026-11-22).
Pilot genres: Slitherlink (`slither`) and Masyu (`mashu`) — implemented, see §13.

## 1. Purpose

pzprst is a small Rust library that reproduces **pzprjs's static board model and
its serialization contracts** — URL, FileIO, pzpr-trace state — and adds the
operations the PuzzleKit ecosystem needs on top: canonicalization, board
symmetries (rotate / flip) and symmetry-aware duplicate keys.

It exists because PuzzleKit and puzzlekit-dataset currently each have their own
partial, slightly different encoders. Example: PuzzleKit emits `slither/3/2/h12g3`
while pzprjs emits `slither/3/2/h17d` for the same board, so URL strings can't be
compared. pzprst is the single source of truth for "what is the pzpr form of this
board".

### 1.1 The contract

> For every supported genre, `pzprst` decodes exactly what pinned pzprjs decodes,
> and its canonical output is **byte-identical** to pzprjs
> `puzzle.getURL(URL_PZPRV3)` (body after `?`) and `puzzle.getFileData()`.
> Transforms are byte-identical to pzprjs `board.operate("flipx" | "flipy" | "turnl" | "turnr")`
> followed by the same encoders.

Reference: `SmilingWayne/pzprjs-trace` @ `d40aeab0b12e723a9ea10ebeece28fc2f6a7fe42`
(local checkout `../pzprjs`). Each fixture records this commit.

Where pzprjs's behaviour is an obvious accident (e.g. `cols=0` silently becoming a
10×10 board), pzprst returns an error and the divergence is listed in §9.

### 1.2 Non-goals

- Input/UI: mouse, keys, undo/redo, operation history, recordings, trial mode,
  rendering, error highlighting, timers.
- Solving or answer checking.
- Mapping cspuz output (PuzzleKit keeps this; it may later build pzprst Boards).
- Kanpen / heyawake-applet / old pzpr-applet URL formats; pencilbox files.
- Its own language bindings. Python reaches pzprst through PuzzleKit's Rust
  extension (§10.2); other hosts link the crate or wrap it in their own thin layer.

## 2. Example behaviour

| Input | `canonical_url` |
|---|---|
| `slither/3/2/h12g3` (PuzzleKit) | `slither/3/2/h17d` |
| `https://puzz.link/p?slitherlink/3/2/h17d` | `slither/3/2/h17d` |
| `slither/3/2/h17d/` or `slither/3/2/h17dzzz` | `slither/3/2/h17d` (+ `TrailingData` diagnostic for the latter) |
| `masyu/6/6/00016f769000`, `pearl/…` | `mashu/6/6/00016f769000` |
| `mashu/6/6/0001` | `mashu/6/6/000100000000` (+ `TruncatedBody`) |
| `slither/xf/3/2/h17d` | `slither/f/3/2/h17d` (+ `UnknownFlag('x')`) |
| `slither/1/1/.` | `slither/1/1/./` (pzprjs appends `/` after a non-alphanumeric tail) |

Transforms of `slither/3/2/h17d` (from pzprjs):
`flipx → slither/3/2/b8c`, `flipy → slither/3/2/7db`,
`turnl → slither/2/3/1dgc`, `turnr → slither/2/3/cg3b`.

## 3. Architecture

### 3.1 Approach

Port pzprjs's **object model**, not a per-genre bespoke model. A board is a set of
pzpr objects addressed by pzpr's doubled coordinates `(bx, by)`:

- cell `(2c+1, 2r+1)`; cross `(2c, 2r)`;
- border with even `bx`/odd `by` (vertical border, separates left/right cells)
  and odd `bx`/even `by` (horizontal border);
- excell outside the grid: pzprjs `Board.hasexcell` 1 puts `cols*excellRows`
  cells above and `rows*excellCols` left (plus an optional top-left one when
  `emptyexcell.ques == 51`), 2 puts a row on each side; `Board::obj_at` mirrors
  pzprjs `getObjectPos("obj", …)`, and `Group::ExCell` addresses them directly.
  Crosses are a `(cols+1) x (rows+1)` grid gated on `Board.hascross`.

Each object carries the same property names as pzprjs (`qnum`, `qnum2`,
`qnum3`, `qnum4`, `ques`, `qdir`, `line`, `qsub`, …) with the same defaults
(`qnum*` = -1, `ques`/`qdir`/`qans`/`qsub`/`qcmp`/`line` = 0, `anum` = -1).
Codecs are line-by-line ports of pzprjs functions (`decode4Cell`,
`genericDecodeTriple`, `decodeObj`, …), so a reviewer can diff them against the
JS. Transforms are a port of `BoardExec.turnflipGroup` (pure coordinate maps)
plus the two per-genre directional hooks, mirroring pzprjs `BoardExec`:

- `Genre::adjust_board_data_before` = `adjustBoardData`: runs on the
  pre-transform content, before the coordinates move;
- `Genre::adjust_board_data` = `adjustBoardData2`: runs on the transformed
  board.

pzprjs calls the hooks once per board *operation*, while pzprst calls them once
with the composed `Transform`; a genre with directional values therefore applies
the operations in `tools/gen_fixtures.js` order (rot90 = `turnr`,
transpose = `turnl` + `flipy`, …) inside its hook.

Rejected alternatives: (a) typed per-genre grids only — convenient but every
codec and transform would be a re-derivation, which is how PuzzleKit drifted;
(b) running pzprjs in an embedded JS engine — exact but heavy, slow, and keeps
Node-shaped semantics in Python wheels.

### 3.2 Repository layout

```text
pzprst/
├── Cargo.toml                 workspace
├── crates/
│   ├── pzprst/                core library (deps: base64, miniz_oxide)
│   │   ├── src/lib.rs         public API re-exports
│   │   ├── src/error.rs       Error, Diagnostic
│   │   ├── src/jsnum.rs       JS Number()/parseInt() semantics where pzprjs relies on them
│   │   ├── src/url.rs         URL wrapper parsing / generation (port of parser.js URLData)
│   │   ├── src/variety.rs     pid / alias table (generated from pzprjs variety.js)
│   │   ├── src/board.rs       Board, Geometry, object stores, property defaults
│   │   ├── src/transform.rs   D4 transforms (turnflipGroup port), composition
│   │   ├── src/encode.rs      ports of variety-common/Encode.js (4cell, circle, …);
│   │   │                      becomes a directory once it grows
│   │   ├── src/fileio.rs      FileIO reader/writer (puzzle/FileData.js + common helpers)
│   │   ├── src/state.rs       pzpr-trace state (raw DEFLATE + base64url), trace matching
│   │   ├── src/genres/        mod.rs (trait) + one <pid>.rs per genre
│   │   ├── build.rs           registers every src/genres/<pid>.rs (no shared list to edit)
│   │   └── tests/             corpus properties + opt-in pzprjs parity, behaviour.rs
│   └── pzprst-cli/            `pzprst [--strict] [--diag] [--transform T] canon|dedupe|fileio|state`
├── fixtures/                  committed compact input JSONL; full/ is local (gitignored)
├── tools/
│   ├── genres/<pid>.js        fixture inputs per genre (edge-case URLs, FileIOs, randomizer)
│   ├── gen_fixtures.js        compact inputs / temporary pzprjs references
│   ├── check_parity.js        generate references and compare every input
│   ├── triage.js              per-genre porting needs; --all; --plan (parallel waves)
│   ├── pzprjs.js              pzprjs output for one input (CLI-compatible arguments)
│   └── parity.js              dataset-wide comparison
├── .agents/skills/pzprst-port-genre/   worker skill + dispatch workflow for genre ports
└── docs/SPEC.md
```

Adding a genre touches only `src/genres/<pid>.rs`, `tools/genres/<pid>.js` and
the generated `fixtures/<pid>.jsonl`; common codecs it needs are appended to
`encode.rs` / `fileio.rs` with a doc comment starting with the pzprjs function
name in backticks, which `tools/triage.js` uses to know what is ported.

### 3.3 Genre plug-in contract

```rust
pub(crate) trait Genre {
    fn pid(&self) -> &'static str;
    fn hasborder(&self) -> u8;                                     // later: hascross, hasexcell …
    fn rule_flags(&self) -> &'static [(char, &'static str)];       // pflag char ↔ config name
    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String; // decodePzpr
    fn encode_url(&self, b: &Board) -> Result<String>;             // encodePzpr
    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()>;
    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()>;
    fn validate(&self, b: &Board, d: &mut Diagnostics);            // ranges, URL-representability
    fn symmetries(&self) -> &'static [Transform] { &Transform::ALL }
    fn adjust_board_data(&self, _t: Transform, _b: &mut Board) {}  // directional content
}
```

Rule options decoded from `pflag` / FileIO flag lines live in `Board.rules`,
keyed by the pzprjs config name (`slither_full`, `loop_full`), and are part of
puzzle identity. A `v:<id>` variant is preserved verbatim in `Board.variant`;
like pzprjs, FileIO does not store it.

## 4. Data flow

```text
URL string ──url::parse──► UrlParts{pid, mode, pflag, variant, cols, rows, body, params}
            ──variety::to_pid──► Genre ──decode_url──► Board (+ Diagnostics)
FileIO text ──fileio::parse header──► Genre ──decode_fileio──► Board (+ snapshot)
state       ──base64url → inflate-raw──► FileIO text ──► Board
trace URL   ──► Board(url) + Board(state) ──check_match──► Board(state)
Board::parse picks one of the above: FileIO text, trace URL (non-empty state=),
             bare state (no `/`), otherwise puzzle URL.

Board ──encode_url──► canonical body (callers add a host prefix)
Board ──encode_fileio──► FileIO text ──deflate-raw → base64url──► state
Board ──transform(t)──► Board          Board ──dedupe_key──► (key, representative, transform)
```

## 5. Formats

### 5.1 URL wrapper (port of `URLData.parseURLType/parseURLData/outputURLData`)

Accepted: any host (`puzz.link/p?`, `pzv.jp/p.html?`, `pzprxs.vercel.app/p?`,
`pzpr-trace.net/p?`), or a bare body. Before handing the query to the pzprjs-port,
pzprst strips leading `key=value&` parameters itself (pzprjs's `\w+=\w+&` regex
cannot skip a base64url `state` containing `-`). Recognised parameters: `type`
(mode, ignored for identity), `state`, `trace`; others → `UnknownParameter`
diagnostic. `_edit` / `_play` suffixes set the mode and are dropped.

Then, exactly as pzprjs: optional `v:<id>` segment, optional non-numeric `pflag`
segment (JS `isNaN` semantics), `cols`, `rows`, body = remaining segments joined
by `/` with one trailing empty segment removed.

`cols`/`rows` must be integers in `1..=MAX_DIM` (default 1024); otherwise
`InvalidSize` (divergence: pzprjs falls back to 10×10).

`URL_PZPRFILE` (`?pzprv3/…` file-in-URL) — phase 2, reuses FileIO.
Kanpen/heyaapp/applet types → `UnsupportedUrlType`.

Output: `[v:<id>/][pflag/]cols/rows/body`, `pid` = genre `urlid`; append `/` if
the result's last char is not `[A-Za-z0-9]`.

### 5.2 Genre body codecs (pilot)

| Genre | Body | Flags | Values |
|---|---|---|---|
| `slither` (aliases `slitherlink`) | `encode4Cell` | `f` → `slither_full` | `qnum ∈ {-1,-2,0..4}` |
| `mashu` (aliases `masyu`, `pearl`) | `encodeCircle` (base-27 triples) | `f` → `loop_full` | `qnum ∈ {-1,-2,1,2}`; `-2` not representable in URL (`LossyUrl`) |

Like pzprjs, flags are only read when the body is non-empty (`mashu/f/3/3` has
`loop_full = false`; reported as `UnknownFlag`).

Ura-masyu (`uramashu`) is a volatile UI setting not stored in URL or FileIO and is
out of scope.

Decoder fidelity notes (all mirrored, each raises a diagnostic):
- `decode4Cell`: characters outside `0-9 a-e g-z .` advance one cell
  (`UnexpectedChar`); stops when cells run out (`TrailingData` if input remains).
- `genericDecodeTriple`: consumes `min(ceil(n/3), len)` chars via JS
  `parseInt(c, 27)`; non-digits yield 0 (`UnexpectedChar`); short body pads
  with empty (`TruncatedBody`).
- Unknown pflag characters (`UnknownFlag`).

### 5.3 FileIO (port of `FileIO.filedecode/fileencode` + common helpers)

Header: `[v:<id>]\npzprv3[.N]\n<pid>\n<rows>\n<cols>\n` (rows before cols).
Input may use `/` instead of newlines (pzprjs `parseFile` does the same).
Body helpers are ported literally: `getItemList` reads a fixed number of lines,
splits on spaces and flattens tokens (so tokens may wrap lines), `decodeConfigFlag`
consumes a line only if it equals the flag.

| Genre | Written version | Layout |
|---|---|---|
| `slither` | `pzprv3.1` (reads 0 and 1) | `[f]`, cell qnum (`n`/`-`/`.`), cell qsub (bg colour 0/1/2), borders `hasborder=2`: R lines × (C+1) vertical then (R+1) lines × C horizontal; `n`=line, `-1`=cross, `0` |
| `mashu` | `pzprv3` | `[f]`, cell qnum, borders `hasborder=1`: R × (C-1) vertical then (R-1) × C horizontal |

Trailing `info:{…}` (metadata/history) and `TrialData` blocks are not board
state: dropped with `DroppedMetadata` / `DroppedHistory` / `DroppedTrial`.
Unsupported file versions → `UnsupportedFileVersion` (divergence: pzprjs silently
loads nothing).

### 5.4 Snapshot

The snapshot is the set of non-question properties FileIO stores: for `slither`
`border.line`, `border.qsub` (cross), `cell.qsub` (background colour, kept because
FileIO round-trips it); for `mashu` `border.line`, `border.qsub`.
`Board.has_snapshot()` is true iff any of these is non-default.
`to_url` never includes the snapshot.

### 5.5 pzpr-trace state

`state = base64url_nopad(deflate_raw(utf8(fileio)))`. Decoding follows
pzpr-trace's URL path (`src-ui/state.js`, `decode(value, manual=false)`): only
`[A-Za-z0-9_-]` without padding, raw DEFLATE, UTF-8. The lenient manual-import
path (standard base64, padding, zlib) is not reproduced. Inflated size is capped
at pzpr-trace's 1 MiB by default (`ResourceLimit`). Compressed bytes are not
required to match the browser's `CompressionStream`; the contract is on the
inflated FileIO text.

Trace URLs `…/p?state=S&<pid>/<cols>/<rows>/<body>`:
1. Decode URL → board U; decode state → board S.
2. Require equal pid, cols, rows, rules (`full`, variant) and question properties
   (`qnum` …); else `StateMismatch{field}`. No symmetry normalization before
   matching.
3. Return S (question from U ≡ S, snapshot from S).
A non-empty `trace` param → `TraceNotSupported` unless `ignore_trace = true`
(then `IgnoredTrace` diagnostic).

## 6. Building boards

There is no JSON view; URL and FileIO are the interchange formats. Rust callers
build boards with `Board::new(pid, cols, rows)`, `cell_mut(r, c)`,
`obj_mut(Group, bx, by)` (pzprjs property names) and `set_rule(name, on)`.
A typed view can be added when a consumer (e.g. a WASM/TS host) needs one.

## 7. Transforms and duplicate detection

### 7.1 Transforms

D4 elements with pzprjs mapping (`C`,`R` = old cols/rows; `TURN*` swap dims):

| Name | pzprjs | `(bx, by) →` |
|---|---|---|
| `identity` | — | `(bx, by)` |
| `rot90` (clockwise) | `turnr` | `(2R − by, bx)` |
| `rot180` | `turnr`×2 | `(2C − bx, 2R − by)` |
| `rot270` | `turnl` | `(by, 2C − bx)` |
| `flipx` (mirror left↔right) | `flipx` | `(2C − bx, by)` |
| `flipy` (mirror top↔bottom) | `flipy` | `(bx, 2R − by)` |
| `transpose` | `turnl`,`flipy` | `(by, bx)` |
| `antitranspose` | `turnr`,`flipy` | `(2R − by, 2C − bx)` |

Applied to every object group and to the snapshot, then
`genre.adjust_board_data(t)` (directional content; no-op for the pilot). `compose`,
`inverse` are provided; group laws are property-tested.

### 7.2 Canonical URL vs dedupe key

- `canonical_url(board)` = pzprjs encoding of the board in its **current**
  orientation. Normalizes encodings and aliases, never orientation.
- `dedupe_key(board)` = `"d4v1:" + min_bytes{ canonical_url(t(board)) | t ∈ D4 }`
  plus the element `t*` attaining it (first in table order on ties) so callers can
  map an answer between orientations. Snapshot is ignored. Rules and variant are
  part of the URL, so different rules never collide.
- The policy is versioned by the key prefix (`DEDUPE_POLICY = "d4v1"`).
  Changing the representative choice requires a new policy string.

Genres may restrict the group (`Genre::symmetries`); both pilot genres allow
full D4 (clues are orientation-free).

## 8. Errors and diagnostics

`Error`: `InvalidUrl`, `UnknownPuzzle(pid)`, `UnsupportedPuzzle(pid)` (pzprjs knows
it, pzprst doesn't yet), `UnsupportedUrlType`, `InvalidSize`, `InvalidFileIo{line,
msg}`, `UnsupportedFileVersion`, `ValueOutOfRange`, `StateDecode`,
`StateMismatch{field}`, `TraceNotSupported`, `ResourceLimit`, `UnknownRule`,
`InvalidTransform`, `Strict(Vec<Diagnostic>)`. `Error::kind()` gives the stable
variant name for bindings.

`Diagnostic { code, message }` codes (one entry per code): `TrailingData`,
`TruncatedBody`, `UnexpectedChar`, `UnknownFlag`, `UnknownParameter`,
`IgnoredState`, `IgnoredTrace`, `LossyUrl`, `ValueOutOfRange`,
`DroppedMetadata`, `DroppedHistory`, `DroppedTrial`. All diagnostics are
produced while reading and are on `Board::diagnostics()`; writing never adds any.
`LossyUrl` marks question data the URL cannot carry (a mashu `?` circle). The
answer snapshot and FileIO's missing variant are documented losses, not
diagnostics.

Default mode is **lenient** (decode exactly like pzprjs, return diagnostics).
`strict = true` turns any diagnostic into `Error::Strict`. Nothing that pzprjs
would reject is accepted silently by pzprst; nothing is lost without a diagnostic.

## 9. Intentional divergences from pzprjs

| pzprjs | pzprst |
|---|---|
| `cols`/`rows` ≤ 0 or NaN → 10×10 board | `InvalidSize` |
| `\w+=\w+&` query skipping fails on base64url state | generic `key=value&` stripping |
| Unsupported FileIO version silently loads nothing | `UnsupportedFileVersion` |
| Values outside genre range accepted from FileIO | kept + `ValueOutOfRange` diagnostic; URL encode errors as pzprjs throws (mashu) or emits whatever pzprjs emits (slither) |
| Non-numeric FileIO tokens become `NaN` properties | `InvalidFileIo` |
| `?v:x/pid/…` (variant before pid) mis-parses the pid as a flag | `UnsupportedUrlType` |

Each row has a fixture.

## 10. Bindings

pzprst ships only the Rust crate and a CLI. Hosts in other languages wrap the
crate themselves, exposing just what they need.

### 10.1 Rust

```rust
use pzprst::{canonical_url, dedupe_key, Board, Options, Transform};
let opts = Options::default();                    // strict, ignore_trace, max_state_bytes
canonical_url("masyu/6/6/00016f769000", &opts)?;  // "mashu/6/6/00016f769000"
let b = Board::parse(input, &opts)?;              // URL, trace URL, state or FileIO
b.diagnostics();                                  // read-time repairs
b.transform(Transform::Rot90).to_url()?;          // new Board, canonical URL body
b.dedupe_key()?;                                  // DedupeKey { key, representative, transform }
b.to_fileio()?; b.to_state()?; b.to_trace_url("https://pzpr-trace.net/p?")?;
```

### 10.2 Python, via PuzzleKit

PuzzleKit's Rust extension (`puzzlekit._cspuz`) depends on the `pzprst` crate and
exports `pzpr_canonical_url`, `pzpr_transform_url`, `pzpr_dedupe_key`,
`pzpr_supported`, `PZPR_TRANSFORMS` and `PZPRJS_COMMIT`. The public wrapper is
`puzzlekit.pzpr` (`canonical_url`, `transform_url`, `dedupe_key`,
`supported_puzzles`); URL results carry the `https://puzz.link/p?` prefix and
errors are `ValueError("<Error::kind>: <message>")`. `puzzlekit.solve` uses it for
`normalized_url` of supported genres, so `to_trace_url()` is canonical too.
puzzlekit-dataset uses the same module. Until pzprst is published, PuzzleKit
refers to it by a workspace path; switch to a pinned git rev before release.

### 10.3 TypeScript / WASM (when needed)

The core crate builds for `wasm32-unknown-unknown` unchanged. A host-side crate
(e.g. in puzzlekit-web) adds `wasm-bindgen` exports and gets generated `.d.ts`.

### 10.4 CLI

`pzprst [--strict] [--diag] [--transform NAME] canon|dedupe|fileio|state [input…]`.
Any input form is accepted (`Board::parse`); with no input it reads one input per
line from stdin and prints one result per line (`ERR <kind>: <message>` on
failure). Used for shell batches and by `tools/parity.js`.

## 11. Testing

1. **Reference parity**: `fixtures/<pid>.jsonl` contains compact inputs generated
   from `tools/genres/<pid>.js`: hand-written edge cases and 30 seeded random
   boards per genre (≤ 6×5, with snapshots and rule flags). Every registered
   genre must have one. `node tools/check_parity.js [pid...]` runs the pinned,
   clean, freshly built **SmilingWayne/pzprjs-trace** to generate temporary
   reference outputs, then compares canonical URL and FileIO for each input and
   all eight transformed outputs. This check is opt-in in `cargo test`; missing
   references or an unexpected commit fail the explicit parity command. See the
   README for setup. `node tools/gen_fixtures.js --full <pid>` also writes the
   gitignored `fixtures/full/` (120 larger random boards and up to 60 dataset
   URLs); `node tools/check_parity.js --full <pid>` includes these inputs.
2. **Property tests** (`cargo test`, no Node): decode∘encode = id on input boards; transform group laws
   (`t ∘ t⁻¹ = id`, `rot90⁴ = id`, `flipx² = id`); `dedupe_key(t(b)) == dedupe_key(b)`
   for all t; `canonical(canonical(u)) == canonical(u)`.
3. **Full-dataset parity** (dev-only, `tools/parity.js` + `pzprst-cli`): every
   Slitherlink/Masyu `puzzlink_url` in puzzlekit-dataset canonicalized by both
   implementations; report must show 0 differences.
4. **PuzzleKit**: `tests/test_pzpr.py` covers the Python surface and
   `normalized_url`; its pzpr-trace import test checks the states end to end.
5. **State**: round-trip on fixtures; strict-encoding rejection; oversize input.

## 12. Development plan

**Phase 0 — Baseline (this change).** Workspace, fixture generator, variety table,
URL wrapper, Board/geometry, transforms, error model.

**Phase 1 — Pilot `slither` + `mashu` (this change).** URL, FileIO, state, trace
URL matching, transforms, canonical + dedupe, CLI, fixture + property +
dataset-parity tests. PuzzleKit links the crate and uses it for `normalized_url`.

**Phase 2 — Model stress test.** Genres that exercise the remaining object kinds
and transforms: `yajilin` (directional clues → `adjust_board_data`), `easyasabc`
(excell + `indicator.count`), `battleship` (piece bank), one region genre
(`heyawake`, borders + room numbers) and `URL_PZPRFILE`.

**Phase 3 — Integration.**
- Publish pzprst and pin PuzzleKit to a git rev.
- puzzlekit-dataset: replace the Node `pzprcodec` canonicalize/dedupe path with
  `puzzlekit.pzpr` for supported genres (dry-run report first; key = `dedupe_key`).
- PuzzleKit: cross-check adapter FileIO/state output against pzprst before
  deciding whether adapters should build pzprst Boards.

**Phase 4 — Breadth.** Genre families by dataset demand, each with fixtures and
a capability row; a WASM host crate if puzzlekit-web needs it.

Capability matrix (pilot):

| pid | URL dec/enc | FileIO r/w | state r/w | transforms | dedupe | snapshot marks |
|---|---|---|---|---|---|---|
| slither | ✓ | ✓ (v0 read, v1) | ✓ | D4 | d4v1 | lines, crosses, cell bg |
| mashu | ✓ | ✓ | ✓ | D4 | d4v1 | lines, crosses |

## 13. Implementation status (pilot, 2026-11-22)

Implemented: everything in Phases 0–1 for `slither` and `mashu`.

Historical pilot verification (the test layout has since changed; see §11):

| Check | Result |
|---|---|
| Fixture parity (173 committed + 600 full records × {canonical, FileIO} × 8 transforms) | identical |
| Full dataset (`tools/parity.js`): 4830 Slitherlink + 3175 Masyu `puzzlink_url`, canonical + 7 transforms | 64 040 / 64 040 identical; 1284 + 858 sources were not canonical |
| Property tests: FileIO/state round-trip, idempotence, group laws, dedupe invariance | pass |
| PuzzleKit with pzprst linked: full pytest incl. pzpr-trace state import | 442 passed |
| PuzzleKit 0.4.0 `to_trace_url()` (2 slither, 1 mashu) read in strict mode | match, lossless |
| Dataset D4 duplicates (same records) | 3 Slitherlink, 5 Masyu groups found only by symmetry |

Commands:

```bash
cargo test                                   # Rust properties and behavior
node tools/check_parity.js                   # full pinned pzprjs-trace comparison
node tools/gen_fixtures.js [--full]          # regenerate inputs (needs ../pzprjs build)
node tools/gen_variety.js                    # regenerate the pid/alias table
cargo build --release -p pzprst-cli
node tools/parity.js --bin target/release/pzprst
cd ../puzzlekit && pip install -e ".[dev]" && pytest -q   # Python surface
```

Next: Phase 2 (`yajilin`, `easyasabc`, `battleship`, `heyawake`, `URL_PZPRFILE`).

Porting backlog (`node tools/triage.js --all`, pzprjs d40aeab0, 346 pids):
141 ported, 11 triage-ready but blocked on a missing core feature, 7 blocked on
cross objects, 186 need core infrastructure (directional hooks, crosses, rooms,
excells, piece bank, array properties, indicators, segments), 1 cannot be
introspected. Ready and codec-only genres are dispatched in waves (`--plan`, see
`.agents/skills/pzprst-port-genre/dispatch.md`); infrastructure features are
maintainer work and unblock the rest by feature.

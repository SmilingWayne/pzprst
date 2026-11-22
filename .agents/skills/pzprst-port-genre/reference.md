# pzprjs → pzprst reference

Paths are relative to the `pzprst/` repo root. pzprjs sources are in
`../pzprjs/src/` (`variety/<file>.js` per genre, shared code in
`variety-common/Encode.js`, `variety-common/FileData.js`, `puzzle/*.js`).

## Contents

- [Board model](#board-model)
- [Genre trait](#genre-trait)
- [Translating pzprjs code](#translating-pzprjs-code)
- [Existing codecs](#existing-codecs)
- [Diagnostics and errors](#diagnostics-and-errors)
- [Fixture config](#fixture-config)
- [Debugging a mismatch](#debugging-a-mismatch)
- [Known pitfalls](#known-pitfalls)

## Board model

`crate::board::Board` (fields are `pub(crate)`; genre code may use them):

| pzprjs | pzprst |
|--------|--------|
| `bd.cols`, `bd.rows` | `b.cols()`, `b.rows()` |
| `bd.cell[c]` (row-major) | `b.cells[c]` (same order) |
| `bd.border[id]` | `b.borders[b.border_by_id(id).unwrap()]` (see below) |
| `bd.getc(bx, by)` | `b.obj(Group::Cell, bx, by)` / `b.obj_mut(...)` → `Option` |
| `bd.getb(bx, by)` | `b.obj(Group::Border, bx, by)` / `b.obj_mut(...)` |
| `cell.bx`, `cell.by` | `b.geom.cell_pos(c)` |
| `bd.hasborder` | `b.geom.hasborder` (set from `Genre::hasborder`) |
| `puzzle.getConfig("x")` for rule configs | `b.rules["x"]` (`BTreeMap<&str, bool>`) |

Coordinates are pzprjs doubled coordinates: cell `(col, row)` is at
`(2*col+1, 2*row+1)`; a vertical border has even `bx`, odd `by`; a horizontal
border has odd `bx`, even `by`.

**Border order.** `b.borders` is in FileIO order (all vertical borders row by
row, then all horizontal rows). pzprjs `bd.border[id]` numbers inner borders
the same way but puts the outer frame **last** (hasborder 2). When pzprjs code
loops over `bd.border[id]` or uses `border.id`, translate through
`b.border_by_id(id)`. When it loops by coordinates, use `b.obj(Group::Border, bx, by)`.

Every object (`crate::Obj`) has the same properties with pzprjs defaults:

| property | default | usual meaning |
|----------|---------|---------------|
| `qnum` | -1 | clue number; -2 is `?` |
| `qnum2` | -1 | second clue number |
| `ques` | 0 | shaded/special question cell, border wall (`ques=1`) |
| `qdir` | 0 | clue arrow direction |
| `anum` | -1 | answer number |
| `qans` | 0 | answer shading / answer border |
| `qsub` | 0 | auxiliary mark (dot, cross, colour) |
| `qcmp` | 0 | "clue done" mark |
| `line` | 0 | line on a border |

If pzprjs uses any other property (`snum`, `qnums`, `qnum3`, `qchar`, …), the
genre is BLOCKED (needs an `Obj` change).

## Genre trait

Defined in `crates/pzprst/src/genres/mod.rs`. Fill each method from pzprjs:

| method | from pzprjs |
|--------|-------------|
| `pid()` | the pid string exactly as triage prints it |
| `hasborder()` | `Board: { hasborder: N }` (0 when absent) |
| `rule_flags()` | `(char, "config")` pairs: the URL pflag letter checked by `checkpflag("x")` in `decodePzpr`, written by `outflag` in `encodePzpr`, read/written by `decodeConfigFlag("x", "config")` in FileIO. Empty `&[]` when none. Order = order pzprjs writes them. |
| `decode_url(b, body, d)` | `Encode.decodePzpr`. Return the unconsumed rest (`this.outbstr`). Call decoders in the same order, passing the rest along. |
| `encode_url(b)` | `Encode.encodePzpr`. Concatenate the encoder outputs in the same order. Flags are written by the caller; do not add them. |
| `decode_fileio(r, b, d)` | `FileIO.decodeData`, including `r.filever` branches and `decodeConfigFlag` calls in order |
| `encode_fileio(b, w)` | `FileIO.encodeData`. Set `w.filever = N` exactly when pzprjs sets `this.filever`. |
| `validate(b, d)` | Value checks for boards read from FileIO or state. Push `ValueOutOfRange` for values pzprjs input could never produce (outside `minnum`..`maxnum`, unknown property values). Push `LossyUrl` only when **question** data (`qnum`, `qnum2`, `ques`, `qdir`) is legal on the board but dropped by `encodePzpr` (e.g. a `?` the URL cannot store). Answer properties (`anum`, `qans`, `qsub`, `qcmp`, `line`) are never part of a URL; they must not trigger `LossyUrl`, or every solved board fails in strict mode. |
| `symmetries()` | Leave the default unless rotating the board changes the puzzle meaning; then BLOCKED. |
| `adjust_board_data()` | Leave the default. pzprjs overrides mean BLOCKED (triage reports them). |

`maxnum` may be a function of the board size (`maxnum: function() { return
Math.max(this.board.cols, this.board.rows); }`). Compute it in `validate` from
`b.cols()` / `b.rows()` the same way.

## Translating pzprjs code

Split the body once: `let chars: Vec<char> = bstr.chars().collect();`

| JavaScript | Rust |
|------------|------|
| `bstr.charAt(i)` | `chars[i]` (check `i < chars.len()`) |
| `bstr.substr(i, n)` | `jsnum::substr(&chars, i, n)` |
| `bstr.substr(i)` / `this.outbstr = bstr.substr(i)` | `chars[i.min(chars.len())..].iter().collect::<String>()` returned from the decoder |
| `parseInt(s, 16)` (any string) | `jsnum::parse_int(&s, 16)` → `None` means NaN |
| `parseInt(ca, 36)` (one char) | `jsnum::digit(ca, 36)` → `Option<u32>` |
| `this.include(ca, "0", "9")` | `('0'..='9').contains(&ca)` |
| `ca >= "g" && ca <= "z"` | `('g'..='z').contains(&ca)` |
| `n.toString(16)` | `jsnum::to_radix(n as u32, 16)` |
| `s.padStart(3, 0)` | `format!("{:0>3}", s)` |
| `+ca`, `Number(ca)` in FileIO | `fileio::int(ca)?` inside a `decode_cell` closure |
| `(x / 3) | 0`, `x >> 1` | integer division on non-negative `i32`/`usize` |
| `this.outbstr += cm` in an encoder | return `cm` (`String`, or `Result<String>` when the encoder can fail) |

Signatures used by existing codecs; copy them for new ones:

```rust
// URL decoder (encode.rs): returns the unconsumed rest of the body.
pub fn decode_xxx(b: &mut Board, bstr: &str, d: &mut Diagnostics) -> String
// URL encoder: returns the body fragment. Use Result only if pzprjs can throw.
pub fn encode_xxx(b: &Board) -> String
pub fn encode_xxx(b: &Board) -> Result<String>
// generic* helpers take a length and closures, like pzprjs set_func/get_func.
pub fn decode_triple(length: usize, bstr: &str, d: &mut Diagnostics, set: impl FnMut(usize, i32)) -> String
pub fn encode_triple(length: usize, get: impl FnMut(usize) -> i32) -> Result<String>

// FileIO property codecs (fileio.rs).
pub(crate) fn decode_xxx(r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()>
pub(crate) fn encode_xxx(w: &mut FileWriter, b: &Board)
```

FileIO helpers on `FileReader` / `FileWriter`:

- `r.decode_cell(b, d, |cell, ca| { …; Ok(()) })`, `r.decode_border(b, d, …)`:
  one token `ca` per object, like pzprjs `decodeCell` / `decodeBorder`.
  Return `Err(message)` where pzprjs would read garbage (non-numeric token).
- `w.encode_cell(b, |cell| format!(…))`, `w.encode_border(b, …)`: return the
  token **including** its trailing space (`"1 "`, `". "`), exactly like pzprjs.
- `r.read_line()`, `w.write_line(s)`, `r.decode_config_flag(b, "f", "key")`,
  `w.encode_config_flag(b, "f", "key")`, `r.filever`, `w.filever`.

Worked example: the pzprjs loop

```javascript
while (i < bstr.length && c < length) {
	var ca = bstr.charAt(i);
	var res = this.readNumber16(bstr, i);
	if (res[0] !== -1) { set_func(c, res[0]); i += res[1]; c++; }
	else if (ca >= "g" && ca <= "z") { c += parseInt(ca, 36) - 15; i++; }
	else { i++; }
}
this.outbstr = bstr.substr(i);
```

becomes

```rust
let chars: Vec<char> = bstr.chars().collect();
let (mut c, mut i) = (0usize, 0usize);
while i < chars.len() && c < length {
    let ca = chars[i];
    let (val, used) = read_number16(&chars, i);
    if val != -1 {
        set(c, val);
        i += used;
        c += 1;
    } else if ('g'..='z').contains(&ca) {
        c += jsnum::digit(ca, 36).unwrap() as usize - 15;
        i += 1;
    } else {
        d.push(DiagnosticCode::UnexpectedChar, format!("unexpected character {ca:?} is skipped"));
        i += 1;
    }
}
if c < length {
    d.push(DiagnosticCode::TruncatedBody, "body ends before the last cell");
}
chars[i.min(chars.len())..].iter().collect()
```

The silent `else { i++; }` branch is kept and only reported as a diagnostic.

## Existing codecs

Run `rg -n '^/// \`' crates/pzprst/src/encode.rs crates/pzprst/src/fileio.rs`
for the current list. Every function whose doc comment starts with a pzprjs
name in backticks is a port of that function.

## Diagnostics and errors

- Where pzprjs silently skips, clamps or ignores input, produce the same board
  and push a diagnostic: `UnexpectedChar`, `TruncatedBody`, `TrailingData`,
  `ValueOutOfRange`, `LossyUrl` (see `crate::error::DiagnosticCode`). Never
  return an error there.
- Return `Err(Error::ValueOutOfRange(..))` only where pzprjs throws.
- If JavaScript would store `NaN` (e.g. `parseInt` of a truncated escape), push
  `UnexpectedChar`, leave the object at its default, and do not put that input
  into the fixture config (pzprjs output containing NaN cannot be matched).
  Mention it in the report notes.

## Fixture config

`tools/genres/<pid>.js` exports `{ pid, dataset, urls, fileios, randomize }`.

- `dataset`: the folder under `../puzzlekit-dataset/assets/data/` for this
  genre (`ls` it; names are the English genre name, e.g. `Masyu` for `mashu`,
  the 4th field of the pid's line in `../pzprjs/src/pzpr/variety.js`), or
  `null`. Never read the dataset JSON whole; `rg -o -m 3 'https?://[^"]*' <file>`
  gives sample URLs.
- `urls` (at least 12): the canonical form; every alias from the pid's line in
  `../pzprjs/src/pzpr/variety.js` (`kanpen`, `alias`, … names); a full
  `https://puzz.link/p?` URL; each pflag
  and their combination; a truncated body; trailing junk (`zz`); `?` if
  allowed; the largest allowed value; `1/1`, `1/N`, `N/1` boards; 2–3 real
  dataset URLs.
- `fileios` (at least 2): well-formed FileIO with question **and** answer
  data, one with every config flag line. Use one separator throughout: either
  all `/` or all `\n`, never mixed. Get valid text from pzprjs
  (`node tools/pzprjs.js fileio '<url>'`), then change answer tokens inside the
  layer that holds them; keep the line and token counts unchanged. pzprjs
  tolerates malformed text, so "0 pzprjs errors" does not prove it is well-formed.
- `randomize(p, r)`: set every property that the genre's FileIO stores, on every
  cell/border, to legal values only (mostly empty, sometimes each legal value,
  `-2` only if `?` is allowed); set each rule config with
  `p.setConfig("<key>", r.rnd() < 0.3)`. Use only `r.rnd()` and `r.pick([...])`,
  never `Math.random()`.

## Debugging a mismatch

The test prints up to 10 lines like:

```text
r12 rot90 fileio:
  got  "pzprv3/…"
  want "pzprv3/…"
```

1. Find the input: `rg '"id":"r12"' fixtures/<pid>.jsonl`.
2. Compare both implementations on that input:

   ```bash
   node tools/pzprjs.js canon '<input>'
   cargo run -q -p pzprst-cli -- canon '<input>'
   node tools/pzprjs.js --transform rot90 fileio '<input>'
   cargo run -q -p pzprst-cli -- --transform rot90 fileio '<input>'
   ```

3. Shrink: make a smaller board by hand that still differs; diff token by token.
4. Read the pzprjs function again from the top for that branch.

What the failing check points at:

| failing check | look at |
|---------------|---------|
| `canonical` only | `encode_url`, or the decoder left a value pzprjs would not |
| `fileio` only | `encode_fileio`, token spacing, `filever`, config flag order |
| both, every record | `decode_*` reads the wrong property or order |
| only transformed checks | orientation-dependent data → BLOCKED |
| panic "failed: …" | your decoder returned `Err` where pzprjs accepted the input |
| properties test: `idempotent` / `strict-clean` | encoder writes something its own decoder flags; compare with pzprjs encoder |

## Known pitfalls

- `decodePzpr`/`encodePzpr` may be overridden in an `Encode@<pid>` section;
  the plain `Encode` section then belongs to sibling pids.
- `filever`: decoders branch on old file versions; port every branch.
- Value 0 is a valid clue in many genres; "empty" is `-1`, `?` is `-2`.
- Encoders that skip the last cells still return a run-length marker; copy the
  `count` handling exactly (`count == 20` flushes, `15 + count` in base 36 is
  common, but check the function).
- Do not write the board size or pid in `encode_url`; the caller does.
- Some pzprjs FileIO writers append nothing for an all-empty layer and some
  readers tolerate missing lines; port the exact condition.

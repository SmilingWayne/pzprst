# pzprst

A Rust library and CLI for pzprjs-compatible puzzle URLs, FileIO, pzpr-trace
states, board symmetries, and deduplication. It implements static puzzle boards;
it does not solve puzzles. `pzprst::supported()` lists implemented pzprjs pids.
The reference is **[SmilingWayne/pzprjs-trace](https://github.com/SmilingWayne/pzprjs-trace)**,
not the upstream pzprjs repository; its pinned commit is `PZPRJS_COMMIT`.

```rust
use pzprst::{Board, Options, Transform};

let board = Board::from_url("slither/3/2/h12g3", &Options::default())?;
assert_eq!(board.to_url()?, "slither/3/2/h17d");
assert_eq!(board.transform(Transform::FlipX).to_url()?, "slither/3/2/b8c");
let fileio = board.to_fileio()?;
# Ok::<(), pzprst::Error>(())
```

The CLI has `canon`, `dedupe`, `fileio`, and `state` commands:

`Board::try_transform` and the CLI reproduce pzprjs transform errors and its
rectangular segment behavior. `Board::transform` remains an infallible geometric
D4 transform for callers that need its group properties.

```sh
cargo run -q -p pzprst-cli -- --transform rot90 fileio 'slither/3/2/h17d'
```

## Tests

Run the standalone Rust tests and the property tests over all committed inputs:

```sh
cargo test -q
cargo clippy --workspace --all-targets -q
```

`fixtures/<pid>.jsonl` contains compact test inputs, not stored pzprjs outputs.
Fixture inputs do not record the reference commit; the generator checks the
reference checkout against `PZPRJS_COMMIT` when producing comparison results.
The property tests check FileIO/state round trips, canonicalization, symmetries,
and deduplication for every registered genre; set `PZPRST_GENRE=slither` to run
only that genre. These tests do not require Node. The full pzprjs-trace comparison
is intentionally reported as ignored by `cargo test` and must be run separately:

```sh
node tools/check_parity.js                  # all committed inputs / all genres
node tools/check_parity.js slither mashu   # selected genres
node tools/check_state_parity.js slither mashu # pzprjs-trace accepts Rust states
```

For that comparison, use Node 24+ and a **clean checkout of pzprjs-trace** at
the pinned commit. With the repositories as siblings:

```sh
git clone https://github.com/SmilingWayne/pzprjs-trace.git ../pzprjs
# Check out the commit in crates/pzprst/src/variety_table.rs (PZPRJS_COMMIT).
cd ../pzprjs
npm ci
npm run build
cd ../pzprst
node tools/check_parity.js
```

If the checkout already exists, reuse it and run the checkout/build steps there.
For another path, pass `--pzprjs /path/to/pzprjs-trace` or set `PZPRJS`. The
tool verifies the commit and tracked source changes. `dist/js/pzpr.js` is an
untracked build artifact: **rebuild after checkout**; checking Git HEAD alone
does not establish which source produced an existing build.

The checker generates temporary expected outputs from the committed inputs,
compares canonical URL and FileIO before and after all eight transforms, and
removes the temporary files. Missing reference data or a wrong commit fails
the check. No pzprjs installation is needed to build or use the Rust library.

When adding/updating a genre, regenerate its compact inputs using the same clean,
built reference checkout, then run both property and reference tests:

```sh
node tools/gen_fixtures.js slither
PZPRST_GENRE=slither cargo test -q -p pzprst-conformance --test fixtures
node tools/check_parity.js slither
```

`node tools/gen_fixtures.js --full <pid>` also generates optional, gitignored
larger inputs (and dataset URLs if available); `node tools/check_parity.js --full
<pid>` compares both committed and full inputs. See [the specification](docs/SPEC.md)
for formats, intentional divergences, and the porting contract.

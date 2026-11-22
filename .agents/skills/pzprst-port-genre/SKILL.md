---
name: pzprst-port-genre
description: Use when assigned a pzprjs puzzle genre (a pid such as lither or nurikabe) to add to pzprst, the Rust port of pzpr board formats, or when a wave plan from tools/triage.js names a genre task or a codec carrier task.
---

# Port one pzprjs genre to pzprst

## Overview

pzprst must produce **byte-identical** URLs and FileIO to the pinned pzprjs
(`../pzprjs`) for every input and every board symmetry. You do not decide what
correct output is: pzprjs does. The generated reference comparison is the
judge, and a genre is done only when it passes.

Port the pzprjs code line by line. Do not simplify, "clean up" or re-design it.
Same branch order, same edge cases, same defaults.

Your task names a pid. A **codec carrier** task also lists common functions
("ports: decodeNumber16, …") that you add to the shared codec files. A carrier
task is a genre task **plus** those functions: you still write the genre and
its fixtures, and you are done only when triage says `PORTED`. The genre's
fixtures are what test the new functions.

## Hard rules

1. **Files you may touch:**
   - create `crates/pzprst/src/genres/<pid>.rs`
   - create `tools/genres/<pid>.js`
   - `fixtures/<pid>.jsonl`, written **only** by `node tools/gen_fixtures.js <pid>`
   - carrier tasks only: append functions to `crates/pzprst/src/encode.rs`
     (URL codecs) or `crates/pzprst/src/fileio.rs` (FileIO codecs)

   Everything else is read-only: `board.rs`, `url.rs`, `transform.rs`,
   `state.rs`, `genres/mod.rs`, other genres, tests, tools, `Cargo.toml`.
2. Never edit a fixture by hand, never change an expected value, never skip,
   weaken or `#[ignore]` a test, never add `#[allow(...)]` to hide a warning.
3. No new dependencies. No git commits unless your task says so.
4. **STOP and report BLOCKED** when any of these happens:
   - triage says `NEEDS_INFRA` or `ERROR`, unless the only infrastructure
     triage names is `adjustBoardData`/`adjustBoardData2`/directional values:
     pzprst now has both hooks (`Genre::adjust_board_data_before` and
     `Genre::adjust_board_data`, plus `Obj::qnum3`/`qnum4`). See reference.md
     § Directional content; the fixtures compare all eight transforms, so the
     hooks are what decides;
   - the port needs a change to a read-only file (a new `Obj` property,
     crosses, excells, rooms, array properties, piece banks, …);
   - a pzprjs URL flag changes how the body is decoded instead of setting a
     rule (look for `checkpflag` inside decode logic);
   - a common function you need is missing and your task did not list it;
   - fixtures still mismatch after 3 focused fix attempts.

   A precise BLOCKED report is a good result. A green test obtained by bending
   a rule is a failed task.

## Workflow

Run everything from the `pzprst/` repo root. Create one todo per step.

1. **Triage.** `node tools/triage.js <pid>`. Keep the output; it lists the
   source file, `hasborder`, cell value range, the four pzprjs entry points and
   every common function used, each marked `[ported]` or `[MISSING]`.
   - `PORTED`: report "already done" and stop.
   - `NEEDS_INFRA` / `ERROR`: report BLOCKED with the triage output and stop.
   - `READY`, or `NEEDS_CODEC` with every missing function in your task list:
     continue.
2. **Read the pzprjs source.** Open the file named on the `source:` line. Read
   every section whose name starts with `Encode`, `FileIO`, `Board`, `Cell`,
   `Border` or `Graphic`, including `"Name@<pid>"` sections. A `@<pid>` section
   overrides the shared one for your pid only. Note: `hasborder`, `maxnum` /
   `minnum`, `disInputHatena`, config flags (`checkpflag`, `decodeConfigFlag`),
   `filever` branches.
3. **Port missing common functions** (carrier tasks only). Find each one with
   `rg -n "<name>: function" ../pzprjs/src/variety-common ../pzprjs/src/puzzle`.
   Translate it into `encode.rs` or `fileio.rs` next to similar functions. The
   doc comment **must start with the pzprjs name in backticks**:
   ``/// `decodeNumber16`: cell numbers, one hex digit or a `-`/`+` escape.``
   Triage finds ported functions through this marker. Follow
   [reference.md](reference.md) for signatures and JS-to-Rust idioms.
4. **Write `crates/pzprst/src/genres/<pid>.rs`.** Start from a copy of
   `genres/mashu.rs` (cell clues + lines) or `genres/slither.rs` (hasborder 2,
   qsub). The file must define `pub(crate) static GENRE: &dyn Genre = &<Type>;`.
   `build.rs` registers it automatically. Fill in the trait as described in
   [reference.md § Genre trait](reference.md#genre-trait). Port the
   "genre-specific helpers" that triage lists (functions defined in the
   variety file, e.g. `decodeCellQcmp`) as private functions in this file,
   with the same backtick doc-comment marker.
5. **Write `tools/genres/<pid>.js`.** Start from a copy of
   `tools/genres/mashu.js`; see [reference.md § Fixture config](reference.md#fixture-config).
6. **Generate compact input fixtures.** `node tools/gen_fixtures.js <pid>` must print
   `0 pzprjs errors`. If pzprjs rejects one of your inputs, that input is
   invalid: fix or delete the input, not the output.
7. **Test the genre.**

   ```bash
   PZPRST_GENRE=<pid> cargo test -q --test fixtures
   node tools/check_parity.js <pid>
   ```

   On failure, follow [reference.md § Debugging a mismatch](reference.md#debugging-a-mismatch).
8. **Test everything.**

   ```bash
   cargo fmt --all
   cargo test -q
   cargo clippy --workspace --all-targets -q   # must print nothing
   ```

   Any warning is a failure. A `dead_code` warning on a function you ported
   means the genre that should call it is not written yet: go back to step 4.

9. **Dataset parity** (only if your config has a `dataset` folder that exists
   under `../puzzlekit-dataset/assets/data/`):

   ```bash
   cargo build -q --release -p pzprst-cli
   node tools/parity.js --bin "$(cargo metadata -q --format-version 1 | node -pe 'JSON.parse(require("fs").readFileSync(0)).target_directory')/release/pzprst" <pid>
   ```

   Every URL must be identical.
10. **Confirm.** `node tools/triage.js <pid>` must now say `PORTED`. `READY`
    means the genre file is missing: you are not done.
11. **Report** using the template below, and nothing else.

## Report template

```text
pid: <pid>
status: DONE | BLOCKED
files: <every file created or changed>
ported common functions: <names, or none>
fixtures: <record count> records, pzprjs errors 0
tests: fixtures(<pid>) pass | fail; cargo test pass | fail; clippy clean | <warnings>
parity: <dataset> <n>/<n> identical | no dataset
diagnostics: <diagnostic codes your genre emits and when>
notes: <anything surprising: pzprjs quirks reproduced, values you could not test>
blocked: <only if BLOCKED: what is missing, which read-only file would change, triage line or mismatch output as evidence>
```

## Red flags: stop and re-read the hard rules

| Thought | Reality |
|---------|---------|
| "The fixture must be wrong" | pzprjs is right by definition. Your port differs. |
| "I'll tweak `board.rs` / `url.rs` a little" | Read-only. Report BLOCKED. |
| "pzprjs does something odd here; I'll do the sensible thing" | Reproduce the odd thing. Note it in the report. |
| "Only the transform checks fail, the identity ones pass" | Directional or orientation-dependent data. That is infrastructure: BLOCKED. |
| "I'll drop that input from the config so the test passes" | Only inputs pzprjs itself rejects may be dropped. |
| "I'll write the common function inline in my genre file" | Common functions go to `encode.rs` / `fileio.rs`, only when your task lists them. |
| "Tests pass, so I can skip clippy / parity / the report" | The coordinator merges on the report. Run everything. |
| "Carrier task: the codecs are in, so I'm done" | Untested codecs. Write the genre; DONE needs `PORTED`. |
| "Only dead_code warnings, those are harmless" | They mean unfinished work. Clippy must print nothing. |

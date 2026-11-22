# AGENTS.md — pzprst

Rust port of pzprjs board, URL, FileIO and pzpr-trace state handling, plus D4
transforms and dedupe keys. Spec: [`docs/SPEC.md`](docs/SPEC.md). Byte parity
with the pinned pzprjs (`../pzprjs`, commit in `variety_table.rs`) is the
acceptance criterion for everything.

## Layout

```text
crates/pzprst/src/        library; genres/<pid>.rs is one genre (auto-registered by build.rs)
crates/pzprst/tests/      standalone crate behavior tests
crates/pzprst-conformance/ repository-only corpus properties and reference parity
crates/pzprst-cli/        `pzprst canon|dedupe|fileio|state` for scripts and parity runs
tools/genres/<pid>.js     fixture inputs per genre
tools/gen_fixtures.js     writes compact fixtures/<pid>.jsonl (never edit fixtures by hand)
tools/check_parity.js     generates temporary outputs and compares with pinned pzprjs-trace
tools/triage.js           what a genre needs; --plan for parallel waves
tools/pzprjs.js           pzprjs output for one input, same arguments as the CLI
tools/parity.js           dataset-wide comparison against pzprjs
```

## Commands

```bash
cargo test -q
cargo clippy --workspace --all-targets -q
node tools/gen_fixtures.js <pid>
node tools/check_parity.js <pid> # requires clean, built pinned pzprjs-trace
node tools/triage.js <pid> | --all | --plan
```

## Tasks

- **Adding a genre:** follow `.agents/skills/pzprst-port-genre/SKILL.md`.
- **Dispatching many genres to agents:** `.agents/skills/pzprst-port-genre/dispatch.md`.
- Core changes (new `Obj` properties, rooms, crosses, excells, directional
  transforms) are maintainer work; update `docs/SPEC.md` with them.
- PuzzleKit consumes this crate directly from its Rust extension; there are no
  bindings in this repo.

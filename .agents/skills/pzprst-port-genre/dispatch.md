# Dispatching genre ports to several agents

For the coordinator (a person or a strong agent). Workers follow
[SKILL.md](SKILL.md); this file is about planning, isolation and merging.

## Prerequisites

- `../pzprjs` at the pinned commit with `dist/js/pzpr.js` built; Node 24+.
- Rust toolchain; `cargo test -q` passes on the base branch.
- `pzprst` has at least one commit (worktrees branch from it).
- Optional: `../puzzlekit-dataset` for parity runs.

## 1. Plan a wave

```bash
node tools/triage.js --plan --size 8                 # all genres
node tools/triage.js --plan --size 8 --puzzlekit-only  # genres PuzzleKit already exposes
```

The plan has three parts:

- **Genre tasks**: `READY` genres. They touch only their own three files, so
  any number can run in parallel.
- **Codec carrier tasks**: `NEEDS_CODEC` genres chosen so that no two carriers
  port the same common function. Each also adds its listed functions to
  `encode.rs` / `fileio.rs`. `[then unlocks N more]` is how many other genres
  become `READY` once it merges.
- **Blocked on infrastructure**: features missing from the core (rooms,
  crosses, excells, directional transforms, array properties). These are not
  worker tasks; give them to a maintainer or strong agent, one feature at a time.

Waves are cheap: merge, then re-plan. Carriers of wave *n* turn dozens of genres
into genre tasks for wave *n+1*.

## 2. Isolate each task

One branch and one worktree per task, as a **sibling** of `pzprst/` so that
`../pzprjs` and `../puzzlekit-dataset` still resolve:

```bash
cd pzprst
git worktree add ../pzprst-<pid> -b genre/<pid>
```

Each worktree builds into its own `target/`; do not share `CARGO_TARGET_DIR`
between worktrees (the build locks and recompiles every time). If a worktree
must live elsewhere, export `PZPRJS=/abs/path/to/pzprjs` for the tools.

## 3. Task prompt

Send each worker exactly this, filled in:

```text
You are porting one puzzle genre in the pzprst Rust library.
Working directory: <abs path>/pzprst-<pid>
First read .agents/skills/pzprst-port-genre/SKILL.md completely and follow it
step by step, including its hard rules and report template. Read
.agents/skills/pzprst-port-genre/reference.md when a step points to it.

Task: genre task for pid "<pid>"
   or: codec carrier for pid "<pid>"; ports: <comma-separated names from the plan>
Do not commit. When done, reply only with the filled-in report.
```

## 4. Review a returned branch

Run in the worktree; each line must pass before merging.

```bash
git status --short          # only the allowed files (see SKILL.md hard rule 1)
node tools/gen_fixtures.js <pid> && git diff --exit-code fixtures/   # fixtures are generated, not hand-edited
cargo fmt --all --check
cargo test -q
cargo clippy --workspace --all-targets -q
node tools/triage.js <pid>  # PORTED
```

Then read the genre file against the pzprjs source for 2 minutes: same decoder
order, no invented branches, `validate` matches `maxnum`/`minnum`. Read every
new common function line by line against pzprjs; these are shared by many genres
later, so they deserve the closest look. Also check that `git diff` adds no
`#[allow`, `#[ignore` or deleted assertions.

If the report says BLOCKED, check the evidence, add the genre to the
infrastructure backlog, and delete the worktree.

## 5. Merge

1. Commit each accepted worktree on its branch (`git add -A && git commit`).
2. Merge carriers first, then genre tasks. Conflicts can only be in
   `encode.rs` / `fileio.rs` and are append-only: keep both sides.
3. After the wave: `cargo test -q`, `cargo clippy …`, optionally
   `node tools/gen_fixtures.js --full <pids…>` and `cargo test -q` again for the
   larger random sets, and `node tools/parity.js --bin <pzprst> <pids…>`.
4. `git worktree remove ../pzprst-<pid>` for each merged task.
5. `node tools/triage.js --plan` for the next wave.

## Escalation

Re-assign a task to a stronger agent when a worker reports BLOCKED for a reason
that is not infrastructure (for example "mismatch after 3 attempts"), or when the
review finds rule violations. Infrastructure items go to the maintainer as
separate tasks with the triage `--all` list of genres they unblock.

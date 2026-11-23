//! Properties over committed inputs, plus opt-in parity with pinned pzprjs-trace.
//! Run `node tools/check_parity.js [pid...]` to generate and compare reference
//! outputs. `PZPRST_GENRE=<pid>` limits local property tests to one genre.

use std::path::{Path, PathBuf};

use pzprst::{supported, Board, Options, Transform, PZPRJS_COMMIT};
use serde_json::Value;

struct Record {
    id: String,
    kind: String,
    input: String,
    value: Value,
}

fn load(pid: &str) -> Vec<Record> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
    let file = format!("{pid}.jsonl");
    let committed = dir.join(&file);
    assert!(
        committed.exists(),
        "{pid} is registered but has no fixtures; add tools/genres/{pid}.js and run node tools/gen_fixtures.js {pid}"
    );
    let mut records = read(&committed);
    let full = dir.join("full").join(&file);
    if full.exists() {
        records.extend(read(&full));
    }
    records
}

fn read(path: &Path) -> Vec<Record> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut lines = text.lines();
    let header: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
    assert_eq!(
        header["header"]["pzprjs_commit"], PZPRJS_COMMIT,
        "fixtures and variety table use different pzprjs commits"
    );
    lines
        .map(|l| {
            let v: Value = serde_json::from_str(l).unwrap();
            Record {
                id: v["id"].as_str().unwrap().into(),
                kind: v["kind"].as_str().unwrap().into(),
                input: v["input"].as_str().unwrap().into(),
                value: v,
            }
        })
        .collect()
}

fn open(r: &Record) -> Board {
    let opts = Options::default();
    let b = if r.kind == "url" {
        Board::from_url(&r.input, &opts)
    } else {
        Board::from_fileio(&r.input, &opts)
    };
    b.unwrap_or_else(|e| panic!("{}: {:?} failed: {e}", r.id, r.input))
}

fn parity(pid: &str) {
    let dir = std::env::var_os("PZPRST_REFERENCE_DIR")
        .map(PathBuf::from)
        .expect("run `node tools/check_parity.js` to generate pzprjs-trace reference outputs");
    let file = format!("{pid}.jsonl");
    let mut records = read(&dir.join(&file));
    let full = dir.join("full").join(&file);
    if full.exists() {
        records.extend(read(&full));
    }
    assert!(!records.is_empty(), "{pid}: reference contains no records");
    let mut failures = Vec::new();
    let mut check = |id: &str, what: String, got: String, want: &Value| {
        if want.as_str() != Some(got.as_str()) {
            failures.push(format!("{id} {what}:\n  got  {got:?}\n  want {want}"));
        }
    };
    for r in &records {
        assert!(
            r.value.get("error").is_none(),
            "{}: pzprjs error fixtures not expected",
            r.id
        );
        let b = open(r);
        check(
            &r.id,
            "canonical".into(),
            b.to_url().unwrap(),
            &r.value["canonical"],
        );
        check(
            &r.id,
            "fileio".into(),
            b.to_fileio().unwrap(),
            &r.value["fileio"],
        );
        let reference_state = r.value["state"].as_str().expect("reference state");
        let imported = Board::from_state(reference_state, &Options::default()).unwrap();
        check(
            &r.id,
            "reference state".into(),
            imported.to_fileio().unwrap(),
            &r.value["fileio"],
        );
        for t in Transform::ALL {
            let want = &r.value["transforms"][t.name()];
            let tb = b.transform(t);
            check(
                &r.id,
                format!("{t} url"),
                tb.to_url().unwrap(),
                &want["url"],
            );
            check(
                &r.id,
                format!("{t} fileio"),
                tb.to_fileio().unwrap(),
                &want["fileio"],
            );
            let imported = Board::from_state(
                want["state"].as_str().expect("reference state"),
                &Options::default(),
            )
            .unwrap();
            check(
                &r.id,
                format!("{t} reference state"),
                imported.to_fileio().unwrap(),
                &want["fileio"],
            );
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} records mismatch pzprjs:\n{}",
        failures.len(),
        records.len(),
        failures
            .iter()
            .take(10)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn genres() -> Vec<&'static str> {
    let only = std::env::var("PZPRST_GENRE").ok();
    let available = supported();
    if let Some(only) = only {
        let names: Vec<_> = only.split(',').collect();
        for name in &names {
            assert!(
                available.contains(name),
                "unknown or unsupported test genre: {name}"
            );
        }
        available
            .into_iter()
            .filter(|pid| names.contains(pid))
            .collect()
    } else {
        available
    }
}

#[test]
#[ignore = "requires pinned pzprjs-trace; run node tools/check_parity.js"]
fn every_genre_matches_pzprjs() {
    for pid in genres() {
        parity(pid);
    }
}

fn properties(pid: &str) {
    let opts = Options::default();
    for r in load(pid) {
        let b = open(&r);
        // FileIO and state round-trip the whole board.
        assert_eq!(
            Board::from_fileio(&b.to_fileio().unwrap(), &opts).unwrap(),
            b,
            "{} fileio",
            r.id
        );
        assert_eq!(
            Board::from_state(&b.to_state().unwrap(), &opts).unwrap(),
            b,
            "{} state",
            r.id
        );
        // Canonical URLs are fixed points.
        let canon = b.to_url().unwrap();
        assert_eq!(
            Board::from_url(&canon, &opts).unwrap().to_url().unwrap(),
            canon,
            "{} idempotent",
            r.id
        );
        assert!(
            Board::from_url(
                &canon,
                &Options {
                    strict: true,
                    ..opts.clone()
                }
            )
            .is_ok(),
            "{} canonical is strict-clean",
            r.id
        );
        // Dedupe key is invariant and its transform reaches the representative.
        let key = b.dedupe_key().unwrap();
        assert_eq!(
            b.transform(key.transform).to_url().unwrap(),
            key.representative,
            "{} representative",
            r.id
        );
        for t in Transform::ALL {
            let tb = b.transform(t);
            assert_eq!(
                tb.dedupe_key().unwrap().key,
                key.key,
                "{} dedupe under {t}",
                r.id
            );
            assert_eq!(tb.transform(t.inverse()), b, "{} inverse of {t}", r.id);
            for u in Transform::ALL {
                assert_eq!(
                    tb.transform(u),
                    b.transform(t.then(u)),
                    "{} compose {t} {u}",
                    r.id
                );
            }
        }
    }
}

#[test]
fn every_genre_satisfies_properties() {
    for pid in genres() {
        properties(pid);
    }
}

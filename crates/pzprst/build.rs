//! Registers every `src/genres/<pid>.rs` (except `mod.rs`), so adding a genre touches no shared file.
//! Each genre file must define `pub(crate) static GENRE: &dyn Genre`.

use std::fmt::Write as _;
use std::path::Path;
use std::{env, fs};

fn main() {
    let dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/genres");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut pids: Vec<String> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| {
            let name = e.unwrap().file_name().into_string().unwrap();
            let pid = name.strip_suffix(".rs")?.to_string();
            (pid != "mod").then_some(pid)
        })
        .collect();
    pids.sort();
    let mut out = String::new();
    for pid in &pids {
        assert!(
            pid.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
            "genre file name must be a pzprjs pid: {pid}.rs"
        );
        let path = dir.join(format!("{pid}.rs"));
        let module = pid.replace('-', "_");
        writeln!(
            out,
            "#[path = {:?}]\nmod g_{module};",
            path.display().to_string()
        )
        .unwrap();
    }
    let list: Vec<String> = pids
        .iter()
        .map(|p| format!("g_{}::GENRE", p.replace('-', "_")))
        .collect();
    writeln!(
        out,
        "static GENRES: &[&dyn Genre] = &[{}];",
        list.join(", ")
    )
    .unwrap();
    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("genres.rs"),
        out,
    )
    .unwrap();
}

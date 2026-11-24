#!/usr/bin/env node
// Check Rust state exports with the pinned pzprjs-trace state implementation.
// Usage: node tools/check_state_parity.js [--pzprjs PATH] <pid...>
"use strict";
const fs = require("fs");
const path = require("path");
const { execFileSync, spawnSync } = require("child_process");
const root = path.resolve(__dirname, "..");
const args = process.argv.slice(2);
const i = args.indexOf("--pzprjs");
const reference = path.resolve(i < 0 ? (process.env.PZPRJS || path.join(root, "../pzprjs")) : args[i + 1]);
if (i >= 0) args.splice(i, 2);
if (!args.length) throw Error("supply at least one pid");
const table = fs.readFileSync(path.join(root, "crates/pzprst/src/variety_table.rs"), "utf8");
const pinned = table.match(/PZPRJS_COMMIT:\s*&str\s*=\s*"([0-9a-f]+)"/)[1];
const commit = execFileSync("git", ["rev-parse", "HEAD"], { cwd: reference, encoding: "utf8" }).trim();
const changed = execFileSync("git", ["status", "--porcelain", "--untracked-files=no", "--", "src", "src-ui"], { cwd: reference, encoding: "utf8" });
if (commit !== pinned || changed.trim()) throw Error("reference must be clean and at " + pinned);
const pzpr = require(path.join(reference, "dist/js/pzpr.js"));
const state = require(path.join(reference, "src-ui/state.js"));
execFileSync("cargo", ["build", "-q", "--release", "-p", "pzprst-cli"], { cwd: root, stdio: "inherit" });
const metadata = JSON.parse(execFileSync("cargo", ["metadata", "-q", "--format-version", "1"], { cwd: root, encoding: "utf8" }));
const bin = path.join(metadata.target_directory, "release", "pzprst");
const transforms = { identity: [], rot90: ["turnr"], rot180: ["turnr", "turnr"], rot270: ["turnl"], flipx: ["flipx"], flipy: ["flipy"], transpose: ["turnl", "flipy"], antitranspose: ["turnr", "flipy"] };

(async () => {
	let checks = 0, rejected = 0;
	for (const pid of args) {
		const records = fs.readFileSync(path.join(root, "fixtures", pid + ".jsonl"), "utf8").trim().split("\n").slice(1).map(JSON.parse).filter(r => r.kind === "fileio").slice(-4);
		if (!records.length) throw Error(pid + ": no FileIO records");
		for (const { input } of records) for (const [name, ops] of Object.entries(transforms)) {
			const puzzle = new pzpr.Puzzle();
			puzzle.open(input);
			let referenceError = null;
			try { ops.forEach(op => puzzle.board.operate(op)); }
			catch (error) { referenceError = String((error && error.message) || error); }
			if (referenceError !== null) {
				const result = spawnSync(bin, ["--transform", name, "state", input], { encoding: "utf8" });
				if (result.error || result.status === 0 || !result.stderr.includes(referenceError)) {
					throw Error(pid + " " + name + ": expected reference transform error " + referenceError);
				}
				checks++; rejected++;
				continue;
			}
			const expected = state.exportText(puzzle);
			const encoded = execFileSync(bin, ["--transform", name, "state", input], { encoding: "utf8" }).trim();
			const decoded = await state.decode(encoded, false);
			if (decoded !== expected) throw Error(pid + " " + name + ": state text differs");
			const target = new pzpr.Puzzle();
			target.open(expected);
			state.target(target, decoded);
			checks++;
		}
		console.log(pid + ": " + records.length + " states × 8 transforms checked");
	}
	console.log(checks + " state checks passed (" + rejected + " reference transform errors matched)");
})().catch(error => { console.error(error); process.exit(1); });

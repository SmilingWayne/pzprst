#!/usr/bin/env node
// Compare every committed input with a freshly built, pinned pzprjs-trace.
// Usage: node tools/check_parity.js [--pzprjs PATH] [--full] [pid...]
"use strict";

const fs = require("fs");
const os = require("os");
const path = require("path");
const { spawnSync } = require("child_process");

const root = path.resolve(__dirname, "..");
const args = process.argv.slice(2);
const generatorArgs = [];
const pids = [];
for (let i = 0; i < args.length; i++) {
	if (args[i] === "--pzprjs") {
		if (!args[i + 1]) throw Error("--pzprjs needs a path");
		generatorArgs.push(args[i], path.resolve(args[++i]));
	} else if (args[i] === "--full") {
		generatorArgs.push(args[i]);
	} else if (args[i].startsWith("-")) {
		throw Error(`unknown option: ${args[i]}`);
	} else {
		pids.push(args[i]);
	}
}

const temp = fs.mkdtempSync(path.join(os.tmpdir(), "pzprst-parity-"));
function run(command, args, env) {
	const result = spawnSync(command, args, { cwd: root, env, stdio: "inherit" });
	if (result.error) throw result.error;
	if (result.status !== 0) throw Error(`${command} failed (${result.status})`);
}

try {
	run(process.execPath, [path.join(__dirname, "gen_fixtures.js"), "--reference", "--output", temp, ...generatorArgs, ...pids], process.env);
	const env = { ...process.env, PZPRST_REFERENCE_DIR: temp };
	// The generator checks the reference checkout against this pin before
	// creating the temporary results. Pass it in memory, not in every JSONL.
	const table = fs.readFileSync(path.join(root, "crates/pzprst/src/variety_table.rs"), "utf8");
	const pin = table.match(/PZPRJS_COMMIT:\s*&str\s*=\s*"([0-9a-f]+)"/);
	if (!pin) throw Error("missing pzprjs-trace pin in variety_table.rs");
	env.PZPRST_REFERENCE_COMMIT = pin[1];
	if (pids.length) env.PZPRST_GENRE = pids.join(",");
	else delete env.PZPRST_GENRE;
	run("cargo", ["test", "-q", "-p", "pzprst-conformance", "--test", "fixtures", "every_genre_matches_pzprjs", "--", "--ignored"], env);
} finally {
	fs.rmSync(temp, { recursive: true, force: true });
}

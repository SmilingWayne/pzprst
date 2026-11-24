#!/usr/bin/env node
// What does porting a pzprjs genre to pzprst involve?
//
//   node tools/triage.js <pid>           detailed report for one genre
//   node tools/triage.js --all [--tsv]   one line per pzprjs genre
//   node tools/triage.js --plan [--size N] [--puzzlekit-only]
//                                        next wave of independent tasks: every READY genre,
//                                        plus up to N codec carriers whose missing codecs
//                                        do not overlap (so parallel branches never port the
//                                        same function twice)
//
// It loads the pinned pzprjs build, reads the genre's Encode (URL) and FileIO methods,
// follows calls into genre-specific helpers, and checks which pzprjs functions already
// exist in pzprst. A Rust function counts as ported when its doc comment starts with the
// pzprjs name in backticks, e.g. "/// `decodeNumber16`: ...".
//
// Verdicts:
//   PORTED       already registered in pzprst
//   READY        every common codec exists; write src/genres/<pid>.rs
//   NEEDS_CODEC  common pzprjs codecs are missing; port them first (codec task)
//   NEEDS_INFRA  needs board features pzprst lacks (crosses, excells, rooms, banks,
//                directional transforms, ...); for the maintainer, not a genre task
"use strict";

const fs = require("fs");
const path = require("path");

const ROOT = path.resolve(__dirname, "..");
const args = process.argv.slice(2);
function arg(name, dflt) {
	const i = args.indexOf(name);
	return i >= 0 ? args[i + 1] : dflt;
}
const PZPRJS = path.resolve(arg("--pzprjs", process.env.PZPRJS || path.join(ROOT, "../pzprjs")));
const PUZZLEKIT = path.resolve(arg("--puzzlekit", path.join(ROOT, "../puzzlekit")));
const pzpr = require(path.join(PZPRJS, "dist/js/pzpr.js"));

// Board features pzprst does not model yet. Codec names matching these need infrastructure.
const INFRA_PATTERNS = [
	[/Cross/, "cross objects (vertices)"],
	[/Excell|Exnum/i, "excell objects (outside clues)"],
	[/Room|Area/, "room/area computation"],
	[/Bank|Piece/, "piece bank"],
	[/Indicator|Count/, "board indicator"],
	[/Arrow|Dir/, "directional values (transform hooks)"],
	[/Segment/, "segments"],
	[/Snum|Qnums/, "array properties (snum/qnums) missing from Obj"]
];

function rustSources() {
	const out = [];
	const walk = (dir) => {
		for (const f of fs.readdirSync(dir)) {
			const p = path.join(dir, f);
			if (fs.statSync(p).isDirectory()) {
				walk(p);
			} else if (f.endsWith(".rs")) {
				out.push(fs.readFileSync(p, "utf8"));
			}
		}
	};
	walk(path.join(ROOT, "crates/pzprst/src"));
	return out.join("\n");
}
const RUST = rustSources();
const PORTED = new Set([...RUST.matchAll(/^\s*\/\/\/ `(\w+)`/gm)].map((m) => m[1]));
// build.rs registers genre files, including hyphenated pids and modules that
// share a parameterized Genre implementation. This reports registration only;
// it does not establish that conformance tests passed.
const REGISTERED = new Set(fs.readdirSync(path.join(ROOT, "crates/pzprst/src/genres"))
	.filter((name) => name.endsWith(".rs") && name !== "mod.rs")
	.map((name) => name.slice(0, -3)));

function puzzlekitPids() {
	const dir = path.join(PUZZLEKIT, "src/puzzlekit/puzzles");
	if (!fs.existsSync(dir)) {
		return new Set();
	}
	const out = new Set();
	for (const f of fs.readdirSync(dir).filter((f) => f.endsWith(".py"))) {
		const m = fs.readFileSync(path.join(dir, f), "utf8").match(/Adapter\('(\w+)'/);
		if (m) {
			out.add(m[1]);
		}
	}
	return out;
}

// A method is genre-specific when it differs from pzpr.common's (variety-common/ and puzzle/).
function overridden(klass, proto, name) {
	return proto[name] !== pzpr.common[klass].prototype[name];
}

const CALL = /\.((?:decode|encode|generic|read|write|out|check)\w*)\(/g;

// Follow calls from the entry methods; genre-specific helpers are expanded, common ones are leaves.
function callGraph(klass, proto, entries) {
	const own = new Set();
	const common = new Set();
	const bodies = {};
	const queue = entries.slice();
	const seen = new Set();
	while (queue.length) {
		const name = queue.shift();
		if (seen.has(name)) {
			continue;
		}
		seen.add(name);
		const fn = proto[name];
		if (typeof fn !== "function") {
			continue;
		}
		if (overridden(klass, proto, name)) {
			own.add(name);
			bodies[name] = fn.toString().replace(/\s+/g, " ");
			for (const m of bodies[name].matchAll(CALL)) {
				queue.push(m[1]);
			}
		} else {
			common.add(name);
		}
	}
	entries.forEach((e) => own.delete(e));
	return { own: [...own], common: [...common], bodies };
}

function sourceFile(pid) {
	const info = pzpr.variety(pid);
	const file = (info && info.script) || pid;
	return path.join(PZPRJS, "src/variety", file + ".js");
}

function triage(pid) {
	const p = new pzpr.Puzzle();
	p.open(pid + "/4/4");
	const b = p.board;
	const enc = callGraph("Encode", p.klass.Encode.prototype, ["decodePzpr", "encodePzpr"]);
	const fio = callGraph("FileIO", p.klass.FileIO.prototype, ["decodeData", "encodeData"]);
	const ignore = new Set(["checkpflag", "decodeConfigFlag", "encodeConfigFlag", "outpflag", "readLine", "getItemList"]);
	const common = [...new Set(enc.common.concat(fio.common))].filter((n) => !ignore.has(n)).sort();
	const missing = common.filter((n) => !PORTED.has(n));
	const infra = [];
	if (b.hasexcell) {
		infra.push("hasexcell=" + b.hasexcell + " (excell objects)");
	}
	for (const name of ["adjustBoardData", "adjustBoardData2"]) {
		if (overridden("BoardExec", p.klass.BoardExec.prototype, name)) {
			infra.push(name + " override (directional content under transforms)");
		}
	}
	for (const name of common.concat(enc.own, fio.own)) {
		for (const [re, what] of INFRA_PATTERNS) {
			if (re.test(name)) {
				infra.push(name + " → " + what);
			}
		}
	}
	if (b.bank) {
		infra.push("board.bank (piece bank)");
	}
	let verdict;
	if (REGISTERED.has(pid)) {
		verdict = "PORTED";
	} else if (infra.length) {
		verdict = "NEEDS_INFRA";
	} else if (missing.length) {
		verdict = "NEEDS_CODEC";
	} else {
		verdict = "READY";
	}
	const cell = b.cell[0];
	let cellRange = null;
	try { if (cell) cellRange = [cell.getminnum(), cell.getmaxnum()]; }
	catch (_) { /* Some reference cells require an initialized room graph. */ }
	return {
		pid,
		verdict,
		source: path.relative(ROOT, sourceFile(pid)),
		board: { hasborder: b.hasborder, hascross: b.hascross, hasexcell: b.hasexcell },
		cellRange,
		decodePzpr: p.klass.Encode.prototype.decodePzpr.toString().replace(/\s+/g, " "),
		encodePzpr: p.klass.Encode.prototype.encodePzpr.toString().replace(/\s+/g, " "),
		decodeData: p.klass.FileIO.prototype.decodeData.toString().replace(/\s+/g, " "),
		encodeData: p.klass.FileIO.prototype.encodeData.toString().replace(/\s+/g, " "),
		common,
		missing,
		genreHelpers: { Encode: enc.own, FileIO: fio.own },
		infra: [...new Set(infra)]
	};
}

function report(t) {
	const lines = [
		"pid:         " + t.pid,
		"verdict:     " + t.verdict,
		...(t.verdict === "PORTED" ? ["registration only; conformance must be checked separately"] : []),
		"source:      " + t.source + "  (look for sections Encode / FileIO / Board / Cell, incl. \"…@" + t.pid + "\")",
		"board:       hasborder=" + t.board.hasborder + " hascross=" + t.board.hascross + " hasexcell=" + t.board.hasexcell,
		"cell range:  " + (t.cellRange ? "minnum " + t.cellRange[0] + ", maxnum " + t.cellRange[1] + " (on a 4x4 board)" : "-"),
		"",
		"Encode.decodePzpr: " + t.decodePzpr,
		"Encode.encodePzpr: " + t.encodePzpr,
		"FileIO.decodeData: " + t.decodeData,
		"FileIO.encodeData: " + t.encodeData,
		"",
		"common pzprjs functions used:"
	];
	t.common.forEach((n) => lines.push("  " + (PORTED.has(n) ? "[ported]  " : "[MISSING] ") + n));
	lines.push("genre-specific helpers to port inside src/genres/" + t.pid + ".rs:");
	lines.push("  Encode: " + (t.genreHelpers.Encode.join(", ") || "-"));
	lines.push("  FileIO: " + (t.genreHelpers.FileIO.join(", ") || "-"));
	lines.push("infrastructure needed: " + (t.infra.length ? "" : "none"));
	t.infra.forEach((n) => lines.push("  " + n));
	return lines.join("\n");
}

function triageAll() {
	const out = [];
	for (const pid of pzpr.variety.getList()) {
		try {
			out.push(triage(pid));
		} catch (e) {
			out.push({ pid, verdict: "ERROR", missing: [], infra: [String(e.message || e)] });
		}
	}
	return out;
}

if (args.includes("--plan")) {
	const size = Number(arg("--size", 8));
	const pk = puzzlekitPids();
	const all = triageAll().filter((t) => !args.includes("--puzzlekit-only") || pk.has(t.pid));
	const tag = (pid) => (pk.has(pid) ? " (puzzlekit)" : "");
	const ready = all.filter((t) => t.verdict === "READY");
	const codec = all.filter((t) => t.verdict === "NEEDS_CODEC");
	const unlocks = (t) => codec.filter((u) => u.missing.every((c) => t.missing.includes(c))).length;
	codec.sort((a, b) => unlocks(b) - unlocks(a) || (pk.has(b.pid) ? 1 : 0) - (pk.has(a.pid) ? 1 : 0) || a.missing.length - b.missing.length || a.pid.localeCompare(b.pid));
	const taken = new Set();
	const carriers = [];
	for (const t of codec) {
		if (carriers.length >= size) {
			break;
		}
		if (t.missing.some((c) => taken.has(c))) {
			continue;
		}
		t.missing.forEach((c) => taken.add(c));
		carriers.push(t);
	}
	console.log("Wave plan (re-run after merging each wave).\n");
	console.log("Genre tasks, no shared files (" + ready.length + "):");
	ready.forEach((t) => console.log("  " + t.pid + tag(t.pid)));
	console.log("\nCodec carrier tasks; each also ports the listed common functions (" + carriers.length + "):");
	carriers.forEach((t) => console.log("  " + t.pid + tag(t.pid) + "  ports: " + t.missing.join(", ") + "  [then unlocks " + (unlocks(t) - 1) + " more]"));
	const infra = {};
	all.filter((t) => t.verdict === "NEEDS_INFRA").forEach((t) =>
		new Set(t.infra.map((s) => s.replace(/^.* → /, "").replace(/=\d+/, ""))).forEach((k) => (infra[k] = (infra[k] || 0) + 1))
	);
	console.log("\nBlocked on infrastructure (maintainer tasks), genres per feature:");
	Object.entries(infra).sort((a, b) => b[1] - a[1]).forEach(([k, n]) => console.log("  " + String(n).padStart(3) + "  " + k));
} else if (args.includes("--all")) {
	const pk = puzzlekitPids();
	const rows = triageAll().map((t) => [t.pid, t.verdict, pk.has(t.pid) ? "puzzlekit" : "", t.missing.join(" "), t.infra.join("; ")]);
	const order = { PORTED: 0, READY: 1, NEEDS_CODEC: 2, NEEDS_INFRA: 3, ERROR: 4 };
	rows.sort((a, b) => order[a[1]] - order[b[1]] || (b[2] ? 1 : 0) - (a[2] ? 1 : 0) || a[0].localeCompare(b[0]));
	if (args.includes("--tsv")) {
		console.log(["pid", "verdict", "in_puzzlekit", "missing_codecs", "infra"].join("\t"));
		rows.forEach((r) => console.log(r.join("\t")));
	} else {
		const counts = {};
		rows.forEach((r) => (counts[r[1]] = (counts[r[1]] || 0) + 1));
		console.log(JSON.stringify(counts));
		rows.forEach((r) => console.log(r[1].padEnd(12) + r[0].padEnd(16) + r[2].padEnd(10) + r[3] + (r[4] ? "  | " + r[4] : "")));
		const need = {};
		rows.filter((r) => r[1] === "NEEDS_CODEC").forEach((r) => r[3].split(" ").forEach((c) => (need[c] = (need[c] || 0) + 1)));
		console.log("\nmissing codecs among NEEDS_CODEC genres (count):");
		Object.entries(need).sort((a, b) => b[1] - a[1]).forEach(([c, n]) => console.log("  " + String(n).padStart(3) + "  " + c));
	}
} else {
	const pid = args.find((a, i) => !a.startsWith("--") && !["--pzprjs", "--puzzlekit"].includes(args[i - 1]));
	if (!pid) {
		console.error("usage: node tools/triage.js <pid> | --all [--tsv]");
		process.exit(2);
	}
	const resolved = pzpr.variety.toPID ? pzpr.variety.toPID(pid) : pid;
	if (!resolved) {
		console.error("unknown pzprjs pid " + JSON.stringify(pid));
		process.exit(2);
	}
	console.log(report(triage(resolved)));
}

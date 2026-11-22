#!/usr/bin/env node
// Generate golden fixtures from the pinned pzprjs build.
//
//   node tools/gen_fixtures.js [--full] [--pzprjs ../pzprjs] [--dataset ../puzzlekit-dataset] [pid...]
//
// Genres come from tools/genres/<pid>.js (all of them when no pid is given):
//   { pid, dataset: "<puzzlekit-dataset folder>" | null,
//     urls: [hand-written URL edge cases], fileios: [hand-written FileIO texts],
//     randomize(puzzle, r) }   // fill a blank board; r.rnd() in [0,1), r.pick(array)
//
// Writes the committed set fixtures/<pid>.jsonl (edge cases + 30 random boards up to 6x5).
// --full also writes fixtures/full/<pid>.jsonl (120 random boards up to 12x9 + 60 dataset
// URLs), which is gitignored; tests use it when present. Each line:
//   {id, kind: "url"|"fileio", input, canonical, fileio, transforms: {name: {url, fileio}}}
// or {id, kind, input, error} when pzprjs throws (the generator exits non-zero then).
"use strict";

const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");

const ROOT = path.resolve(__dirname, "..");
const args = process.argv.slice(2);
function arg(name, dflt) {
	const i = args.indexOf(name);
	return i >= 0 ? args[i + 1] : dflt;
}
const PZPRJS = path.resolve(arg("--pzprjs", process.env.PZPRJS || path.join(ROOT, "../pzprjs")));
const DATASET = path.resolve(arg("--dataset", path.join(ROOT, "../puzzlekit-dataset")));
const FULL = args.includes("--full");
const pzpr = require(path.join(PZPRJS, "dist/js/pzpr.js"));
const COMMIT = execSync("git rev-parse HEAD", { cwd: PZPRJS }).toString().trim();

const TRANSFORMS = {
	identity: [],
	rot90: ["turnr"],
	rot180: ["turnr", "turnr"],
	rot270: ["turnl"],
	flipx: ["flipx"],
	flipy: ["flipy"],
	transpose: ["turnl", "flipy"],
	antitranspose: ["turnr", "flipy"]
};

function body(puzzle) {
	return puzzle.getURL(pzpr.parser.URL_PZPRV3).split("?").slice(1).join("?");
}

function open(input) {
	const p = new pzpr.Puzzle();
	p.open(input);
	return p;
}

function record(id, kind, input) {
	try {
		const p = open(input);
		const fileio = p.getFileData();
		const out = { id, kind, input, canonical: body(p), fileio, transforms: {} };
		for (const [name, ops] of Object.entries(TRANSFORMS)) {
			const q = open(fileio);
			for (const op of ops) {
				q.board.operate(op);
			}
			out.transforms[name] = { url: body(q), fileio: q.getFileData() };
		}
		return out;
	} catch (e) {
		return { id, kind, input, error: String((e && e.message) || e) };
	}
}

function mulberry32(seed) {
	return function() {
		seed |= 0;
		seed = (seed + 0x6d2b79f5) | 0;
		let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
		t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
		return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
	};
}

function datasetUrls(dir, limit) {
	const base = dir && path.join(DATASET, "assets/data", dir);
	if (!base || !fs.existsSync(base)) {
		return [];
	}
	const out = [];
	const files = fs.readdirSync(base).filter((f) => f.endsWith(".json")).sort();
	for (const f of files) {
		const text = fs.readFileSync(path.join(base, f), "utf8");
		const re = /"puzzlink_url":\s*"([^"]*)"/g;
		let m;
		while ((m = re.exec(text)) && out.length < limit) {
			out.push(m[1]);
		}
		if (out.length >= limit) {
			break;
		}
	}
	return out;
}

function write(dir, pid, lines) {
	fs.mkdirSync(dir, { recursive: true });
	const header = { pzprjs_commit: COMMIT, pid, generator: "tools/gen_fixtures.js" };
	const text = [JSON.stringify({ header })].concat(lines.map((l) => JSON.stringify(l))).join("\n") + "\n";
	const file = path.join(dir, pid + ".jsonl");
	fs.writeFileSync(file, text);
	const errors = lines.filter((l) => l.error);
	console.log(path.relative(ROOT, file) + ": " + lines.length + " records (" + errors.length + " pzprjs errors)");
	errors.slice(0, 5).forEach((l) => console.log("  " + l.id + " " + JSON.stringify(l.input) + ": " + l.error));
	return errors.length;
}

const genreDir = path.join(__dirname, "genres");
const wanted = args.filter((a, i) => !a.startsWith("--") && !["--pzprjs", "--dataset"].includes(args[i - 1]));
const pids = wanted.length
	? wanted
	: fs.readdirSync(genreDir).filter((f) => f.endsWith(".js")).map((f) => f.slice(0, -3)).sort();

let failures = 0;
for (const pid of pids) {
	const g = require(path.join(genreDir, pid + ".js"));
	if (g.pid !== pid) {
		throw Error("tools/genres/" + pid + ".js declares pid " + JSON.stringify(g.pid));
	}
	const rnd = mulberry32(pid.length * 7919 + 17);
	const r = { rnd, pick: (arr) => arr[Math.floor(rnd() * arr.length)] };
	let lines = [];
	let n = 0;
	const push = (kind, input) => lines.push(record(pid + "-" + n++, kind, input));
	const pushRandom = (count, maxCols, maxRows) => {
		for (let i = 0; i < count; i++) {
			const cols = 1 + Math.floor(rnd() * maxCols);
			const rows = 1 + Math.floor(rnd() * maxRows);
			const p = open(pid + "/" + cols + "/" + rows);
			g.randomize(p, r);
			const fileio = p.getFileData();
			push("fileio", fileio);
			push("url", body(open(fileio)));
		}
	};
	g.urls.forEach((u) => push("url", u));
	g.fileios.forEach((f) => push("fileio", f));
	pushRandom(30, 6, 5);
	failures += write(path.join(ROOT, "fixtures"), pid, lines);

	if (FULL) {
		lines = [];
		pushRandom(120, 12, 9);
		datasetUrls(g.dataset, 60).forEach((u) => push("url", u));
		failures += write(path.join(ROOT, "fixtures/full"), pid, lines);
	}
}
process.exit(failures ? 1 : 0);

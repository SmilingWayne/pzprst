#!/usr/bin/env node
// Compare pzprst with pinned pzprjs on every puzzlink_url of a dataset genre.
//
//   node tools/parity.js --bin <path/to/pzprst> [--pzprjs ../pzprjs] [--dataset ../puzzlekit-dataset]
//                        [--no-transforms] [pid...]
//
// Genres are tools/genres/<pid>.js files with a `dataset` folder (all of them by default).
"use strict";

const fs = require("fs");
const path = require("path");
const { execFileSync } = require("child_process");

const ROOT = path.resolve(__dirname, "..");
const args = process.argv.slice(2);
function arg(name, dflt) {
	const i = args.indexOf(name);
	return i >= 0 ? args[i + 1] : dflt;
}
const BIN = arg("--bin");
if (!BIN) {
	console.error("--bin <pzprst binary> is required");
	process.exit(2);
}
const PZPRJS = path.resolve(arg("--pzprjs", process.env.PZPRJS || path.join(ROOT, "../pzprjs")));
const DATASET = path.resolve(arg("--dataset", path.join(ROOT, "../puzzlekit-dataset")));
const genreDir = path.join(__dirname, "genres");
const wanted = args.filter((a, i) => !a.startsWith("--") && !["--bin", "--pzprjs", "--dataset"].includes(args[i - 1]));
const GENRES = (wanted.length
	? wanted
	: fs.readdirSync(genreDir).filter((f) => f.endsWith(".js")).map((f) => f.slice(0, -3)).sort()
)
	.map((pid) => require(path.join(genreDir, pid + ".js")).dataset)
	.filter(Boolean);
const TRANSFORMS = args.includes("--no-transforms")
	? {}
	: {
			rot90: ["turnr"],
			rot180: ["turnr", "turnr"],
			rot270: ["turnl"],
			flipx: ["flipx"],
			flipy: ["flipy"],
			transpose: ["turnl", "flipy"],
			antitranspose: ["turnr", "flipy"]
	  };
const pzpr = require(path.join(PZPRJS, "dist/js/pzpr.js"));

function urls(dir) {
	const base = path.join(DATASET, "assets/data", dir);
	const out = [];
	if (!fs.existsSync(base)) {
		console.log(dir + ": no dataset folder at " + base);
		return out;
	}
	for (const f of fs.readdirSync(base).filter((f) => f.endsWith(".json")).sort()) {
		const re = /"puzzlink_url":\s*"([^"]*)"/g;
		const text = fs.readFileSync(path.join(base, f), "utf8");
		let m;
		while ((m = re.exec(text))) {
			out.push(m[1]);
		}
	}
	return out;
}

function pzprjs(url, ops) {
	try {
		const p = new pzpr.Puzzle();
		p.open(url);
		for (const op of ops) {
			p.board.operate(op);
		}
		return p.getURL(pzpr.parser.URL_PZPRV3).split("?").slice(1).join("?");
	} catch (e) {
		return "ERR pzprjs: " + e.message;
	}
}

function pzprst(cmd, inputs) {
	const out = execFileSync(BIN, cmd, { input: inputs.join("\n") + "\n", maxBuffer: 1 << 28 }).toString();
	return out.split("\n").slice(0, inputs.length);
}

let failed = 0;
for (const genre of GENRES) {
	const list = urls(genre);
	if (list.length === 0) {
		console.log(genre + ": no puzzlink_url records; nothing to compare (report parity as: no dataset)");
		continue;
	}
	const checks = [["canonical", ["canon"], []]].concat(
		Object.entries(TRANSFORMS).map(([name, ops]) => [name, ["--transform", name, "canon"], ops])
	);
	let changed = 0;
	for (const [name, cmd, ops] of checks) {
		const ours = pzprst(cmd, list);
		let diff = 0;
		list.forEach((u, i) => {
			const want = pzprjs(u, ops);
			if (ours[i] !== want) {
				if (diff < 5) {
					console.log("  " + genre + " " + name + " " + u + "\n    pzprst " + ours[i] + "\n    pzprjs " + want);
				}
				diff++;
			}
			if (name === "canonical" && want !== u.split("?").slice(1).join("?")) {
				changed++;
			}
		});
		failed += diff;
		console.log(genre + " " + name + ": " + (list.length - diff) + "/" + list.length + " identical");
	}
	console.log(genre + ": " + changed + " source URLs were not already canonical");
}
process.exit(failed ? 1 : 0);

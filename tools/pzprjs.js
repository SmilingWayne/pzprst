#!/usr/bin/env node
// Print what pinned pzprjs produces, with the same arguments as the pzprst CLI:
//
//   node tools/pzprjs.js [--transform NAME] canon|fileio <input>
//
// Compare with: cargo run -q -p pzprst-cli -- [--transform NAME] canon|fileio <input>
"use strict";

const path = require("path");

const ROOT = path.resolve(__dirname, "..");
const args = process.argv.slice(2);
const OPS = {
	identity: [],
	rot90: ["turnr"],
	rot180: ["turnr", "turnr"],
	rot270: ["turnl"],
	flipx: ["flipx"],
	flipy: ["flipy"],
	transpose: ["turnl", "flipy"],
	antitranspose: ["turnr", "flipy"]
};
let transform = "identity";
const i = args.indexOf("--transform");
if (i >= 0) {
	transform = args.splice(i, 2)[1];
}
const [cmd, input] = args;
if (!["canon", "fileio"].includes(cmd) || input === undefined || !OPS[transform]) {
	console.error("usage: node tools/pzprjs.js [--transform NAME] canon|fileio <input>");
	process.exit(2);
}
const pzpr = require(path.join(process.env.PZPRJS || path.join(ROOT, "../pzprjs"), "dist/js/pzpr.js"));
const p = new pzpr.Puzzle();
p.open(input.includes("/") || input.includes("\n") ? input : input.replace(/\\n/g, "\n"));
OPS[transform].forEach((op) => p.board.operate(op));
console.log(
	cmd === "canon"
		? p.getURL(pzpr.parser.URL_PZPRV3).split("?").slice(1).join("?")
		: p.getFileData().replace(/\n$/, "")
);

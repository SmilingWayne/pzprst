// Fixture inputs for pzprjs pid "yajisoko". See tools/gen_fixtures.js for the format.
// "yosenabe" and "brownies" share pzprjs's variety/yosenabe.js but override
// Encode/FileIO, so they are separate pids and are not covered here.
"use strict";

module.exports = {
	pid: "yajisoko",
	dataset: null,
	urls: [
		// One cell of every URL value kind on a 3x3: infinity/-4, ?/-2 and numbers.
		"yajisoko/3/3/012345678",
		"https://puzz.link/p?yajisoko/3/3/012345678",
		"https://puzz.link/p?yajisoko_edit/3/3/012345678",
		"http://pzv.jp/p.html?type=player&yajisoko/3/3/012345678",
		// Unknown flag.
		"yajisoko/x/3/3/012345678",
		// The -5 "no line" binary layer on a 1x1 and a 2x1 board.
		"yajisoko/1/1/gg",
		"yajisoko/2/1/g3g",
		// Every cell a dark cell (`?`).
		"yajisoko/3/3/.........",
		// Clues 3 (the 4x4 maximum) as three-digit escapes.
		"yajisoko/4/4/-19-1a-1b-1c-1d-1e-1f-20-21-22-23-24-25-26-27-28",
		// Base-36 run lengths, 1/N and N/1 boards.
		"yajisoko/1/5/k",
		"yajisoko/5/1/k",
		"yajisoko/4/4/v",
		// Truncated body, trailing junk and empty bodies.
		"yajisoko/3/3/012",
		"yajisoko/3/3/012345678zz",
		"yajisoko/3/3",
		"yajisoko/3/3/",
		// Real puzzle URLs (puzz.link).
		"https://puzz.link/p?yajisoko/5/5/e.g.-12.-10g-11.h-17h.eg-12.-18.g.-10",
		"https://puzz.link/p?yajisoko/9/9/-16g-17.-1c.-21g-12g.k.gch-16.-2ah-15.g-22.g.-17g.-11g.i.g-10.g-13.g.-16g.ch-12.-1ah-1ag.k.g-1dg-22.-18.-18gb",
		"https://puzz.link/p?yajisoko/8/8/cdjec.-16-16-16-1b-1b-1b..-18g-13-12g-12..-18-10-10g-10-17..-18-13h-12-17..-18cgcc-17..-13g-13-12g-17..-10-15-1a-10-15-1a./"
	],
	fileios: [
		// 3x3 with `∞`, `?`, a -5 cell, answer lines and packed qsub/qcmp marks.
		"pzprv3/yajisoko/3/3/1,-4 0,-4 1,-5 /1,-4 1,-4 2,-4 /1,-4 3,-4 1,-4 /1 -1 /0 0 /0 0 /0 0 0 /0 0 0 /1 19 16 /0 0 0 /0 0 0 /",
		// 2x2 with `∞`, `?`, a number and answer marks.
		"pzprv3/yajisoko/2/2/0,-4 # /0,-2 0,0 /0 /0 /0 0 /0 0 /0 1 /",
		// 1x1: no inner borders, so the border layer is empty.
		"pzprv3/yajisoko/1/1/0,-4 //0 "
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows) - 1;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qnum = -1;
			cell.qnum2 = -1;
			cell.qdir = 0;
			if (x < 0.35) {
				// empty
			} else if (x < 0.45) {
				cell.qnum = -2;
			} else if (x < 0.55) {
				cell.qnum2 = -4;
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			} else if (x < 0.65) {
				cell.qnum2 = -2;
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			} else if (x < 0.75) {
				cell.qnum2 = -5;
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			} else {
				cell.qnum2 = Math.floor(r.rnd() * (maxnum + 1));
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			}
			cell.qsub = Math.floor(r.rnd() * 16);
			cell.qcmp = r.rnd() < 0.3 ? 1 : 0;
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

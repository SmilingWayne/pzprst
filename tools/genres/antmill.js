// Fixture inputs for pzprjs pid "antmill". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "antmill",
	dataset: null,
	urls: [
		// Real puzzles, already canonical with pzprjs.
		"antmill/10/10/g2l3j3o16j1k6zj1h6h2m6p6h2g6n",
		"antmill/10/10/u32k8i32i8zl2i2j7j31h6l6u",
		"antmill/12/12/o1k6n2l3h6h2m2z2p2n2m2g3p23h6p12i3u",
		"https://puzz.link/p?antmill/10/10/g2l3j3o16j1k6zj1h6h2m6p6h2g6n",
		"http://pzv.jp/p.html?type=player&antmill/10/10/u32k8i32i8zl2i2j7j31h6l6u",
		// A hand-checked canonical body plus an unknown flag letter.
		"antmill/3/3/3i6i",
		"antmill/x/3/3/3i6i",
		// The largest URL value 8 = right 2 / bottom 2.
		"antmill/3/3/8m",
		"antmill/2/2/826",
		// `?` decodes to right 0 / bottom -2.
		"antmill/3/3/.",
		// Truncated, run lengths, skipped characters, trailing junk.
		"antmill/3/3/3",
		"antmill/3/3/g3",
		"antmill/3/3/A",
		"antmill/3/3/--1",
		"antmill/3/3/3i6izz",
		"antmill/3/3",
		"antmill/3/3/",
		// 1/1, 1/N and N/1 boards.
		"antmill/1/1/0",
		"antmill/1/1/",
		"antmill/1/2/q",
		"antmill/2/1/q"
	],
	fileios: [
		// Question and answer layers with both answer tokens.
		"pzprv3/antmill/2/2/2 /2 /2 2 /# + /. # /",
		"pzprv3/antmill/3/3/1 0 /0 2 /0 0 /0 0 0 /0 0 0 /# . + /. # . /. . # /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qans = x < 0.2 ? 1 : 0;
			cell.qsub = x >= 0.2 && x < 0.35 ? 1 : 0;
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.ques = x < 0.15 ? 1 : x < 0.3 ? 2 : 0;
		});
	}
};

// Fixture inputs for pzprjs pid "lookair". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "lookair",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"lookair/10/10/3l3l4l4b2n1b2l2l2l2",
		"lookair/8/8/3c3w0a2e2s3d3b",
		"lookair/6/6/1m2c2d2b21h",
		"https://puzz.link/p?lookair/8/8/3c3w0a2e2s3d3b",
		"http://pzv.jp/p.html?type=player&lookair/6/6/1m2c2d2b21h",
		"https://puzz.link/p?lookair_edit/6/6/1m2c2d2b21h",
		// One cell of every kind: clue 0, largest clue 5, out-of-range 6 and 9,
		// ? and empty.
		"lookair/1/1/0",
		"lookair/1/1/5",
		"lookair/1/1/6",
		"lookair/1/1/9",
		"lookair/1/1/.",
		// Characters outside the number10 alphabet each consume one cell.
		"lookair/3/3/--1",
		"lookair/3/3/A",
		// Run lengths, truncated bodies, trailing junk, 1/N and N/1 boards.
		"lookair/1/8/z1",
		"lookair/8/1/z1",
		"lookair/3/3/z",
		"lookair/4/4/123",
		"lookair/4/4/123zz",
		"lookair/4/4",
		"lookair/4/4/",
		"lookair/2/2/zz"
	],
	fileios: [
		// Clue layer and shading layer with both answer tokens.
		"pzprv3/lookair/3/3/1 2 3 /. . . /. 0 . /# . # /. . + /. . . /",
		// 2x2 board with the largest clue and an answer in every cell.
		"pzprv3/lookair/2/2/5 0 /1 4 /# + /. # /",
		// Empty clue layer and a single answer mark.
		"pzprv3/lookair/1/1/. /+ /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.45) {
				cell.qnum = -1;
			} else if (x < 0.58) {
				cell.qnum = -2;
			} else {
				cell.qnum = Math.floor(r.rnd() * 6);
			}
			const z = r.rnd();
			cell.qans = z < 0.25 ? 1 : 0;
			cell.qsub = z >= 0.25 && z < 0.4 ? 1 : 0;
		});
	}
};

// Fixture inputs for pzprjs pid "nawabari". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "nawabari",
	dataset: "Nawabari",
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"nawabari/6/6/3d3d3a3b3j3a3b3c3",
		"nawabari/10/10/2c32c3b1d1o1b1d3f1b1f1d2b1o1d2b2c32c2",
		"nawabari/5/6/b2b2b2b2b2b2c2b2e",
		"https://puzz.link/p?nawabari/6/6/3d3d3a3b3j3a3b3c3",
		"http://pzv.jp/p.html?type=player&nawabari/2/2/77b",
		// Shaded cells, a question mark and clue 0..4.
		"nawabari/2/2/77b",
		"nawabari/2/2/1-23",
		"nawabari/2/2/444a",
		"nawabari/1/1/4",
		"nawabari/1/1/./",
		"nawabari/1/4/012",
		"nawabari/4/1/012",
		// Base-36 blank runs and the empty body.
		"nawabari/27/1/a",
		"nawabari/4/4/",
		// Truncated body, junk after the last cell, unknown pflag.
		"nawabari/4/4/2c2g",
		"nawabari/2/2/77ZZ",
		"nawabari/f/4/4/2c2g"
	],
	fileios: [
		// Every cell token and every border answer token.
		"pzprv3/nawabari/2/2/* - /0 4 /1 0 -1 /2 1 0 /0 1 /-1 2 /0 0 /",
		"pzprv3/nawabari/3/2/0 1 /- 4 /. * /1 0 -1 /2 1 0 /0 -1 2 /0 1 /-1 2 /0 0 /1 0 /",
		// Empty board: no clue and no answer data.
		"pzprv3/nawabari/1/2/. . /0 0 0 /0 0 /0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.ques = 0;
			cell.qnum = -1;
			if (x < 0.15) {
				// Shaded cells store nothing else: FileIO writes "*" and reads it back.
				cell.ques = 7;
			} else if (x < 0.3) {
				cell.qnum = -2;
			} else if (x < 0.7) {
				cell.qnum = r.pick([0, 0, 1, 2, 3, 4]);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qans = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.qans = 1;
			} else if (x < 0.3) {
				border.qsub = 1;
			} else if (x < 0.4) {
				border.qans = 1;
				border.qsub = 1;
			}
		});
	}
};

// Fixture inputs for pzprjs pid "fivecells". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "fivecells",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"fivecells/10/10/2c2g3a3a1e3c3e2a23e0f0c22b2j1d1a1f2e23k",
		"fivecells/10/10/a1a22g2f2f1a1a1a22d1m11a2a2a2a2b2c2c22a1a22b2k2c2a2",
		"fivecells/10/10/c2d2c2h3b11e2e0d0b1b2n22c1h2d3i2c2c",
		"https://puzz.link/p?fivecells/2/2/7.b",
		"http://pzv.jp/p.html?type=player&fivecells/4/4/2c2g",
		// A question mark lives in the URL as ".".
		"fivecells/2/2/7.",
		"fivecells/3/3/.3z",
		// Largest clue 3, shaded cells, the two extreme blank runs.
		"fivecells/2/2/33c",
		"fivecells/1/1/7",
		"fivecells/1/1/./",
		"fivecells/1/4/123",
		"fivecells/4/1/123",
		"fivecells/27/1/a",
		// Truncated body, and a body that overruns the board.
		"fivecells/4/4/2c2g",
		"fivecells/4/4/2c2g3a3a1e3c3e2a23e0f0c22b2j1d1a1f2e23k",
		// Junk after the last cell, and an unknown pflag (fivecells has none).
		"fivecells/2/2/7.bZZ",
		"fivecells/f/4/4/2c2g"
	],
	fileios: [
		// Every cell token, and answer marks including the outer frame.
		"pzprv3/fivecells/2/2/* - /2 3 /1 0 -1 /2 1 0 /0 1 /-1 2 /0 0 /",
		// A 0 clue, a shaded cell, a question mark and border crosses.
		"pzprv3/fivecells/3/2/0 1 /. . /3 * /0 0 0 /0 -1 0 /0 0 0 /0 1 0 0 /1 0 -1 0 /0 0 0 0 /",
		// Empty board: no clue and no answer data.
		"pzprv3/fivecells/1/2/. . /0 0 0 /0 0 /0 0 /"
	],
	randomize(p, r) {
		// pzprjs throws while decoding a 2x1 URL: `decodeFivecells` clears the
		// shaded cells `initBoardSize` pre-marks with `setQues(0)`, which rebuilds
		// its area graph and dereferences an undefined node. Keep the random
		// boards representable for pzprjs.
		if (p.board.cols === 2 && p.board.rows === 1) {
			p.board.initBoardSize(3, 1);
		}
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
				cell.qnum = r.pick([0, 0, 1, 2, 3]);
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

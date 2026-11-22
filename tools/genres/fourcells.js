// Fixture inputs for pzprjs pid "fourcells". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "fourcells",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"fourcells/6/6/c3b3a3d3f3a3b3a3a3b3c",
		"fourcells/10/10/d3f1c3b1g3e3a1f2d1b23b1d2f1a3e3g1b2c1f3d",
		"fourcells/10/10/a3d3a3d2e2a1b1m1a3e1d3b3b2e2j3b2b1a2c1a3k",
		"https://puzz.link/p?fourcells/6/6/c3b3a3d3f3a3b3a3a3b3c",
		"http://pzv.jp/p.html?type=player&fourcells/2/2/2-13",
		// A question mark and clue 1..3, plus shaded cells.
		"fourcells/2/2/2-13",
		"fourcells/2/2/33c",
		"fourcells/2/2/77",
		"fourcells/1/1/3",
		"fourcells/1/1/./",
		"fourcells/1/4/123",
		"fourcells/4/1/123",
		// Base-36 blank runs: the 26-cell flush, and the empty body (four cells
		// so `initBoardSize` pre-marks nothing).
		"fourcells/27/1/a",
		"fourcells/2/2/",
		// Truncated body, junk after the last cell, unknown pflag.
		"fourcells/4/4/2c2g",
		"fourcells/2/2/2a13ZZ",
		"fourcells/f/4/4/2c2g"
	],
	fileios: [
		// Every cell token and every border answer token (`pzprv3.1`).
		"pzprv3.1/fourcells/2/2/2 - /1 3 /1 0 -1 /0 1 0 /0 1 /-1 2 /0 0 /",
		"pzprv3.1/fourcells/3/2/1 2 /- 3 /. . /1 0 -1 /2 1 0 /0 -1 2 /0 1 /-1 2 /0 0 /1 0 /",
		// `pzprv3` (file version 0) stores the inner borders only; answers come
		// back on a `pzprv3.1` frame.
		"pzprv3/fourcells/2/2/2 - /1 3 /0 /1 /-1 2 /",
		// A one-column version-0 border layer: pzprjs's walk runs off the row and
		// writes these tokens to the right frame.
		"pzprv3/fourcells/2/1/3 /. /1 /-1 /"
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
				cell.qnum = r.pick([1, 1, 2, 3]);
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

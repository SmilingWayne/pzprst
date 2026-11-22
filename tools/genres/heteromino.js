// Fixture inputs for pzprjs pid "heteromino". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "heteromino",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"heteromino/10/10/k7a7b777b7a7d7b777b777v7a7b777b7a7b7a7b777b7a7k",
		"heteromino/7/8/7b7c7b7a7777a7a77c7j777b77a7b77h",
		"heteromino/8/8/j77777e7g7e7a7e7a7a7d7l",
		"https://puzz.link/p?heteromino/8/8/j77777e7g7e7a7e7a7a7d7l",
		"http://pzv.jp/p.html?type=player&heteromino/2/2/77b",
		// Shaded cells, a question mark and single-digit clues.
		"heteromino/2/2/77b",
		"heteromino/2/2/1-23",
		"heteromino/2/2/999a",
		"heteromino/1/1/7",
		"heteromino/1/1/./",
		"heteromino/1/4/123",
		"heteromino/4/1/123",
		// Base-36 blank runs and the empty body.
		"heteromino/27/1/a",
		"heteromino/4/4/",
		// Truncated body, junk after the last cell, unknown pflag.
		"heteromino/4/4/2c2g",
		"heteromino/2/2/77ZZ",
		"heteromino/f/4/4/2c2g"
	],
	fileios: [
		// Every cell token and every border answer token.
		"pzprv3/heteromino/2/2/* - /7 9 /1 0 -1 /2 1 0 /0 1 /-1 2 /0 0 /",
		"pzprv3/heteromino/3/2/1 2 /- 9 /. * /1 0 -1 /2 1 0 /0 -1 2 /0 1 /-1 2 /0 0 /1 0 /",
		// Empty board: no clue and no answer data.
		"pzprv3/heteromino/1/2/. . /0 0 0 /0 0 /0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.ques = 0;
			cell.qnum = -1;
			if (x < 0.3) {
				// Heteromino boards are mostly shaded "outside" cells.
				cell.ques = 7;
			} else if (x < 0.4) {
				cell.qnum = -2;
			} else if (x < 0.7) {
				// Clues are single decimal digits: the URL codec writes one
				// character per cell, so 10 or more would not round-trip.
				cell.qnum = r.pick([1, 2, 3, 4, 5, 6, 7, 8, 9]);
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

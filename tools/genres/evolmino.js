// Fixture inputs for pzprjs pid "evolmino". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "evolmino",
	dataset: null,
	urls: [
		// Real puzzles, already canonical with pzprjs.
		"evolmino/7/7/c1c03101c03101c19w0n6j80zg6f",
		"evolmino/5/5/o8037i2iim4442z5",
		"evolmino/7/7/q680d9qq2q0d9q02iza666663zze",
		"https://puzz.link/p?evolmino/7/7/c1c03101c03101c19w0n6j80zg6f",
		"http://pzv.jp/p.html?type=player&evolmino/5/5/o8037i2iim4442z5",
		// Cell values 1 (black) and 2 (square) with border arrow runs.
		"evolmino/3/3/b3j031311313",
		"evolmino/3/3/9",
		"evolmino/1/1/i",
		"evolmino/4/4/000",
		// Unknown flag letter, trailing junk, skipped characters.
		"evolmino/x/3/3/9",
		"evolmino/3/3/9zz",
		"evolmino/3/3/!",
		"evolmino/3/3/A",
		// Truncated bodies and 1/1, 1/N and N/1 boards.
		"evolmino/3/3",
		"evolmino/3/3/",
		"evolmino/1/1",
		"evolmino/2/1/v",
		"evolmino/1/2/v"
	],
	fileios: [
		// Cell layer, arrow layer and answer layer (anum and qsub).
		"pzprv3/evolmino/3/3/# . 0 /. # . /0 . # /1 2 /1 2 /1 2 /1 2 0 /0 1 2 /0 . + /. 0 . /. + 0 /",
		// A 2x2 board with both arrow directions and every cell state.
		"pzprv3/evolmino/2/2/# 0 /. . /1 /2 /1 2 /0 + /. 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.2) {
				cell.ques = 1;
				cell.qnum = -1;
			} else if (x < 0.4) {
				cell.ques = 0;
				cell.qnum = 1;
			} else {
				cell.ques = 0;
				cell.qnum = -1;
			}
			const y = r.rnd();
			cell.anum = y < 0.2 ? 1 : -1;
			cell.qsub = y >= 0.2 && y < 0.35 ? 1 : 0;
		});
		p.board.border.each((border) => {
			if (r.rnd() < 0.5) {
				border.qdir = border.isVert() ? r.pick([3, 4]) : r.pick([1, 2]);
			} else {
				border.qdir = 0;
			}
		});
	}
};

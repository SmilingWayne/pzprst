// Fixture inputs for pzprjs pid "lixloop". See tools/gen_fixtures.js for the format.
// lixloop shares variety/yajilin.js with yajilin. Its `b` URL flag (display type)
// is not stored in FileIO, so it is not a fixture input here: the fixture
// transforms rebuild the board from FileIO and pzprjs itself drops the flag.
"use strict";

module.exports = {
	pid: "lixloop",
	dataset: null,
	urls: [
		// Hand-built 3x3: every direction and clues 1-6.
		"lixloop/3/3/111213142526273132",
		"https://puzz.link/p?lixloop/3/3/111213142526273132",
		"https://puzz.link/p?lixloop_edit/3/3/111213142526273132",
		"http://pzv.jp/p.html?type=player&lixloop/3/3/111213142526273132",
		// Rule flag alone, with an unknown flag, and on a 1x1 board.
		"lixloop/o/3/3/111213142526273132",
		"lixloop/xo/3/3/111213142526273132",
		"lixloop/o/1/1/1.",
		// Largest allowed clue (7) on a 1x1 board.
		"lixloop/1/1/17",
		// `?` with a direction and `+` (arrow without number).
		"lixloop/1/1/1.",
		"lixloop/1/1/+",
		// Largest clue on a 4x1 board.
		"lixloop/4/1/11121314",
		// Base-36 run lengths, 1/N and N/1 boards.
		"lixloop/1/5/g",
		"lixloop/5/1/g",
		"lixloop/4/4/v",
		"lixloop/2/2/1112",
		// Three-character escape and a two-digit number.
		"lixloop/1/1/-14",
		"lixloop/1/1/5a",
		"lixloop/1/1/510",
		// Truncated body, trailing junk and empty bodies.
		"lixloop/3/3/1112",
		"lixloop/3/3/111213142526273132zz",
		"lixloop/3/3",
		"lixloop/3/3/",
		"lixloop/4/4/."
	],
	fileios: [
		// 3x3 with clues, `?`, `+`, the config flag, answer marks and lines.
		"pzprv3/lixloop/3/3/o/1,1 1,2 1,3 /1,4 2,5 2,6 /2,7 3,1 3,2 /# + . /. # + /. . # /1 0 /0 -1 /1 1 /0 1 0 /1 0 0 /",
		// 2x2 with `?`, `+` and every answer mark.
		"pzprv3/lixloop/2/2/1,- 2,3 /# 3,- /+ # /. + /1 /0 /0 1 /",
		// 1x1: no inner borders, so the border layer is empty.
		"pzprv3/lixloop/1/1/1,- /./"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qdir = 0;
			if (x < 0.4) {
				cell.qnum = -1;
			} else if (x < 0.5) {
				cell.qnum = -2;
				cell.qdir = 1 + Math.floor(r.rnd() * 4);
			} else if (x < 0.55) {
				cell.qnum = -3;
			} else {
				cell.qnum = 1 + Math.floor(r.rnd() * 7);
				cell.qdir = Math.floor(r.rnd() * 5);
			}
			const a = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (a < 0.2) {
				cell.qans = 1;
			} else if (a < 0.35) {
				cell.qsub = 1;
			}
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
		p.setConfig("yajilin_out", r.rnd() < 0.3);
	}
};

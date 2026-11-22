// Fixture inputs for pzprjs pid "bwloop". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "bwloop",
	dataset: null,
	urls: [
		"bwloop/4/4/f00000",
		"bwloop/4/4/100000",
		"https://puzz.link/p?bwloop/4/4/f00000",
		"http://pzv.jp/p.html?type=player&bwloop/4/4/f00000",
		"bwloop/f/4/4/f00000",
		"bwloop/xf/4/4/f00000",
		"bwloop/x/4/4/f00000",
		"bwloop/4/4/f",
		"bwloop/4/4/100000zz",
		"bwloop/4/4",
		"bwloop/4/4/",
		"bwloop/4/4/qqqqqi",
		"bwloop/2/2/f0",
		"bwloop/1/1/i",
		"bwloop/3/1/i",
		"bwloop/1/3/i",
		"bwloop/25/1/qqqqqqqqi",
		"bwloop/1/25/qqqqqqqqi"
	],
	fileios: [
		"pzprv3/bwloop/4/4/f/1 2 . - /2 1 . . /. . . . /. . . . /0 0 0 /0 0 0 /1 0 0 /0 -1 0 /0 1 0 0 /0 0 0 0 /1 -1 0 0 /",
		"pzprv3/bwloop/2/3/1 . 2 /- 1 . /1 0 /0 0 /1 0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 1, 2, -2]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

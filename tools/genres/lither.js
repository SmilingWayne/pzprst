// Fixture inputs for pzprjs pid "lither". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "lither",
	dataset: null,
	urls: [
		"lither/4/4/8a638d6c",
		"https://puzz.link/p?lither/4/4/8a638d6c",
		"https://puzz.link/p?lither_edit/4/4/8a638d6c",
		"http://pzv.jp/p.html?type=player&lither/4/4/8a638d6c",
		"lither/4/4/8a638d6c/",
		"lither/4/4/8a638d6c//",
		"lither/4/4/8a638d6czz",
		"lither/4/4/8a6",
		"lither/4/4",
		"lither/4/4/",
		"lither/xf/4/4/8a638d6c",
		"lither/f/4/4/8a638d6c",
		"lither/1/1/3",
		"lither/1/1/4",
		"lither/1/1/.",
		"lither/1/5/01234",
		"lither/5/1/01234",
		"lither/2/2/./.",
		"lither/6/6/v",
		"lither/3/3/zzzz",
		"lither/12/12/zzzzzzzzzzzzzzzzzzzzzzzz"
	],
	fileios: [
		"pzprv3.1/lither/4/4/3 . 0 . /. 1 . 3 /3 . 3 . /. 1 . 2 /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /-1 1 -1 -1 -1 /-1 -1 -1 1 1 /1 1 -1 1 1 /1 1 -1 1 -1 /1 1 -1 1 /1 1 -1 -1 /-1 -1 1 1 /1 -1 1 -1 /-1 -1 1 1 /",
		"pzprv3/lither/4/4/3 . 0 . /. 1 . 3 /3 . 3 . /. 1 . 2 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /",
		"pzprv3.1/lither/2/3/2 . 0 /. 1 - /1 2 0 /0 1 0 /0 1 1 0 /0 1 0 0 /0 0 0 /0 0 0 /0 0 0 /",
		"pzprv3.1/lither/1/1/3 /2 /0 1 /0 /0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 0, 1, 2, 3, -2]);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

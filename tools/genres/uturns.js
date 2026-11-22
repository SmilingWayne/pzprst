// Fixture inputs for pzprjs pid "uturns". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "uturns",
	dataset: null,
	urls: [
		"uturns/9/9/..3.1..2..1.3..2..1..3..2..1..3..2..1..3..2..1..3..2..1..3..2..1..3..2..1..3..2..1",
		"https://puzz.link/p?uturns/9/9/..3.1..2..1.3..2..1..3..2..1..3..2..1..3..2..1..3..2..1..3..2..1..3..2..1..3..2..1",
		"uturns/4/4/1..3.2..1.3..2..1..3..2..1",
		"https://puzz.link/p?uturns/4/4/1..3.2..1.3..2..1..3..2..1",
		"http://pzv.jp/p.html?type=player&uturns/4/4/1..3.2..1.3..2..1..3..2..1",
		"uturns/3/3/123456789",
		"uturns/3/3/.....",
		"uturns/2/1/0",
		"uturns/2/1/9",
		"uturns/2/1/3",
		"uturns/1/1/.",
		"uturns/1/1/",
		"uturns/2/3/1",
		"uturns/3/3/123456789zz",
		"uturns/3/3/12A",
		"uturns/3/3/z1",
		"uturns/9/3/z1",
		"uturns/3/3/123456789/",
		"uturns/x/3/3/123456789",
		"uturns/1/2/12",
		"uturns/4/4/1b2b3",
		"uturns/4/4/3.2.1."
	],
	fileios: [
		"pzprv3/uturns/2/2/1 . /2 3 /0 /1 /1 0 /",
		"pzprv3/uturns/2/2/- . /. 2 /-1 /1 /0 -1 /",
		"pzprv3/uturns/3/2/1 2 /3 - /. 1 /1 /0 /-1 /0 0 /0 0 /",
		"pzprv3/uturns/1/1/3 //"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 1, 2, 3, -2]);
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

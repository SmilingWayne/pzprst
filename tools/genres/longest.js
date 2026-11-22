// Fixture inputs for pzprjs pid "longest". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "longest",
	dataset: null,
	urls: [
		"longest/3/3/5v0",
		"longest/2/2/100",
		"longest/3/3/004",
		"longest/2/2/00o",
		"longest/4/4/vvvvvvvv",
		"longest/1/1/8",
		"longest/1/1/0",
		"longest/1/2",
		"longest/1/2/8",
		"longest/1/5/00",
		"longest/5/1/00",
		"longest/2/2/1G",
		"longest/2/2/1/0",
		"longest/3/3/5",
		"longest/3/3/zz",
		"longest/2/2/",
		"longest/2/2/0000",
		"longest/3/3/5v0zz",
		"longest/6/5/0",
		"https://puzz.link/p?longest/3/3/5v0",
		"http://pzv.jp/p.html?type=player&longest/2/2/100",
		"longest/x/3/3/5v0"
	],
	fileios: [
		"pzprv3/longest/3/3/0 0 0 0 /2 1 -1 0 /0 2 1 0 /0 0 0 /1 1 1 /1 0 0 /0 0 0 /",
		"pzprv3/longest/2/5/-1 2 0 0 0 0 /0 0 1 0 0 0 /0 0 0 0 0 /0 0 2 -1 0 /2 0 0 0 0 /",
		"pzprv3.1/longest/1/1/1 0 /2 /-1 /"
	],
	randomize(p, r) {
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.ques = 1;
			} else if (x < 0.35) {
				border.qans = 1;
			} else if (x < 0.45) {
				border.qsub = 1;
			}
		});
	}
};

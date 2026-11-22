// Fixture inputs for pzprjs pid "barns". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "barns",
	dataset: null,
	urls: [
		"barns/4/4/gg0gh308a8",
		"barns/2/2/00g",
		"https://puzz.link/p?barns/4/4/gg0gh308a8",
		"http://pzv.jp/p.html?type=player&barns/4/4/gg0gh308a8",
		"barns/4/4/gg0gh308a8/",
		"barns/4/4/gg0gh308a8//",
		"barns/4/4/gg0gh308a8zz",
		"barns/2/2/000zz",
		"barns/8/8/0",
		"barns/8/8",
		"barns/8/8/",
		"barns/2/2/0",
		"barns/1/1/0",
		"barns/1/4/v",
		"barns/4/1/g",
		"barns/4/1/V",
		"barns/5/1/vv",
		"barns/1/5/vv",
		"barns/3/5/v0000000",
		"barns/5/3/v18iggal",
		"barns/3/3/!!!",
		"barns/25/1/vvvvvvvvvv",
		"barns/1/25/vvvvvvvvvv",
		"barns/6/5/gg0gh308a8",
		"barns/8/8/vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv0"
	],
	fileios: [
		"pzprv3/barns/2/2/1 . /. . /1 /0 /0 0 /1 /0 /0 0 /",
		"pzprv3/barns/3/2/. . /. . /. . /0 /0 /0 /0 0 /0 0 /0 /0 /0 /0 0 /0 0 /",
		"pzprv3/barns/3/5/1 1 1 1 1 /. . . . 1 /. 1 . . . /1 0 0 1 /0 1 0 0 /0 0 1 0 /0 1 0 1 0 /1 0 1 0 1 /0 0 0 0 /1 0 0 0 /0 1 0 0 /0 0 0 0 0 /-1 0 0 0 0 /",
		"pzprv3/barns/4/4/1 . . . /. 1 . . /. . . . /. . . 1 /1 0 0 /0 1 0 /0 0 1 /1 0 0 /0 1 0 0 /0 0 1 0 /1 0 0 1 /0 0 0 /1 0 0 /0 1 0 /0 0 1 /-1 0 0 0 /0 -1 0 0 /0 0 -1 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 0, 0, 0, 6]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.ques = 1;
			} else if (x < 0.5) {
				border.line = 1;
			} else if (x < 0.6) {
				border.qsub = 2;
			}
		});
	}
};

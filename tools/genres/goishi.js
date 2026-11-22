// Fixture inputs for pzprjs pid "goishi". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "goishi",
	dataset: null,
	urls: [
		"goishi/4/4/dtfg",
		"goishi/4/4/vvvg",
		"https://puzz.link/p?goishi/4/4/dtfg",
		"http://pzv.jp/p.html?type=player&goishi/4/4/dtfg",
		"goishi/z/4/4/dtfg",
		"goishi/4/4/d",
		"goishi/4/4/dtf",
		"goishi/4/4/dtfgzz",
		"goishi/1/1/0",
		"goishi/1/1/g",
		"goishi/5/1/a",
		"goishi/1/5/a",
		"goishi/5/5/00000",
		"goishi/5/5/vvvvv",
		"goishi/7/3/ancug",
		"goishi/7/3/!ncug"
	],
	fileios: [
		"pzprv3/goishi/4/4/1 . . 4 /. 0 . . /0 . 0 . /. . . 16 /",
		"pzprv3/goishi/2/3/2 . 1 /. 3 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			if (r.rnd() < 0.5) {
				cell.ques = 7;
				cell.anum = -1;
			} else {
				cell.ques = 0;
				cell.anum = r.rnd() < 0.4 ? r.pick([1, 2, 3, 4, 5]) : -1;
			}
		});
	}
};

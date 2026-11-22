// Fixture inputs for pzprjs pid "aho". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "aho",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"aho/6/6/j3g3h33g33m33h3h3h33i",
		"aho/6/6/6l2i9i1j4i6i3l5",
		"aho/6/6/6l4h2l62l3h8l5",
		"https://puzz.link/p?aho/6/6/j3g3h33g33m33h3h3h33i",
		"http://pzv.jp/p.html?type=player&aho/6/6/j3g3h33g33m33h3h3h33i",
		"aho/6/6/j3g3h33g33m33h3h3h33i/",
		"aho/4/4/1zz",
		"aho/6/6/6l2i",
		"aho/4/4/.",
		"aho/4/4/+3e7",
		"aho/2/2/-ff",
		"aho/1/1/0",
		"aho/1/5/1",
		"aho/5/1/-19",
		"aho/4/4",
		"aho/3/3/z1",
		"aho/4/4/1G",
		"aho/x/4/4/1",
		"aho/f/4/4/1"
	],
	fileios: [
		// Real 6x6 puzzle with answer borders and crosses.
		"pzprv3/aho/6/6/. . . . 3 . /3 . . 3 3 . /3 3 . . . . /. . . 3 3 . /. 3 . . 3 . /. 3 3 . . . /0 1 0 2 0 /-1 0 0 0 1 /0 0 0 0 0 /0 2 0 0 0 /0 -1 0 0 0 /0 0 0 0 0 /0 0 1 0 0 0 /0 0 0 0 0 2 /1 0 0 0 0 0 /0 0 0 -1 0 0 /0 0 0 0 0 0 /",
		// Question mark, the largest clue and every border answer token.
		"pzprv3/aho/2/3/. - 999 /16 1 . /2 1 /-1 0 /1 2 -1 /",
		// Empty board: no clue and no answer data.
		"pzprv3/aho/3/2/. . . /. . . /0 0 /0 0 /0 0 0 /"
	],
	randomize(p, r) {
		const clues = [-1, -1, -1, -1, -1, 1, 2, 3, 15, 16, 255, 256, 999, -2];
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
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

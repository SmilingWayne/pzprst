// Fixture inputs for pzprjs pid "keywest". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "keywest",
	dataset: null,
	urls: [
		"keywest/5/5/gb7ej1008ega",
		"keywest/6/6/cgb97b8c8b86cbgb",
		"keywest/7/7/hbhdg1cahagagbha4dgchb",
		"https://puzz.link/p?keywest/6/6/cgb97b8c8b86cbgb",
		"http://pzv.jp/p.html?type=player&keywest/3/3/",
		"keywest/3/3/",
		"keywest/1/1/a",
		"keywest/1/1/.",
		"keywest/2/2/12",
		"keywest/3/3/1.2",
		"keywest/3/3/1.2zz",
		"keywest/1/3/c",
		"keywest/3/1/c",
		"keywest/4/4/a",
		"keywest/4/4/5",
		"keywest/10/10/"
	],
	fileios: [
		"pzprv3/keywest/5/5/. 1 . . 2 /. 4 . . . /. . . 1 0 /0 3 . 4 . /. . 0 . . /0 0 1 1 /1 1 1 0 /0 0 0 0 /0 1 1 1 /1 0 0 0 /1 1 1 1 1 /1 1 0 0 0 /0 1 0 1 0 /0 1 0 1 0 /1 . 2 3 . /3 . 3 2 1 /1 2 0 . . /. . 2 . 1 /1 2 . 1 0 /",
		"pzprv3/keywest/5/5/. 1 . . 2 /. 4 . . . /. . . 1 0 /0 3 . 4 . /. . 0 . . /0 0 1 1 /1 1 1 0 /0 0 0 0 /0 1 1 1 /1 0 0 0 /1 1 1 1 1 /1 1 0 0 0 /0 1 0 1 0 /0 1 0 1 0 /. . . . . /. . . . . /. . . . . /. . . . . /. . . . . /",
		"pzprv3/keywest/2/2/. - /. 2 /0 /0 /0 0 /. 0 /. + /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
			cell.anum = r.pick([-1, -1, -1, 0, 1, 2, 3, 4]);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
			cell.qcmp = r.rnd() < 0.1 ? 1 : 0;
		});
		p.board.border.each((border) => {
			border.line = r.rnd() < 0.25 ? 1 : 0;
			border.qsub = r.rnd() < 0.1 ? 2 : 0;
		});
	}
};

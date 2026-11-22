// Fixture inputs for pzprjs pid "recoil". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "recoil",
	dataset: null,
	urls: [
		"https://puzz.link/p?recoil/3/3/11",
		"recoil/3/3/11",
		"recoil/3/3/32",
		"recoil/3/3/33",
		"recoil/3/3/22",
		"recoil/4/5/",
		"recoil/3/4/",
		"recoil/1/1/",
		"recoil/1/1/11",
		"recoil/1/4/11",
		"recoil/4/1/11",
		"recoil/2/2/11",
		"recoil/4/4/33",
		"recoil/3/3/11zz",
		"recoil/3/3/11zzzz",
		"recoil/3/3/11.",
		"recoil/5/2/51",
		"recoil/2/5/15"
	],
	fileios: [
		"pzprv3/recoil/2/2/3 1/. # /# . /0 -1 0 /0 1 0 /0 0 /-2 0 /0 u /1 0 /0 2 /",
		"pzprv3/recoil/2/3/1 3/. . # /# . . /0 0 0 0 /0 -1 0 0 /0 0 0 /0 0 1 /d 0 0 /1 2 0 /0 0 1 /",
		"pzprv3/recoil/1/1/1 1/. /0 0 /0 /0 /2 /"
	],
	randomize(p, r) {
		p.board.startpos.bx = 2 * (1 + Math.floor(r.rnd() * p.board.cols)) - 1;
		p.board.startpos.by = 2 * (1 + Math.floor(r.rnd() * p.board.rows)) - 1;
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 0, 0, 7]);
			cell.qsub = r.pick([0, 0, 0, 0, 1, 2]);
		});
		p.board.border.each((border) => {
			border.qsub = r.pick([0, 0, 0, 0, 0, 2, 11, 12, 13, 14]);
			// pzprjs crashes rebuilding LineGraph when an outer-frame border has a line.
			if (border.isGrid()) {
				border.line = r.pick([0, 0, 0, 0, 1, 2]);
			}
		});
	}
};

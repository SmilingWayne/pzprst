// Fixture inputs for pzprjs pid "edamame". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "edamame",
	dataset: null,
	urls: [
		"edamame/8/8/h59.gd59.gd59.gd59.gd59.gd59.gd59.gd",
		"edamame/3/3/o",
		"https://puzz.link/p?edamame/8/8/h59.gd59.gd59.gd59.gd59.gd59.gd59.gd",
		"http://pzv.jp/p.html?type=player&edamame/8/8/h59.gd59.gd59.gd59.gd59.gd59.gd59.gd",
		"edamame/8/8/h59.gd59",
		"edamame/8/8/h59.gd59.gd59.gd59.gd59.gd59.gd59.gdzz",
		"edamame/x/8/8/h59.gd59.gd59.gd59.gd59.gd59.gd59.gd",
		"edamame/4/4/444444444444444e",
		"edamame/4/4/.g.g.g.g.g.g.g.g",
		"edamame/1/1/e",
		"edamame/5/1/0123e",
		"edamame/1/5/0123e",
		"edamame/12/9/56789.56789.56789.56789.56789.56789.56789.56789.56789.5678e",
		"edamame/2/2/",
		"edamame/2/2"
	],
	fileios: [
		"pzprv3/edamame/4/4/0 1 . - /2 . 3 . /4 . . . /. . . . /0 1 2 0 /0 4 8 0 /0 16 9 0 /0 0 0 0 /1 0 0 /1 0 0 /0 1 0 /0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /",
		"pzprv3/edamame/2/4/2 . - 4 /. 0 . 3 /16 0 0 2 /0 1 0 4 /0 0 1 /0 0 0 /0 0 0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
			if (r.rnd() < 0.25) {
				cell.qans = 1;
			}
			if (r.rnd() < 0.15) {
				cell.line = 1;
			}
			cell.qsub = r.pick([0, 0, 0, 0, 1, 2]);
			cell.qcmp = r.rnd() < 0.2 ? 1 : 0;
		});
		p.board.border.each((border) => {
			if (r.rnd() < 0.25) {
				border.line = 1;
			}
		});
	}
};

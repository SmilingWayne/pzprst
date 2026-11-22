// Fixture inputs for pzprjs pid "vertigo". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "vertigo",
	dataset: null,
	urls: [
		"vertigo/4/4",
		"vertigo/4/4/",
		"vertigo/4/4//",
		"https://puzz.link/p?vertigo/4/4/0000",
		"https://puzz.link/p?vertigo_edit/4/4/0000",
		"vertigo/f/4/4/0000",
		"vertigo/x/4/4/0000",
		"vertigo/4/4/g",
		"vertigo/4/4/000",
		"vertigo/4/4/0000zz",
		"vertigo/4/4/z",
		"vertigo/4/4/A",
		"vertigo/1/1/0",
		"vertigo/1/5/00000",
		"vertigo/5/1/00000",
		"vertigo/25/1/00000",
		"vertigo/1/25/00000",
		"vertigo/10/10/1c8g02240600a18i60o0",
		"vertigo/8/6/cp00001714",
		"vertigo/6/8/504b30q00k"
	],
	fileios: [
		"pzprv3/vertigo/3/2/* . . /* * . /u 2 /0 3d /-3 1 0 /",
		"pzprv3/vertigo/4/4/* * . . /* . . . /. . . . /. . . * /1 0 0 /0 1 0 /0 0 1 /0 0 0 /0 0 0 0 /0 0 0 0 /-2 0 0 0 /",
		"pzprv3/vertigo/1/2/* . /u /",
		"pzprv3/vertigo/1/1/* /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			if (r.rnd() < 0.25) {
				cell.ques = 7;
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.35) {
				border.line = 1;
				border.qsub = 2;
			} else if (x < 0.45) {
				border.qsub = r.pick([11, 12, 13, 14]);
			}
		});
	}
};

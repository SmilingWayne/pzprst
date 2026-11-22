// Fixture inputs for pzprjs pid "zabajaba". See tools/gen_fixtures.js for the format.
// "kaidan", "takoyaki", "edamame" and "wittgen" share pzprjs's variety/kaidan.js
// but override Encode/FileIO/Cell, so they are separate pids and are not covered here.
"use strict";

module.exports = {
	pid: "zabajaba",
	dataset: null,
	urls: [
		"zabajaba/4/4/1234",
		"zabajaba/4/4/012345678",
		"https://puzz.link/p?zabajaba/4/4/12345678",
		"http://pzv.jp/p.html?type=player&zabajaba/4/4/1234",
		"zabajaba_edit/3/3/123",
		"zabajaba/4/4/1",
		"zabajaba/4/4/1234zzzzzz",
		"zabajaba/4/4",
		"zabajaba/4/4/",
		"zabajaba/4/4//",
		"zabajaba/1/1/0",
		"zabajaba/2/2/1.2.",
		"zabajaba/2/2/--1",
		"zabajaba/2/1/0g",
		"zabajaba/1/2/g",
		"zabajaba/30/1/z1",
		"zabajaba/1/30/zp",
		"zabajaba/x/4/4/12",
		"zabajaba/4/4/8",
		"zabajaba/5/5/.8.8."
	],
	fileios: [
		"pzprv3/zabajaba/2/2/1 2 /. . /1 2 /4 8 /1 /-1 /0 1 /",
		"pzprv3/zabajaba/3/3/0 . 8 /. 3 . /- . . /0 1 2 /3 4 6 /8 9 10 /0 0 /1 -1 /0 1 /1 0 0 /0 1 0 /",
		"pzprv3/zabajaba/1/1/- /0 //"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -2, 0, 1, 2, 3, 4, 5, 6, 7, 8]);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
			cell.line = r.rnd() < 0.2 ? 1 : 0;
			cell.qcmp = r.rnd() < 0.15 ? 1 : 0;
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.3) {
				border.line = 1;
			} else if (x < 0.45) {
				border.qsub = 2;
			}
		});
	}
};

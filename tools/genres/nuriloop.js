// Fixture inputs for pzprjs pid "nuriloop". See tools/gen_fixtures.js for the format.
// "golemgrad" shares pzprjs's variety/nuriloop.js but overrides Cell/AnsCheck, so it
// is a separate pid and is not covered here.
"use strict";

module.exports = {
	pid: "nuriloop",
	dataset: null,
	urls: [
		"nuriloop/2/2/12",
		"https://puzz.link/p?nuriloop/4/4/1234",
		"nuriloop_edit/3/3/12.",
		"http://pzv.jp/p.html?type=player&nuriloop/2/2/12",
		"nuriloop/4/4/1",
		"nuriloop/4/4/12zzzzzz",
		"nuriloop/4/4",
		"nuriloop/4/4/",
		"nuriloop/4/4//",
		"nuriloop/1/1/1",
		"nuriloop/2/1/1g",
		"nuriloop/1/2/g",
		"nuriloop/30/1/z1",
		"nuriloop/1/30/zp",
		"nuriloop/x/4/4/12",
		"nuriloop/2/2/--1",
		"nuriloop/5/5/1.2.3.",
		"nuriloop/16/16/+100"
	],
	fileios: [
		"pzprv3/nuriloop/2/2/1 2 /. . /0 1 /2 0 /1 /-1 /0 1 /",
		"pzprv3/nuriloop/3/3/1 . 2 /. 3 . /- . . /0 0 0 /1 0 2 /0 0 0 /0 1 /-1 0 /0 1 /1 0 0 /0 1 0 /",
		"pzprv3/nuriloop/1/1/- /0 //"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.5) {
				cell.qnum = -1;
			} else if (x < 0.62) {
				cell.qnum = -2;
			} else {
				cell.qnum = 1 + Math.floor(r.rnd() * maxnum);
			}
			cell.qsub = r.pick([0, 0, 0, 0, 1, 2]);
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

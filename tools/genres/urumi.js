// Fixture inputs for pzprjs pid "urumi". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "urumi",
	dataset: null,
	urls: [
		"urumi/4/4/1234",
		"urumi/4/4/123456789abcde",
		"https://puzz.link/p?urumi/4/4/123456789abcde",
		"http://pzv.jp/p.html?type=player&urumi/4/4/1234",
		"urumi_edit/3/3/123",
		"urumi/4/4/1",
		"urumi/4/4/1234zzzzzz",
		"urumi/4/4",
		"urumi/4/4/",
		"urumi/4/4//",
		"urumi/1/1",
		"urumi/2/2/1.2.",
		"urumi/2/2/--1",
		"urumi/2/1/1g",
		"urumi/1/2/g",
		"urumi/30/1/z1",
		"urumi/1/30/zp",
		"urumi/x/4/4/12",
		"urumi/16/16/-ff"
	],
	fileios: [
		"pzprv3/urumi/2/2/1 2 /. . /1 /-1 /1 0 /",
		"pzprv3/urumi/3/3/1 . 2 /. 3 . /- . . /0 1 /-1 0 /0 1 /1 0 0 /0 1 0 /",
		"pzprv3/urumi/1/1/- //"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows - 1;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.5 || maxnum < 1) {
				cell.qnum = -1;
			} else if (x < 0.62) {
				cell.qnum = -2;
			} else {
				cell.qnum = 1 + Math.floor(r.rnd() * maxnum);
			}
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

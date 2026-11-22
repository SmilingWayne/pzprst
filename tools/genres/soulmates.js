// Fixture inputs for pzprjs pid "soulmates". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "soulmates",
	dataset: null,
	urls: [
		"soulmates/5/5/12345",
		"https://puzz.link/p?soulmates/5/5/12345",
		"http://pzv.jp/p.html?type=player&soulmates/5/5/12345",
		"soulmates_edit/3/3/120j0g",
		"soulmates/3/3/120j0g",
		"soulmates/3/3/120j0gzz",
		"soulmates/3/3/12",
		"soulmates/3/3/12zz",
		"soulmates/4/4",
		"soulmates/4/4/",
		"soulmates/1/1",
		"soulmates/1/1/0",
		"soulmates/1/5/1234",
		"soulmates/5/1/1234",
		"soulmates/6/6/-23",
		"soulmates/2/2/ff",
		"soulmates/3/3/000",
		"soulmates/6/6/z5",
		"soulmates/2/2/00",
		"soulmates/x/3/3/120j0g",
		"soulmates/20/20/+12c"
	],
	fileios: [
		"pzprv3/soulmates/3/3/1 2 #/. . ./. # ./c + -/. . ./= % 5",
		"pzprv3/soulmates/2/2/1 #/. 2/5 c/- .",
		"pzprv3/soulmates/2/2/# #/# #/./. ./. .",
		"pzprv3/soulmates/2/2/0 ./. ././.",
		"pzprv3/soulmates/1/1/#/."
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows - 1;
		p.board.cell.each((cell) => {
			cell.ques = 0;
			cell.qnum = -1;
			cell.anum = -1;
			cell.qsub = 0;
			cell.qcmp = 0;
			if (r.rnd() < 0.12) {
				cell.ques = 7;
				return;
			}
			if (maxnum >= 1 && r.rnd() < 0.45) {
				cell.qnum = 1 + Math.floor(r.rnd() * maxnum);
			}
			if (maxnum >= 1 && r.rnd() < 0.25) {
				cell.anum = 1 + Math.floor(r.rnd() * maxnum);
			}
			if (cell.anum === -1) {
				const x = r.rnd();
				if (x < 0.08) {
					cell.qcmp = 1;
				} else if (cell.qnum === -1 && x < 0.24) {
					cell.qsub = 2;
				}
			}
		});
	}
};

// Fixture inputs for pzprjs pid "yinyang". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "yinyang",
	dataset: "YinYang",
	urls: [
		// Canonical board with a well-known body, and the URL host forms.
		"yinyang/6/5/0090194019",
		"https://puzz.link/p?yinyang/6/5/0090194019",
		"http://pzv.jp/p.html?type=player&yinyang/6/5/0090194019",
		"https://puzz.link/p?yinyang_edit/6/5/0090194019",
		// Real puzzles from the dataset (already canonical).
		"yinyang/11/11/000063l16366310680k326951032212132b100000",
		"yinyang/6/5/0090194019",
		"yinyang/10/10/0101d8i9264ck60oia913ad1309d4c0600",
		// The largest base-27 digit (all black), and digits outside base 27.
		"yinyang/3/1/q",
		"yinyang/4/4/qqqqqq",
		"yinyang/3/1/Q",
		"yinyang/3/1/zz",
		"yinyang/3/3/....",
		"yinyang/3/3/+12c",
		// Truncated bodies, trailing junk and a missing body.
		"yinyang/6/5/0090",
		"yinyang/6/5/0090194019zz",
		"yinyang/6/5",
		"yinyang/6/5/",
		// One-cell boards and boards with padding after the last cell.
		"yinyang/1/1/0",
		"yinyang/1/1/2",
		"yinyang/1/1/13",
		// One-row and one-column boards.
		"yinyang/25/1/00000",
		"yinyang/1/25/00000",
		"yinyang/2/1/0",
		"yinyang/1/2/1",
		// A body that is longer than the board.
		"yinyang/2/2/120",
		"yinyang/2/2/qq"
	],
	fileios: [
		// Question circles, unknown circles and every answer mark.
		"pzprv3/yinyang/3/3/1 . 2 /- . . /. 1 2 /1 . 2 /. c . /+ - = /",
		// A full 2x2 board with shading only.
		"pzprv3/yinyang/2/2/. . /1 2 /1 . /. 2 /",
		// Question mark only, plus sub marks.
		"pzprv3/yinyang/2/2/- . /. - /% . /. + /",
		// One row.
		"pzprv3/yinyang/1/2/2 1 /1 . /",
		// 1x1 board.
		"pzprv3/yinyang/1/1/2 /1 /",
		// Every cell empty.
		"pzprv3/yinyang/2/3/. . . /. . . /. . . /. . . /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 1, 2, -2]);
			cell.anum = r.pick([-1, -1, -1, -1, 1, 2]);
			cell.qsub = 0;
			cell.qcmp = 0;
			if (cell.anum === -1) {
				const x = r.rnd();
				if (x < 0.12) {
					cell.qcmp = 1;
				} else if (x < 0.3) {
					cell.qsub = 1 + Math.floor(r.rnd() * 4);
				}
			}
		});
	}
};

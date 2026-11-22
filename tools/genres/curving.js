// Fixture inputs for pzprjs pid "curving". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "curving",
	dataset: "CurvingRoad",
	urls: [
		// Canonical empty board, with and without a body.
		"curving/4/4/0000",
		"curving/4/4",
		"curving/4/4/",
		// Circles, a full board and the URL host forms.
		"curving/4/4/1000",
		"https://puzz.link/p?curving/4/4/1000",
		"https://puzz.link/p?curving_edit/4/4/v",
		"curving/4/4/f000",
		// The last cell only, and the largest base-32 digit.
		"curving/4/4/0001",
		"curving/4/4/v",
		"curving/4/4/vvvv",
		"curving/1/1/0",
		"curving/1/1/v",
		// Uppercase digits and digits outside base 32.
		"curving/4/4/V",
		"curving/4/4/zz",
		"curving/5/5/zzzzz",
		// Truncated bodies and trailing junk.
		"curving/3/2/1",
		"curving/6/6/00001",
		"curving/4/4/0000zz",
		// One-row and one-column boards.
		"curving/25/1/00000",
		"curving/1/25/00000",
		"curving/2/1/g",
		"curving/1/2/g",
		"curving/3/2/10"
	],
	fileios: [
		// Circles, shading, background marks and every border value.
		"pzprv3/curving/4/4/. . . . /1 . . . /. . . . /. . . . /# . . + /. . . . /. # . . /. . + . /1 0 0 /0 -1 0 /1 0 1 /0 0 0 /1 0 0 1 /0 -1 0 0 /1 0 0 0 /",
		// A question mark (FileIO only: a curving URL cannot store it).
		"pzprv3/curving/2/2/- 1 /. . /# . /+ . /1 /0 /1 -1 /",
		// One row, answer layer included.
		"pzprv3/curving/1/2/1 . /# + /1 /",
		// An all-zero border layer is dropped by encodeBorderLineIfPresent.
		"pzprv3/curving/2/1/1 . /. . /0 /",
		// 1x1 board: no border layer at all.
		"pzprv3/curving/1/1/. /# /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 1]);
			const x = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.35) {
				cell.qsub = 1;
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

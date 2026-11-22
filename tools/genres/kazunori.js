// Fixture inputs for pzprjs pid "kazunori". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kazunori",
	dataset: null,
	urls: [
		// Canonical forms and empty bodies.
		"kazunori/3/3",
		"kazunori/3/3/",
		"kazunori/4/4",
		// Full URLs (player and editor) and a pzv.jp player URL.
		"https://puzz.link/p?kazunori/4/4/6g0o203h57.i2h.81k9h4",
		"https://puzz.link/p?kazunori_edit/4/4/6g0o203h57.i2h.81k9h4",
		"http://pzv.jp/p.html?kazunori/3/3/60g05h.g.5g7i",
		// Unknown pflags are ignored (kazunori defines no rule flag).
		"kazunori/f/4/4/6g0o203h57.i2h.81k9h4",
		"kazunori/xf/4/4/6g0o203h57.i2h.81k9h4",
		// Number layer: `.` is `?`, `g`-`z` skip borders and junk is ignored.
		"kazunori/3/3/000000.",
		"kazunori/3/3/0000g",
		"kazunori/3/3/0000z",
		"kazunori/3/3/0000!!",
		"kazunori/3/3/00000012",
		// Trailing junk.
		"kazunori/4/4/6g0o203h57.i2h.81k9h4zz",
		// 1x1, N/1 and 1/N boards.
		"kazunori/1/1",
		"kazunori/1/5",
		"kazunori/5/1",
		"kazunori/25/1",
		"kazunori/1/25",
		// A number layer that ends early on a wider board.
		"kazunori/5/5/00000"
	],
	fileios: [
		// 3x3: `!n` numbers, walls, `|`/`-`/`!` and the answer layer.
		"pzprv3/kazunori/3/3/!5 . /| - /. ! /5 . !7 /. . . /1 . c /. + . /- . = /",
		// 4x4 with walls, numbers up to 9 and every answer mark.
		"pzprv3/kazunori/4/4/!3 . | /5 !7 - /. . . /!2 . . /- 8 !1 . /. . . . /9 . . !4 /1 . c . /. + . - /. . . . /% . = 2 /",
		// 2x2 with a wall and a `?`.
		"pzprv3/kazunori/2/2/!3 /| /! . /1 . /. c /",
		// 1x1: the empty border layer.
		"pzprv3/kazunori/1/1//1 /"
	],
	randomize(p, r) {
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.3 ? 1 : 0;
			border.qnum = -1;
		});
		// `Border.maxnum` is the sum of the two adjacent cells' room maxima.
		p.board.rebuildInfo();
		p.board.border.each((border) => {
			const max = border.maxnum();
			const x = r.rnd();
			if (x < 0.15) {
				border.qnum = -2;
			} else if (x < 0.75) {
				border.qnum = 1 + Math.floor(r.rnd() * max);
			}
		});
		p.board.cell.each((cell) => {
			cell.anum = -1;
			cell.qcmp = 0;
			cell.qsub = 0;
			const x = r.rnd();
			if (x < 0.5) {
				cell.anum = 1 + Math.floor(r.rnd() * cell.maxnum());
			} else if (x < 0.6) {
				cell.qcmp = 1;
			} else if (x < 0.7) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
	}
};

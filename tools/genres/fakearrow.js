// Fixture inputs for pzprjs pid "fakearrow". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "fakearrow",
	dataset: null,
	urls: [
		// Canonical form, empty bodies and the empty body with slashes.
		"fakearrow/4/4",
		"fakearrow/4/4/",
		"fakearrow/4/4//",
		// Full URLs (player and editor) and a pzv.jp player URL.
		"https://puzz.link/p?fakearrow/4/4/0000001234123412341234",
		"https://puzz.link/p?fakearrow_edit/4/4/0000001234123412341234",
		"http://pzv.jp/p.html?fakearrow/4/4/0000001234123412341234",
		"https://puzz.link/p?fakearrow/4/4/000000v",
		// Unknown pflags are ignored (fakearrow defines no rule flag).
		"fakearrow/f/4/4/0000001234123412341234",
		"fakearrow/xf/4/4/0000001234123412341234",
		// Border walls, arrows and the value 5 (a cell that does not exist).
		"fakearrow/3/3/k0h0512341234",
		"fakearrow/3/3/k0h0",
		// 1x1: an arrow, `.` (ignored), a `-` hex escape for the same arrow and `g`.
		"fakearrow/1/1/1",
		"fakearrow/1/1/.",
		"fakearrow/1/1/-01",
		"fakearrow/1/1/g",
		// 1/N and N/1 boards.
		"fakearrow/1/5/12345",
		"fakearrow/5/1/12345",
		"fakearrow/25/1/00000",
		"fakearrow/1/25/00000",
		// Base-36 runs, then bodies that end early.
		"fakearrow/4/4/v",
		"fakearrow/4/4/0000",
		"fakearrow/4/4/000000g123123412341234",
		// Trailing junk.
		"fakearrow/4/4/1234123412341234zz",
		// Larger boards that mix walls, arrows and empty cells.
		"fakearrow/2/2/001234",
		"fakearrow/6/6/00000000000012345123451234512345123451234512345123451",
		"fakearrow/5/5/0000000000123451234512345123451234512345",
		"fakearrow/8/6/00000000000000000000000012345123451234512345123451234512345123451234512345"
	],
	fileios: [
		// 3x3: border walls, arrows / `#`, lines with aux marks, shade / marks.
		"pzprv3/fakearrow/3/3/1 0 /0 1 /0 0 /0 0 1 /1 0 0 /1 2 # /3 . 4 /. . . /1u -1 /0 0 /0 1d /0 0 1r /1l 0 0 /. 1 . /. . + /. . . /",
		// 2x2: a wall border, `#` and the answer layers.
		"pzprv3/fakearrow/2/2/1 /0 /0 1 /# 2 /3 4 /1u /0 /0 0 /1 . /. . /",
		// 4x2: pzprjs quirk — a FileIO cell token of 5 is qdir 5 (the URL writes
		// `5` back as `ques` 7), an all-empty border line and a `-1` cross.
		"pzprv3/fakearrow/4/2/1 /0 /1 /0 /1 0 /0 1 /0 0 /1 2 /# 4 /5 . /3 1 /1u /0 /-1 /2d /0 1r /0 0 /0 0 /1 . /. 2 /. . /3 . /",
		// 1x1: one cell and the empty border layers.
		"pzprv3/fakearrow/1/1//1 //. /",
		// 1x1: a cell that does not exist plus its answer mark.
		"pzprv3/fakearrow/1/1//# //1 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			if (r.rnd() < 0.2) {
				cell.ques = 7;
			} else {
				cell.qdir = Math.floor(r.rnd() * 5);
			}
			if (r.rnd() < 0.2) {
				cell.qans = 1;
			}
			if (r.rnd() < 0.2) {
				cell.qsub = Math.floor(r.rnd() * 5);
			}
		});
		p.board.border.each((border) => {
			if (r.rnd() < 0.2) {
				border.ques = 1;
			}
			const x = r.rnd();
			if (x < 0.2) {
				border.line = 1;
			} else if (x < 0.3) {
				border.line = 1;
				border.qsub = 2;
			} else if (x < 0.45) {
				border.qsub = r.pick([11, 12, 13, 14]);
			}
		});
	}
};

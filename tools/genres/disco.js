// Fixture inputs for pzprjs pid "disco". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "disco",
	dataset: null,
	urls: [
		// Canonical boards, with and without a body, and the URL host forms.
		"disco/4/4/00vvo0",
		"https://puzz.link/p?disco/4/4/00vvo0",
		"http://pzv.jp/p.html?type=player&disco/4/4/00vvo0",
		"https://puzz.link/p?disco_edit/4/4/00vvo0",
		"disco/4/4",
		"disco/4/4/",
		// Every wall set: the padding bits of the last group are not stored.
		"disco/4/4/vvvvvv",
		"disco/4/4/0000000000000000",
		// Case-insensitive base-32 digits.
		"disco/4/4/AAAAAA",
		"disco/4/4/aa8aa8",
		// Digits outside base 32.
		"disco/4/4/!!!!!!",
		// Truncated bodies and trailing junk.
		"disco/4/4/0",
		"disco/4/4/00",
		"disco/4/4/000000zz",
		// One-cell and small boards.
		"disco/1/1/0",
		"disco/2/2/00",
		"disco/2/1/0",
		"disco/1/2/0",
		"disco/3/3/0v0",
		"disco/3/3/0v00",
		"disco/5/5/0000000",
		"disco/6/5/abcdef",
		// One-row and one-column boards.
		"disco/25/1/vvvvv",
		"disco/1/25/vvvvv",
		// A full-height board.
		"disco/8/8/000000000000000000000000"
	],
	fileios: [
		// Border walls, shading and background marks.
		"pzprv3/disco/4/4/0 1 0 /1 0 0 /0 0 1 /0 0 0 /1 0 0 0 /0 1 0 0 /0 0 1 1 /# . + . /. # . . /. . . # /. . . . /",
		// A non-square board.
		"pzprv3/disco/2/3/0 1 /1 0 /0 1 0 /+ . # /. # . /",
		// 1x1 board: the empty border layer line, then shading.
		"pzprv3/disco/1/1//# /",
		// One column.
		"pzprv3/disco/3/2/1 /0 /1 /0 1 /0 0 /# . /. + /. # /",
		// Every wall, alternating shading.
		"pzprv3/disco/2/2/1 /1 /1 1 /# . /. # /",
		// A wide board with all layers mixed.
		"pzprv3/disco/3/5/1 0 1 0 /0 1 0 1 /1 0 0 1 /0 1 0 1 0 /1 0 1 0 1 /# . + . # /. # . # . /. . # . + /",
		// Every cell empty.
		"pzprv3/disco/2/3/0 0 /0 0 /0 0 0 /. . . /. . . /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qans = r.rnd() < 0.4 ? 1 : 0;
			cell.qsub = cell.qans === 0 && r.rnd() < 0.15 ? 1 : 0;
		});
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.35 ? 1 : 0;
		});
	}
};

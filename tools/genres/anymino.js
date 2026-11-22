// Fixture inputs for pzprjs pid "anymino". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "anymino",
	dataset: null,
	urls: [
		// Canonical boards, with and without a body, and the URL host forms.
		"anymino/4/4/000000",
		"anymino/4/4",
		"anymino/4/4/",
		"https://puzz.link/p?anymino/4/4/vvovvo",
		"http://pzv.jp/p.html?type=player&anymino/4/4/vvovvo",
		"https://puzz.link/p?anymino_edit/4/4/vvovvo",
		// Every wall set: the padding bits of the last group are not stored.
		"anymino/4/4/vvovvo",
		"anymino/4/4/vvvvvv",
		"anymino/4/4/0000000000000000",
		// Only vertical walls, then only horizontal walls.
		"anymino/4/4/vvo000",
		"anymino/4/4/000vvo",
		// Digits outside base 32 and uppercase digits.
		"anymino/4/4/VVVVVV",
		"anymino/4/4/!!!!!!",
		// Truncated bodies and trailing junk.
		"anymino/4/4/0",
		"anymino/4/4/vv",
		"anymino/4/4/000000zz",
		// One-cell and small boards.
		"anymino/1/1/0",
		"anymino/2/2/0",
		"anymino/2/2/00",
		"anymino/3/2/00",
		"anymino/2/3/00",
		// One-row and one-column boards.
		"anymino/1/4/v",
		"anymino/4/1/v",
		"anymino/25/1/vvvvvvvvvv",
		"anymino/1/25/vvvvvvvvvv"
	],
	fileios: [
		// Border walls, shading and background marks.
		"pzprv3/anymino/4/4/0 1 0 /0 0 1 /1 0 0 /0 0 0 /1 0 0 0 /0 1 0 0 /0 0 0 1 /# . . + /. # . . /. . # . /. . . # /",
		// A disconnected wall layout on a non-square board.
		"pzprv3/anymino/2/3/1 0 /0 1 /0 0 1 /# # . /. + . /",
		// Every wall, every cell shaded.
		"pzprv3/anymino/2/2/1 /1 /1 1 /# # /# # /",
		// One row.
		"pzprv3/anymino/1/3/1 1 /# . + /",
		// 1x1 board: the empty border layer line, then a background mark.
		"pzprv3/anymino/1/1//+ /",
		// Every cell empty.
		"pzprv3/anymino/2/3/0 0 /0 0 /0 0 0 /. . . /. . . /"
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

// Fixture inputs for pzprjs pid "kaero". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kaero",
	dataset: null,
	urls: [
		// Canonical boards and the URL host forms.
		"kaero/4/4/000000.1Z0-7k",
		"kaero/4/4/000000.1Z0-7",
		"https://puzz.link/p?kaero/4/4/000000p",
		"http://pzv.jp/p.html?type=player&kaero/4/4/000000p",
		"https://puzz.link/p?kaero_edit/4/4/000000p",
		// Border layer only, no body at all and a truncated border body.
		"kaero/4/4/000000p",
		"kaero/4/4/000000",
		"kaero/4/4",
		"kaero/4/4/",
		"kaero/4/4/00",
		// Trailing junk after the last cell.
		"kaero/4/4/000000zzzzzz",
		// The largest URL value (-Z is 72), the largest game value (-F is 52).
		"kaero/4/4/000000-Z",
		"kaero/4/4/000000-F",
		"kaero/4/4/000000-7",
		// Base-36 run markers for empty cells.
		"kaero/4/4/000000aA",
		"kaero/4/4/000000m",
		"kaero/4/4/000000abc",
		// A body longer than the board, which decodes the extra cells.
		"kaero/4/4/000000000000p",
		// Padding after the last cell.
		"kaero/1/1/0A",
		"kaero/1/1/0z",
		// One-cell and small boards.
		"kaero/2/2/00",
		"kaero/2/2/0a",
		"kaero/3/2/000e",
		"kaero/2/3/00Bc",
		// One-row and one-column boards.
		"kaero/25/1/00000y",
		"kaero/1/25/00000y",
		// 30 cells: the encoder must use the 26-cell run marker.
		"kaero/6/5/0000000000zn"
	],
	fileios: [
		// Numbers, unknown numbers, marks, border walls and border lines.
		"pzprv3/kaero/4/4/1 . 2 . /. 3 . . /. . - 5 /. . . 6 /1 . + . /. 1 . . /. . = . /. . . % /1 0 1 /0 1 0 /1 0 0 /0 0 1 /1 0 1 0 /0 1 0 1 /0 0 1 0 /1 0 0 /0 1 0 /0 0 1 /1 0 0 /0 1 0 1 /1 0 1 0 /0 1 1 0 /",
		// Two numbers per row and every answer mark kind.
		"pzprv3/kaero/2/2/. . /1 2 /1 . /. + /1 /0 /1 1 /1 /0 /0 1 /",
		// 1x1 board with the largest number; the two empty border layers.
		"pzprv3/kaero/1/1/52 /. ///",
		// A wide board with numbers 1, 26 and 36.
		"pzprv3/kaero/2/3/1 . - /. 26 36 /1 . + /. . = /1 0 /0 1 /1 0 1 /1 0 /0 1 /1 0 0 /",
		// 30 empty cells: the encoder must use the 26-cell run marker.
		"pzprv3/kaero/5/6/. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 1, 2, 13, 26, 27, 36, 37, 52, -2]);
			cell.anum = -1;
			cell.qans = r.rnd() < 0.2 ? 1 : 0;
			cell.qsub = 0;
			cell.qcmp = 0;
			if (cell.qans === 0 && r.rnd() < 0.2) {
				cell.qsub = 1 + Math.floor(r.rnd() * 4);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.ques = x < 0.3 ? 1 : 0;
			border.line = x > 0.6 ? 1 : 0;
		});
	}
};

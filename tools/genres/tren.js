// Fixture inputs for pzprjs pid "tren". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tren",
	dataset: null,
	urls: [
		// Canonical boards and the URL host forms.
		"tren/4/4/1234r",
		"tren/4/4/1234",
		"https://puzz.link/p?tren/4/4/1234",
		"http://pzv.jp/p.html?type=player&tren/4/4/1234",
		"https://puzz.link/p?tren_edit/4/4/1234",
		// The tren_new flag, alone and next to an unknown flag.
		"tren/n/4/4/1234",
		"tren/xn/4/4/1234",
		"tren/x/4/4/1234",
		// No body and an empty body.
		"tren/4/4",
		"tren/4/4/",
		// 0 is a valid number, `.` is unknown, `-18` is the escape for 24.
		"tren/4/4/0.2-",
		"tren/4/4/0",
		"tren/4/4/2",
		"tren/4/4/-f",
		"tren/4/4/-18",
		// Truncated body and trailing junk.
		"tren/4/4/1",
		"tren/4/4/1234zz",
		"tren/4/4/zzzz",
		// One-cell and small boards.
		"tren/1/1/0",
		"tren/2/2/0",
		"tren/3/2/0",
		"tren/2/3/1",
		// One-row and one-column boards.
		"tren/25/1/01234567890123456789012",
		"tren/1/25/01234567890123456789012",
		// 30 empty cells: the encoder must use the 20-cell run marker.
		"tren/6/5",
		// A 10x10 board with a number above 15.
		"tren/10/10/1234567890abcdef-10"
	],
	fileios: [
		// Numbers, border answers (all four values) and cell marks.
		"pzprv3/tren/4/4/1 2 3 4 /. . . . /. . . . /. . . . /2 0 1 /0 1 0 /1 0 -1 /0 0 1 /1 0 1 0 /0 1 0 1 /-1 0 1 0 /1 . + . /. 2 . . /. . = . /. . . % /",
		// The tren_new flag line, with a solved-looking answer.
		"pzprv3/tren/4/4/n/0 . . . /. 1 . . /. . 2 . /. . . - /1 0 0 /0 1 0 /0 0 1 /1 0 0 /0 1 0 1 /1 0 1 0 /0 0 1 1 /1 . . . /+ 1 . . /. = . 1 /. . % 1 /",
		// A small board with border answers and marks.
		"pzprv3/tren/2/2/0 . /. 1 /2 /1 /-1 0 /1 . /. + /",
		// 1x1 board: the empty border answer layer.
		"pzprv3/tren/1/1/0 //. /",
		// A wide board with two numbers per row.
		"pzprv3/tren/3/2/0 . /. 1 /2 . /1 /0 /0 /1 0 /0 1 /1 . /. + /. . /",
		// 30 empty cells: the encoder must use the 20-cell run marker.
		"pzprv3/tren/5/6/. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows) - 2;
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 0, 1, -2, maxnum]);
			cell.anum = -1;
			cell.qans = r.rnd() < 0.15 ? 1 : 0;
			cell.qsub = 0;
			cell.qcmp = 0;
			if (cell.qans === 0 && r.rnd() < 0.2) {
				cell.qsub = 1 + Math.floor(r.rnd() * 4);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qans = x < 0.3 ? 1 : 0;
			border.qsub = x > 0.6 ? 1 : 0;
		});
		p.setConfig("tren_new", r.rnd() < 0.4);
	}
};

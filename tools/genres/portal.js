// Fixture inputs for pzprjs pid "portal". See tools/gen_fixtures.js for the format.
// There is no Portal folder under ../puzzlekit-dataset/assets/data, so the real
// puzzle below comes from pzprjs test/script/portal.js.
"use strict";

module.exports = {
	pid: "portal",
	dataset: null,
	urls: [
		// Real 6x6 puzzle; canonicalises to itself.
		"portal/6/6/g1h2m1j3g2h4o43h",
		"https://puzz.link/p?portal/6/6/g1h2m1j3g2h4o43h",
		"http://pzv.jp/p.html?type=player&portal/6/6/g1h2m1j3g2h4o43h",
		"portal_edit/6/6/g1h2m1j3g2h4o43h",
		// 1x1, 1xN and Nx1 boards.
		"portal/1/1/0",
		"portal/1/3/0",
		"portal/3/1/0",
		// Truncated body, trailing junk, empty body.
		"portal/6/6/g1h2",
		"portal/6/6/g1h2m1j3g2h4o43hzz",
		"portal/3/3/",
		"portal/6/6",
		// The shaded-cell layer follows the number layer (`g` = cell 0 shaded).
		"portal/2/2/jg",
		// Every single-digit value, plus `?` cells (`.` in the number layer).
		"portal/4/4/0123456789abcdef",
		"portal/2/2/0",
		"portal/2/2/."
	],
	fileios: [
		// The real 6x6 puzzle, answered: three shaded cells, some path lines and a cross.
		"pzprv3/portal/6/6/. 1 . . 2 . /. . . . . . /1 . . . . 3 /. 2 # . 4 . /# . . . . . /. . 4 3 . # /1 0 0 0 0 /0 1 0 0 0 /0 0 1 0 0 /0 0 0 1 0 /0 0 0 0 1 /0 0 0 0 0 /1 1 0 0 0 0 /0 1 1 0 0 0 /0 0 1 1 0 0 /-1 0 0 0 0 0 /0 0 0 0 0 0 /",
		// 2x2: shaded cell, `?`, a clue and a cross.
		"pzprv3/portal/2/2/# - /1 . /1 /0 /0 -1 /",
		// 1x1 has no inner borders; only a shaded cell and one empty border line.
		"pzprv3/portal/1/1/# //"
	],
	randomize(p, r) {
		const maxnum = (p.board.cols * p.board.rows) >> 1;
		p.board.cell.each((cell) => {
			cell.qnum = -1;
			cell.ques = 0;
			const x = r.rnd();
			if (x < 0.35) {
				cell.qnum = Math.floor(r.rnd() * (maxnum + 1));
			} else if (x < 0.5) {
				cell.qnum = -2;
			} else if (x < 0.7) {
				cell.ques = 7;
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

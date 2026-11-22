// Fixture inputs for pzprjs pid "renban". See tools/gen_fixtures.js for the format.
// The dataset folder is Renban; the real 4x4 puzzle below is pzprjs's own test
// URL (test/script/renban.js), already canonical.
"use strict";

module.exports = {
	pid: "renban",
	dataset: "Renban",
	urls: [
		// The real 4x4 puzzle from pzprjs's test data.
		"renban/4/4/vmok3g1p5g2h",
		"https://puzz.link/p?renban/4/4/vmok3g1p5g2h",
		"http://pzv.jp/p.html?type=player&renban/4/4/vmok3g1p5g2h",
		"renban_edit/4/4/vmok3g1p5g2h",
		"renban/4/4/vmok3g1p5g2h/",
		"renban/4/4/vmok3g1p5g2hzz",
		// Truncated border layer.
		"renban/4/4/vmok",
		// `?` in the clue layer and a wall-only body.
		"renban/2/2/00.",
		"renban/2/2/00",
		// The largest clue, 999, is the `+` hex escape 0x3e7.
		"renban/4/4/000000+3e7",
		// 1/1, 1/N and N/1 boards.
		"renban/1/1/1",
		"renban/1/5/0k",
		"renban/5/1/0k",
		// Empty bodies, an unknown character and an unknown flag.
		"renban/4/4",
		"renban/4/4/",
		"renban/4/4/000000A",
		"renban/x/4/4/vmok3g1p5g2h"
	],
	fileios: [
		// The real 4x4 puzzle in pzprjs's solved test state: walls, clues and
		// answer numbers in the anumsub layer.
		"pzprv3/renban/4/4/1 1 1 /1 1 1 /0 1 1 /0 1 1 /1 0 1 0 /0 0 0 0 /1 1 1 0 /1 . . . /. . . . /. . . 5 /. 2 . . /. 3 . . /2 . . . /. 3 . . /. . . . /",
		// 3x2: `?` clue, every anumsub mark and one wall.
		"pzprv3/renban/2/3/1 0 /0 1 /1 0 1 /- 1 . /. . 2 /c . + /- = % /",
		// 1x1 has one empty vertical-border line (before the cell lines).
		"pzprv3/renban/1/1//1 /./"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qnum = -1;
			if (x < 0.14) {
				cell.qnum = -2;
			} else if (x < 0.7) {
				cell.qnum = 1 + Math.floor(r.rnd() * 999);
			}
			// One FileIO token per cell: never set two of anum / qcmp / qsub.
			cell.anum = -1;
			cell.qcmp = 0;
			cell.qsub = 0;
			const y = r.rnd();
			if (y < 0.12) {
				cell.anum = r.pick([1, 2, 3, 12, 999]);
			} else if (y < 0.22) {
				cell.qcmp = 1;
			} else if (y < 0.32) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.35 ? 1 : 0;
		});
	}
};

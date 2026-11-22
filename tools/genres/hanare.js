// Fixture inputs for pzprjs pid "hanare". See tools/gen_fixtures.js for the format.
// The dataset folder is Hanare (it has no puzzlink_url values); the real 4x4
// puzzle below is pzprjs's own test URL (test/script/hanare.js), already canonical.
"use strict";

module.exports = {
	pid: "hanare",
	dataset: "Hanare",
	urls: [
		// The real 4x4 puzzle from pzprjs's test data.
		"hanare/4/4/jegf6gu3",
		"https://puzz.link/p?hanare/4/4/jegf6gu3",
		"http://pzv.jp/p.html?type=player&hanare/4/4/jegf6gu3",
		"hanare_edit/4/4/jegf6gu3",
		"hanare/4/4/jegf6gu3/",
		"hanare/4/4/jegf6gu3zz",
		// Truncated border layer.
		"hanare/4/4/jegf",
		// `?` in the clue layer and a wall-only body.
		"hanare/2/2/00.",
		"hanare/2/2/00",
		// The largest clue, 999, is the `+` hex escape 0x3e7.
		"hanare/4/4/000000+3e7",
		// 1/1, 1/N and N/1 boards.
		"hanare/1/1/1",
		"hanare/1/5/0k",
		"hanare/5/1/0k",
		// Empty bodies, an unknown character and an unknown flag.
		"hanare/4/4",
		"hanare/4/4/",
		"hanare/4/4/000000A",
		"hanare/x/4/4/jegf6gu3"
	],
	fileios: [
		// The real 4x4 puzzle in pzprjs's solved test state: walls, clues and
		// answer numbers / `+` marks in the anumsub layer.
		"pzprv3/hanare/4/4/1 0 0 /1 1 0 /1 1 1 /0 1 0 /0 1 1 1 /1 0 0 1 /1 0 1 0 /. . . . /. . . . /. . . . /. . . 3 /2 + 3 + /+ + 3 + /1 + + + /+ 4 + . /",
		// 3x2: `?` clue, every anumsub mark and one wall.
		"pzprv3/hanare/2/3/1 0 /0 1 /1 0 1 /- 1 . /. . 2 /c . + /- = % /",
		// 1x1 has one empty vertical-border line (before the cell lines).
		"pzprv3/hanare/1/1//1 /./"
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

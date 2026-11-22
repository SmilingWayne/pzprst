// Fixture inputs for pzprjs pid "view". See tools/gen_fixtures.js for the format.
// There is no View folder under ../puzzlekit-dataset/assets/data and
// .pzprst-samples.js finds no native-derived links, so the real 5x5 puzzle below
// comes from pzprjs's own test data (test/script/view.js).
"use strict";

module.exports = {
	pid: "view",
	dataset: null,
	urls: [
		// The real 5x5 puzzle from pzprjs's test data (already canonical).
		"view/5/5/m401g3g2g101m",
		"https://puzz.link/p?view/5/5/m401g3g2g101m",
		"http://pzv.jp/p.html?type=player&view/5/5/m401g3g2g101m",
		"view_edit/5/5/m401g3g2g101m",
		"view/5/5/m401g3g2g101m/",
		"view/5/5/m401g3g2g101mzz",
		// Truncated body.
		"view/5/5/m401",
		// Clue 0 is legal (`minnum` 0); `.` is `?`.
		"view/2/2/00.",
		// The largest clue on a 5x5 board is 8, on a 10x10 board 18 (`-12`).
		"view/5/5/8",
		"view/10/10/-12",
		// 1/1, 1/N and N/1 boards.
		"view/1/1/0",
		"view/1/5/01234",
		"view/5/1/01234",
		// All-zero clues, empty bodies, an unknown character and flag.
		"view/4/4/0000000000000000",
		"view/4/4",
		"view/4/4/",
		"view/4/4/000A",
		"view/x/5/5/m401g3g2g101m"
	],
	fileios: [
		// The real 5x5 puzzle in pzprjs's solved test state: clues and the
		// anumsub layer with numbers, `-` marks.
		"pzprv3/view/5/5/. . . . . /. . 4 0 1 /. 3 . 2 . /1 0 1 . . /. . . . . /- - - 3 0 /- - . . . /2 . - . - /. . . 0 2 /- 1 0 1 . /",
		// 3x2: `?` clue, every anumsub mark and a `c` mark.
		"pzprv3/view/2/3/- 1 . /. . 2 /c . + /- = % /",
		// 1x1: clue 0 and an empty answer layer.
		"pzprv3/view/1/1/0 /."
	],
	randomize(p, r) {
		const max = Math.min(999, p.board.cols + p.board.rows - 2);
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qnum = -1;
			if (x < 0.12) {
				cell.qnum = -2;
			} else if (x < 0.75) {
				cell.qnum = Math.floor(r.rnd() * (max + 1));
			}
			// One FileIO token per cell: never set two of anum / qcmp / qsub.
			cell.anum = -1;
			cell.qcmp = 0;
			cell.qsub = 0;
			const y = r.rnd();
			if (y < 0.12) {
				cell.anum = r.pick([0, 1, 2, 3, 8, 18]);
			} else if (y < 0.22) {
				cell.qcmp = 1;
			} else if (y < 0.32) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
	}
};

// Fixture inputs for pzprjs pid "putteria". See tools/gen_fixtures.js for the format.
// The dataset folder is Putteria (it has no puzzlink_url values); the real 6x6
// puzzle below is pzprjs's own test URL (test/script/putteria.js), already canonical.
"use strict";

module.exports = {
	pid: "putteria",
	dataset: "Putteria",
	urls: [
		// The real 6x6 puzzle from pzprjs's test data: walls only.
		"putteria/6/6/2anr5pu5r2pezv",
		"https://puzz.link/p?putteria/6/6/2anr5pu5r2pezv",
		"http://pzv.jp/p.html?type=player&putteria/6/6/2anr5pu5r2pezv",
		"putteria_edit/6/6/2anr5pu5r2pezv",
		"putteria/6/6/2anr5pu5r2pezv/",
		"putteria/6/6/2anr5pu5r2pezvzz",
		// Truncated border layer.
		"putteria/6/6/2anr5p",
		// `?` in the clue layer and a wall-only body.
		"putteria/2/2/00.",
		"putteria/2/2/00",
		// The largest clue, 999, is the `+` hex escape 0x3e7.
		"putteria/4/4/000000+3e7",
		// 1/1, 1/N and N/1 boards.
		"putteria/1/1/1",
		"putteria/1/5/0k",
		"putteria/5/1/0k",
		// Empty bodies, an unknown character and an unknown flag.
		"putteria/4/4",
		"putteria/4/4/",
		"putteria/4/4/000000A",
		"putteria/x/6/6/2anr5pu5r2pezv"
	],
	fileios: [
		// The real 6x6 puzzle in pzprjs's solved test state: walls and the
		// anumsub layer with answer numbers and `+` marks.
		"pzprv3/putteria/6/6/0 0 0 1 0 /0 1 0 1 0 /1 0 1 1 1 /1 1 0 1 1 /0 0 1 0 1 /1 1 0 0 1 /1 1 1 1 0 0 /0 1 0 1 1 1 /0 1 1 0 0 0 /1 0 1 1 0 0 /1 0 1 1 1 0 /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /+ . 4 + + + /+ + + + 4 + /+ 4 + 3 + + /4 + + + + + /+ 5 + 4 + + /1 + 3 + + 4 /",
		// 3x2: `?` clue, every anumsub mark and one wall.
		"pzprv3/putteria/2/3/1 0 /0 1 /1 0 1 /- 1 . /. . 2 /c . + /- = % /",
		// 1x1 has one empty vertical-border line (before the cell lines).
		"pzprv3/putteria/1/1//1 /./"
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

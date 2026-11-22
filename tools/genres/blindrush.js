// Fixture inputs for pzprjs pid "blindrush". See tools/gen_fixtures.js for the format.
// There is no Blind Rush folder under ../puzzlekit-dataset/assets/data and
// .pzprst-samples.js finds no native-derived links, so the real 5x5 puzzle comes
// from pzprjs's own test data: test/script/blindrush.js.
"use strict";

module.exports = {
	pid: "blindrush",
	dataset: null,
	urls: [
		// The real 5x5 puzzle from pzprjs's test data (already canonical).
		"blindrush/5/5/0h0lo028.r1m2i",
		"https://puzz.link/p?blindrush/5/5/0h0lo028.r1m2i",
		"http://pzv.jp/p.html?type=player&blindrush/5/5/0h0lo028.r1m2i",
		"blindrush_edit/5/5/0h0lo028.r1m2i",
		"blindrush/5/5/0h0lo028.r1m2i/",
		"blindrush/5/5/0h0lo028.r1m2izz",
		// Truncated border layer.
		"blindrush/5/5/0h0l",
		// The largest clue on a 5x5 board is 4 (cell.maxnum).
		"blindrush/5/5/0h0lo028.4",
		// `?` and a wall-only body.
		"blindrush/2/2/00.",
		"blindrush/2/2/00",
		// 1/1 (maxnum 0, the clue 0 is legal), 1/N and N/1 boards.
		"blindrush/1/1/0",
		"blindrush/1/1",
		"blindrush/1/5/1",
		"blindrush/5/1/1",
		// Empty body, trailing slash and an unknown character.
		"blindrush/4/4",
		"blindrush/4/4/",
		"blindrush/4/4/000000A",
		"blindrush/x/5/5/0h0lo028.r1m2i"
	],
	fileios: [
		// The real 5x5 puzzle in pzprjs's solved test state: clues, walls,
		// `c` marks and answer lines.
		"pzprv3/blindrush/5/5/- . . . . /. . . . . /. . . 1 . /. . . . . /. 2 . . . /0 0 0 0 /0 1 0 0 /0 1 0 0 /0 0 0 1 /0 1 0 1 /1 1 0 0 0 /0 0 0 0 0 /0 0 0 1 0 /0 1 0 0 0 /c . . . . /. . . . . /. . . . . /. . . . . /. . . . . /1 1 1 1 /0 0 0 0 /1 1 1 0 /0 0 0 0 /0 1 1 1 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /",
		// 3x2: `?` clue, every anumsub mark and every line token.
		"pzprv3/blindrush/2/3/- 1 . /. . 2 /1 0 /0 1 /1 0 1 /c . + /- = % /1 0 /0 -1 /1 0 0 /",
		// 1x1 has one empty vertical-border line and no horizontal-border line.
		"pzprv3/blindrush/1/1/. //. /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const cx = cell.bx >> 1;
			const cy = cell.by >> 1;
			const max = Math.max(
				cx,
				cy,
				p.board.cols - 1 - cx,
				p.board.rows - 1 - cy
			);
			const x = r.rnd();
			cell.qnum = -1;
			if (x < 0.12) {
				cell.qnum = -2;
			} else if (x < 0.55) {
				cell.qnum = Math.floor(r.rnd() * (max + 1));
			}
			// One FileIO token per cell: never set two of anum / qcmp / qsub.
			cell.anum = -1;
			cell.qcmp = 0;
			cell.qsub = 0;
			const y = r.rnd();
			if (y < 0.1) {
				cell.anum = r.pick([0, 1, 2, 3]);
			} else if (y < 0.2) {
				cell.qcmp = 1;
			} else if (y < 0.3) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.ques = x < 0.3 ? 1 : 0;
			border.line = 0;
			border.qsub = 0;
			const y = r.rnd();
			if (y < 0.25) {
				border.line = 1;
			} else if (y < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

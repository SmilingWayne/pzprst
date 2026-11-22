// Fixture inputs for pzprjs pid "tjunction". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tjunction",
	dataset: null,
	urls: [
		// Canonical puzzles with clues 0-4, blanks and `?`.
		"tjunction/4/4/9708.6470d",
		"tjunction/5/3/g64.7859c",
		"tjunction/2/2/4e",
		"https://puzz.link/p?tjunction/4/4/9708.6470d",
		"http://pzv.jp/p.html?type=player&tjunction/5/3/g64.7859c",
		// Smallest and largest clue values, and the 1/1, 1/N, N/1 boards.
		"tjunction/1/1/0",
		"tjunction/1/1/4",
		"tjunction/1/4/4.0b",
		"tjunction/4/1/123e",
		// Blank board, `?`, run markers, trailing junk and a truncated body.
		"tjunction/3/3/o",
		"tjunction/3/3/.",
		"tjunction/3/3/zzzz",
		"tjunction/4/4/9708.6470dzz",
		"tjunction/4/4/9708.64",
		// Unknown flag (dropped from the canonical URL).
		"tjunction/x/3/3/o"
	],
	fileios: [
		// Question and answer data; border lines and crosses in both layers.
		"pzprv3/tjunction/3/3/4 . 2 /0 3 . /- 1 4 /0 0 0 0 /0 1 0 0 /0 0 0 0 /0 0 0 /0 0 0 /0 -1 0 /0 0 0 /0 2 0 /0 1 0 /0 0 0 /",
		"pzprv3/tjunction/3/2/4 . /2 0 /3 - /0 0 0 /0 0 0 /0 0 0 /0 0 /0 0 /0 0 /0 0 /0 0 /0 0 /0 0 /",
		// Every empty: no clue, line or sub mark.
		"pzprv3/tjunction/2/2/. . /. . /0 0 0 /0 0 0 /0 0 /0 0 /0 0 /0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
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

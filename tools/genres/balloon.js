// Fixture inputs for pzprjs pid "balloon". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "balloon",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"balloon/10/10/o10dfcfc0c606u6um0g3h3j6h3zn2g8h12n21h4g1zn3h2j2h",
		"balloon/14/14/000v7pufjsv7pufjs000003sv7pufjsv7pufg000cj-10mcm4zzzn5i2gfhag3iazzzn3m1m3j4",
		"https://puzz.link/p?balloon/10/10/o10dfcfc0c606u6um0g3h3j6h3zn2g8h12n21h4g1zn3h2j2h",
		"http://pzv.jp/p.html?type=player&balloon/4/4/0000cu",
		// The `a` flag (adjacency allowed) on and off.
		"balloon/a/2/2/8g",
		"balloon/2/2/8g",
		// Ice cells, clues including the largest 4x4 value and a question mark.
		"balloon/4/4/0000cu",
		"balloon/2/2/02i",
		"balloon/1/1/g",
		"balloon/2/2/02c",
		"balloon/1/4/0f0f",
		"balloon/4/1/0f0f",
		// Blank runs and a number layer that stops early.
		"balloon/3/3/00zz",
		"balloon/3/3/0012c",
		// Truncated body, junk after the last cell, unknown pflag.
		"balloon/3/3/00",
		"balloon/3/3/00ZZ",
		"balloon/f/3/3/000"
	],
	fileios: [
		// Every cell token, both flag states, answer and line layers.
		"pzprv3/balloon/2/2/a/# - /1 2 /1 /2 /0 1 /0 /0 /0 0 /",
		"pzprv3/balloon/2/2/# - /1 2 /1 /2 /0 1 /0 /0 /0 0 /",
		"pzprv3/balloon/3/2/a/# . 3 /- 2 . /1 -1 /0 1 /0 /1 0 /0 -1 /",
		// Empty board: no clue and no answer data.
		"pzprv3/balloon/1/2/a/. . /0 0 /0 /0 /"
	],
	randomize(p, r) {
		// `Cell.maxnum` is a function of the board size.
		const maxnum = Math.max(
			0,
			p.board.cols * p.board.rows - Math.min(p.board.cols, p.board.rows)
		);
		const clues = [-1, -1, -1, -1, -1, -2];
		const top = Math.min(maxnum, 6);
		for (let n = 1; n <= top; n++) {
			clues.push(n);
		}
		if (maxnum > top) {
			clues.push(maxnum);
		}
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.ques = 0;
			cell.qnum = -1;
			if (x < 0.2) {
				// Ice cells store nothing else: FileIO writes "#" and reads it back.
				cell.ques = 6;
			} else if (x < 0.35) {
				cell.qnum = -2;
			} else if (x < 0.7) {
				cell.qnum = r.pick(clues);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qans = 0;
			border.qsub = 0;
			border.line = 0;
			if (x < 0.15) {
				border.qans = 1;
			} else if (x < 0.25) {
				border.qsub = 1;
			} else if (x < 0.35) {
				border.qans = 1;
				border.qsub = 1;
			} else if (x < 0.6) {
				border.line = 1;
			}
		});
		p.setConfig("balloon_adjacent", r.rnd() < 0.3);
	}
};

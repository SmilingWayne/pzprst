// Fixture inputs for pzprjs pid "orbital". See tools/gen_fixtures.js for the format.
// nagenawa.js also serves nagenawa/ringring, which override Encode/FileIO and are
// separate pids. There is no Orbital folder under ../puzzlekit-dataset/assets/data.
"use strict";

module.exports = {
	pid: "orbital",
	dataset: null,
	urls: [
		// Real 6x6 puzzle from pzprjs test/script/orbital.js; canonicalises to itself.
		"orbital/6/6/88o100s0t4s.m",
		"https://puzz.link/p?orbital/6/6/88o100s0t4s.m",
		"http://pzv.jp/p.html?type=player&orbital/6/6/88o100s0t4s.m",
		"orbital_edit/6/6/88o100s0t4s.m",
		// 1x1, 1xN and Nx1 boards.
		"orbital/1/1/0",
		"orbital/1/3/0",
		"orbital/3/1/0",
		// Truncated body, trailing junk, empty body.
		"orbital/4/4/1",
		"orbital/6/6/88o100s0t4s.mzz",
		"orbital/3/3/",
		"orbital/6/6",
		// `?` clues (`.` in the number layer), after the ice layer.
		"orbital/2/2/0..",
		"orbital/2/2/0.g",
		// Hex escapes: `-`/`+` run into the base-36 run marker that follows.
		"orbital/2/2/0-1f",
		"orbital/2/2/0+1f",
		// An invalid ice character decodes as an all-zero base-32 digit.
		"orbital/2/2/w0",
		// A long run of shaded cells (the ice layer is only partly supplied).
		"orbital/5/5/f"
	],
	fileios: [
		// The real 6x6 puzzle, answered: some orbit lines and one cross.
		"pzprv3/orbital/6/6/. # . . . . /# . . . # # /. . 4 . . . /. # . . . . /. . . . - . /# # # . . . /1 0 0 0 0 /0 1 1 0 0 /0 0 0 1 0 /0 1 0 0 0 /0 0 0 0 1 /0 0 0 0 0 /1 1 0 0 0 0 /0 1 1 0 0 -1 /0 0 0 0 0 0 /0 0 1 0 0 0 /0 0 0 0 0 0 /",
		// 2x2 with a `?` clue, a shaded cell, a maximum clue and an answer.
		"pzprv3/orbital/2/2/# - /4 . /1 /0 /0 1 /1 0 /",
		// 1x1 has no inner borders; only the cell line and one empty border line.
		"pzprv3/orbital/1/1/0 //"
	],
	randomize(p, r) {
		const max = (p.board.cols + p.board.rows - 2) * 2;
		p.board.cell.each((cell) => {
			cell.qnum = -1;
			cell.ques = 0;
			const x = r.rnd();
			if (x < 0.18) {
				cell.ques = 6;
			} else if (x < 0.33) {
				cell.qnum = -2;
			} else if (x < 0.86) {
				cell.qnum = Math.floor(r.rnd() * (max + 1));
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

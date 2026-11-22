// Fixture inputs for pzprjs pid "dbchoco". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "dbchoco",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"dbchoco/12/10/us4l1d5p0f9n0bg371rpvjv0k3k3zw7g7zzzn",
		"dbchoco/14/10/07utq7bv4cja16t323eni9ohng37zoco6zzy3zz",
		"dbchoco/12/10/3g0s07f1rrgus07efrjgss77g2h4i6j3l3n2l1s3u2i1zw4m4i",
		"https://puzz.link/p?dbchoco/12/10/us4l1d5p0f9n0bg371rpvjv0k3k3zw7g7zzzn",
		"http://pzv.jp/p.html?type=player&dbchoco/4/4/00008u",
		// The largest clue of a 4x4 board, ice cells with and without a clue,
		// and a question mark in the number layer. A `0` in the number layer
		// (clue 0) is not used: FileIO writes it as the `?` token.
		"dbchoco/4/4/00008u",
		"dbchoco/2/2/g.",
		"dbchoco/2/2/g5i",
		"dbchoco/1/1/g.",
		"dbchoco/1/1/0",
		"dbchoco/1/4/g5",
		"dbchoco/4/1/g5",
		// Base-36 blank runs and the empty body.
		"dbchoco/3/3/00zz",
		"dbchoco/3/3/0012c",
		"dbchoco/2/2/",
		// Truncated body, junk after the last cell, unknown pflag.
		"dbchoco/3/3/00",
		"dbchoco/3/3/00ZZ",
		"dbchoco/f/3/3/00"
	],
	fileios: [
		// Every cell token (`-` ice, `0` question mark, decimal clues) and
		// every border answer token.
		"pzprv3/dbchoco/2/2/-5 . /0 - /1 /2 /0 1 /",
		"pzprv3/dbchoco/3/2/-8 1 /0 -4 /. . /1 -1 /0 1 /0 /1 0 /0 -1 /",
		// Empty board: no clue and no answer data.
		"pzprv3/dbchoco/1/2/. . /0 /"
	],
	randomize(p, r) {
		// `Cell.maxnum` is a function of the board size.
		const maxnum = (p.board.cols * p.board.rows) >> 1;
		const clues = [-1, -1, -1, -1, -2];
		const top = Math.min(maxnum, 6);
		for (let n = 1; n <= top; n++) {
			clues.push(n);
		}
		if (maxnum > top) {
			clues.push(maxnum);
		}
		p.board.cell.each((cell) => {
			// FileIO stores both properties (`-5` is an ice cell with clue 5).
			cell.ques = r.rnd() < 0.25 ? 6 : 0;
			cell.qnum = r.pick(clues);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qans = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.qans = 1;
			} else if (x < 0.3) {
				border.qsub = 1;
			} else if (x < 0.4) {
				border.qans = 1;
				border.qsub = 1;
			}
		});
	}
};

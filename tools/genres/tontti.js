// Fixture inputs for pzprjs pid "tontti". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tontti",
	dataset: null,
	urls: [
		// Canonical puzzles; the 4x4 board uses the largest clue (`maxnum` = 16).
		"tontti/4/4/-10g2g1fg.1g42g13g",
		"tontti/5/3/gfg4.2g3g6g4g2g",
		"https://puzz.link/p?tontti/6/6/4g1g1s3g1g1s",
		"http://pzv.jp/p.html?type=player&tontti/5/3/gfg4.2g3g6g4g2g",
		// Smallest clue, the 1/1, 1/N and N/1 boards.
		"tontti/1/1/1",
		"tontti/1/4/4.13",
		"tontti/4/1/1234",
		// Blank board, `?`, run markers, trailing junk and a truncated body.
		"tontti/3/3/o",
		"tontti/3/3/.",
		"tontti/3/3/zzzz",
		"tontti/4/4/-10g2g1fg.1g42g13gzz",
		"tontti/4/4/-10g2g1fg.1g42",
		// Unknown flag (dropped from the canonical URL).
		"tontti/x/2/2/14"
	],
	fileios: [
		// Question and answer data; border lines and crosses in both layers.
		"pzprv3/tontti/4/4/16 . 2 . /1 15 . - /1 . 4 2 /. 1 3 . /0 1 0 0 0 /0 0 0 0 0 /0 0 -1 0 0 /0 0 0 0 0 /0 0 0 0 /0 1 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /1 0 0 0 /0 2 0 0 /0 0 0 0 /0 0 0 0 /",
		"pzprv3/tontti/3/2/. 2 /4 - /1 . /0 0 0 /0 0 0 /0 0 0 /0 0 /0 0 /0 0 /0 0 /0 0 /0 0 /0 0 /",
		// Every empty: no clue, line or sub mark.
		"pzprv3/tontti/2/2/. . /. . /0 0 0 /0 0 0 /0 0 /0 0 /0 0 /0 0 /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 1, 2, 3, Math.min(maxnum, 9), maxnum, -2]);
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

// Fixture inputs for pzprjs pid "squarejam". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "squarejam",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"squarejam/19/5/zk3q3i1k2k2i2q2zk",
		"squarejam/11/11/zg1w333n3l1k3l3n221w2zg",
		"squarejam/4/4/h1s",
		"https://puzz.link/p?squarejam/11/11/zg1w333n3l1k3l3n221w2zg",
		"http://pzv.jp/p.html?type=player&squarejam/11/11/zg1w333n3l1k3l3n221w2zg",
		"squarejam/11/11/zg1w333n3l1k3l3n221w2zg/",
		"squarejam/4/4/1zz",
		"squarejam/11/11/zg1w333n3l1k",
		"squarejam/4/4/.",
		"squarejam/4/4/+3e7",
		"squarejam/2/2/-ff",
		"squarejam/1/1/1",
		"squarejam/1/5/1",
		"squarejam/5/1/-19",
		"squarejam/4/4",
		"squarejam/3/3/z1",
		"squarejam/4/4/1G",
		"squarejam/x/4/4/1",
		"squarejam/f/4/4/1"
	],
	fileios: [
		// Real 4x4 clue with answer borders and crosses.
		"pzprv3/squarejam/4/4/. . 1 . /. . . . /. . . . /. . . . /0 1 2 /-1 0 0 /0 0 1 /0 0 0 /0 2 0 0 /1 0 -1 0 /0 0 0 1 /",
		// Question mark, both clue sizes and every answer token.
		"pzprv3/squarejam/2/3/. - 2 /1 2 . /2 1 /-1 0 /1 2 -1 /",
		// Empty board: no clue and no answer data.
		"pzprv3/squarejam/3/3/. . . /. . . /. . . /0 0 /0 0 /0 0 /0 0 0 /0 0 0 /"
	],
	randomize(p, r) {
		const maxnum = Math.min(p.board.cols, p.board.rows);
		const clues = [-1, -1, -1, -1, -1, 1, 2, -2];
		for (let n = 3; n <= Math.min(maxnum, 12); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
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

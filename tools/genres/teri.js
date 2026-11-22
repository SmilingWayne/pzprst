// Fixture inputs for pzprjs pid "teri". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "teri",
	dataset: null,
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"teri/8/8/j23p3m4t3m4p23j",
		"teri/18/10/4j6l6j2h6k89k3l3n6q4h8n4t6h6t8n6h3q6n3l4k43k4h2j4l3j6",
		"https://puzz.link/p?teri/4/4/3h6q4",
		"teri/4/4/3h6q4",
		"teri/4/4/3h6q4/",
		"teri/4/4/3h6q4zz",
		"teri/4/4/3h",
		"teri/1/1/g",
		"teri/1/1/0",
		"teri/1/5/g",
		"teri/5/1/g",
		"teri/4/4/-10",
		"teri/4/4/+64",
		"teri/2/2/h",
		"teri/3/3/HZ",
		"teri/9/9/=001",
		"teri/30/1/g",
		"teri/1/25/k",
		"teri/x/4/4/g",
		"teri/4/4"
	],
	fileios: [
		// Real 4x4 answer board from pzprjs test/script/teri.js (cell marks only).
		"pzprv3/teri/4/4/3 . . 6 /. . . . /. . . . /. . . 4 /. # + . /+ + + + /+ + + + /# + # . /",
		// Border answer layer with 1 / 2 / -1 marks, plus cell marks.
		"pzprv3/teri/4/4/3 . . 6 /. . . . /. . . . /. . . 4 /. . + . /# . + . /# . + . /# . + . /1 -1 2 /1 -1 2 /1 -1 2 /1 -1 2 /1 -1 2 1 /-1 2 1 -1 /2 1 -1 2 /",
		// 2x5 board: two-digit clues, cell marks and every border token.
		"pzprv3/teri/2/5/- 2 3 4 5 /6 . 6 5 4 /+ . . . . /+ # . . . /-1 0 2 1 /0 0 0 0 /0 1 0 -1 0 /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 2; n <= Math.min(maxnum, 6); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			const x = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.35) {
				cell.qsub = 1;
			}
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

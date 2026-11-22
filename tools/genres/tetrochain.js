// Fixture inputs for pzprjs pid "tetrochain". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tetrochain",
	dataset: null,
	urls: [
		// Hand-built 3x3: every direction, numbers 0-8, `?` and `+`.
		"tetrochain/3/3/101112131415161718",
		"https://puzz.link/p?tetrochain/3/3/101112131415161718",
		"https://puzz.link/p?tetrochain_edit/3/3/101112131415161718",
		"http://pzv.jp/p.html?type=player&tetrochain/3/3/101112131415161718",
		// `?` with a direction, `+`, and an unknown flag.
		"tetrochain/3/3/1.",
		"tetrochain/3/3/+",
		"tetrochain/x/3/3/10",
		// Largest allowed clue (3) on a 4x4 board.
		"tetrochain/4/4/30313233343536373839303132333435",
		// 1x1, 1/N and N/1 boards.
		"tetrochain/1/1/00",
		"tetrochain/1/5/g",
		"tetrochain/5/1/g",
		// Base-36 run lengths.
		"tetrochain/4/4/v",
		// Two- and three-character numbers.
		"tetrochain/3/3/510",
		"tetrochain/3/3/-1100",
		// Truncated body, trailing junk and empty bodies.
		"tetrochain/3/3/1011",
		"tetrochain/3/3/101112131415161718zz",
		"tetrochain/3/3",
		"tetrochain/3/3/",
		// Real puzzle URLs (puzz.link).
		"https://puzz.link/p?tetrochain/8/8/m10c12b22e10j42e42b20c44m",
		"https://puzz.link/p?tetrochain/10/10/m242111v11b21f34b12f23b31v344312m",
		"https://puzz.link/p?tetrochain/9/9/c33d37k32d35k31d32k22d41t34"
	],
	fileios: [
		// 3x3 with clues, `?`, `+` and the answer layer.
		"pzprv3/tetrochain/3/3/1,0 1,1 1,2 /1,3 1,4 1,5 /1,6 1,7 1,8 /# + . /# . + /. . . /",
		// 2x2 with `?`, `+` and every answer mark.
		"pzprv3/tetrochain/2/2/0,1 2,- /# 3,4 /+ # /. + /",
		// 1x1 with a direction and no answer data.
		"pzprv3/tetrochain/1/1/1,0 /./"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows) - 1;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qdir = 0;
			if (x < 0.4) {
				cell.qnum = -1;
			} else if (x < 0.5) {
				cell.qnum = -2;
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			} else if (x < 0.6) {
				cell.qnum = -3;
			} else {
				cell.qnum = Math.floor(r.rnd() * (maxnum + 1));
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			}
			const a = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (a < 0.2) {
				cell.qans = 1;
			} else if (a < 0.35) {
				cell.qsub = 1;
			}
		});
	}
};

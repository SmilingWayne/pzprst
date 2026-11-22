// Fixture inputs for pzprjs pid "nuribou". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "nuribou",
	dataset: null,
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"nuribou/10/10/3h4v5j5s6i7zkd2h7zg7i1",
		"nuribou/10/10/i6k7r2m1k7l4k4zhakbl4m6l",
		"nuribou/10/10/k8v3j4h2x6g2h9w6l3lbl5l",
		"https://puzz.link/p?nuribou/10/10/3h4v5j5s6i7zkd2h7zg7i1",
		"http://pzv.jp/p.html?type=player&nuribou/10/10/3h4v5j5s6i7zkd2h7zg7i1",
		"nuribou/10/10/3h4v5j5s6i7zkd2h7zg7i1/",
		"nuribou/10/10/3h4v5j5s6i7zkd2h7zg7i1zz",
		"nuribou/10/10/3h4v",
		"nuribou/3/2/1g2g.3",
		"nuribou/3/3/1.2",
		"nuribou/4/4/",
		"nuribou/1/1/1",
		"nuribou/1/1/.",
		"nuribou/1/1/",
		"nuribou/1/5/12345",
		"nuribou/5/1/12345",
		"nuribou/4/4/+010",
		"nuribou/f/4/4/1234",
		"nuribou/3/3/zzz",
		"nuribou/6/6"
	],
	fileios: [
		"pzprv3/nuribou/2/3/1 . 2 /. - 3 /# . + /. # . /",
		"pzprv3/nuribou/2/2/. . /. - /. + /. # /",
		"pzprv3/nuribou/1/1/1 /+ /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 1; n <= Math.min(maxnum, 8); n++) {
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
	}
};

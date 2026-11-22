// Fixture inputs for pzprjs pid "sananko". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "sananko",
	dataset: null,
	urls: [
		"sananko/4/4/3.0n+.7n",
		"sananko/6/6/s33.3m3p3m3k3./",
		"sananko/6/6/k5.2p4s7p3.9k",
		"https://puzz.link/p?sananko/11/11/v4k4k6lap7m5l7x7lal2x3l5m6p7l5k4k9v",
		"http://pzv.jp/p.html?type=player&sananko/4/4/cx",
		"sananko/3/3/",
		"sananko/1/1/1",
		"sananko/1/1/_",
		"sananko/2/2/12",
		"sananko/3/3/1.2",
		"sananko/3/3/1.2zz",
		"sananko/1/3/2",
		"sananko/3/1/2",
		"sananko/3/3/++++",
		"sananko/4/4/c",
		"sananko/10/10/"
	],
	fileios: [
		"pzprv3/sananko/4/4/3 . 0 . /. . . . /b . 7 . /. . . . /. 0 . 0 /3 3 0 0 /. 3 . 2 /0 0 2 2 /",
		"pzprv3/sananko/4/4/3 . 0 . /. . . . /b . 7 . /. . . . /. 0 . 0 /0 0 0 0 /. 0 . 0 /0 0 0 0 /",
		"pzprv3/sananko/2/2/? . /5 . /. 0 /. + /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 0, 1, 2, 3, 7, 12, -2]);
			cell.ques = cell.qnum === -1 && r.rnd() < 0.15 ? 1 : 0;
			cell.anum = r.pick([-1, -1, -1, 1, 2, 3]);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
		});
	}
};

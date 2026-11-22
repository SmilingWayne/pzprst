// Fixture inputs for pzprjs pid "kropki". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kropki",
	dataset: null,
	urls: [
		"kropki/4/4/aa7fioco",
		"kropki/6/6/f39e0359520000000000",
		"kropki/6/6/d119a4d331330a1d3991",
		"https://puzz.link/p?kropki/7/7/60i0600k0602606ioi00o0202606",
		"http://pzv.jp/p.html?type=player&kropki/2/2/20",
		"kropki/3/3/",
		"kropki/1/1/",
		"kropki/2/2/qq",
		"kropki/2/2/00",
		"kropki/3/3/qqqq",
		"kropki/2/2/qqzz",
		"kropki/2/2/A0",
		"kropki/1/3/",
		"kropki/3/1/",
		"kropki/4/4/0000",
		"kropki/10/10/",
		"kropki/2/2/20"
	],
	fileios: [
		"pzprv3/kropki/4/4/1 a 1 /b 0 1 /0 2 1 /1 2 0 /2 0 b 2 /2 0 1 1 /0 2 . 0 /4 3 1 2 /2 1 3 4 /1 4 2 3 /3 2 4 1 /",
		"pzprv3/kropki/4/4/1 a 1 /b 0 1 /0 2 1 /1 2 0 /2 0 b 2 /2 0 1 1 /0 2 . 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /",
		"pzprv3/kropki/2/2/a /b /2 0 /1 2 /. 1 /"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows);
		p.board.border.each((border) => {
			border.qnum = r.pick([-1, -1, -1, 1, 2, -2]);
			border.qdir = r.pick([0, 0, 0, 0, 1, 2, 3, 4]);
		});
		p.board.cell.each((cell) => {
			cell.anum = r.pick([-1, -1, -1].concat([1, 2, 3, 4].filter((n) => n <= maxnum)));
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
		});
	}
};

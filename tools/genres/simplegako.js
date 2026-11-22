// Fixture inputs for pzprjs pid "simplegako". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "simplegako",
	dataset: null,
	urls: [
		// Real puzzles, verified as already-canonical with tools/pzprjs.js.
		"simplegako/6/6/j7g1l1l3l4l4g",
		"simplegako/5/5/h6m2g5g2m6h",
		"https://puzz.link/p?simplegako/6/6/j7g1l1l3l4l4g",
		"https://puzz.link/p?simplegako_edit/6/6/j7g1l1l3l4l4g",
		"http://pzv.jp/p.html?type=player&simplegako/5/5/h6m2g5g2m6h",
		"simplegako/6/6/j7g1l1l3l4l4g/",
		"simplegako/6/6/j7g1l1l3l4l4gzz",
		"simplegako/6/6/j7g1l1l3l4l4",
		"simplegako/4/4",
		"simplegako/4/4/",
		"simplegako/4/4/.",
		"simplegako/1/1/1",
		"simplegako/1/1/.",
		"simplegako/1/5/1",
		"simplegako/5/1/1",
		// maxnum is cols+rows-1: 5 on a 5x1 board, 2 on 1x2.
		"simplegako/5/1/5",
		"simplegako/1/2/2",
		"simplegako/2/2/-19",
		"simplegako/2/2/+100",
		"simplegako/2/2/7",
		"simplegako/3/3/zz",
		"simplegako/3/3/A",
		"simplegako/2/2/=001",
		"simplegako/2/2/@001",
		"simplegako/2/2/*0001",
		"simplegako/2/2/$00001",
		"simplegako/30/1/zzk",
		"simplegako/1/30/zp",
		"simplegako/x/4/4/1"
	],
	fileios: [
		// Question numbers plus anum / qsub / qcmp answer marks.
		"pzprv3/simplegako/3/3/1 2 . /3 4 . /. . 7 /. . . /2 - . /. . c /",
		"pzprv3/simplegako/2/2/1 2 /3 - /4 . /. + /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols + p.board.rows - 1;
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 1; n <= Math.min(maxnum, 6); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			const a = r.rnd();
			if (a < 0.12) {
				cell.anum = 1 + Math.floor(r.rnd() * maxnum);
			} else if (a < 0.18) {
				cell.qcmp = 1;
			} else if (a < 0.3) {
				cell.qsub = 1 + Math.floor(r.rnd() * 4);
			}
		});
	}
};

// Fixture inputs for pzprjs pid "mirrorbk". See tools/gen_fixtures.js for the format.
// "cbblock", "dbchoco" and "nikoji" share pzprjs's variety/cbblock.js but override
// Encode/FileIO, so they are separate pids and are not covered here.
"use strict";

module.exports = {
	pid: "mirrorbk",
	dataset: null,
	urls: [
		"mirrorbk/4/4/1g",
		"https://puzz.link/p?mirrorbk/8/8/3h2n3h5i1k4h3l3h1k2i2h2n5h420g410820g4000001vo00000",
		"https://puzz.link/p?mirrorbk/6/6/l1l2l3l4l5g0000i0024418",
		"mirrorbk/6/6/l1l2l3l4l5g0000i0024418",
		"mirrorbk_edit/4/4/1g",
		"http://pzv.jp/p.html?type=player&mirrorbk/4/4/1g",
		"mirrorbk/4/4/0001",
		"mirrorbk/4/4/1gzz",
		"mirrorbk/4/4",
		"mirrorbk/4/4/",
		"mirrorbk/4/4//",
		"mirrorbk/1/1",
		"mirrorbk/1/1/-",
		"mirrorbk/2/1/g",
		"mirrorbk/1/2/g",
		"mirrorbk/4/4/z",
		"mirrorbk/30/1/z2",
		"mirrorbk/1/30/zp",
		"mirrorbk/x/4/4/1g",
		"mirrorbk/2/2/--1",
		"mirrorbk/16/16/0000000000000000000000000000",
		"mirrorbk/16/16/+100",
		"mirrorbk/9/9/-50",
		"mirrorbk/3/3/123456789"
	],
	fileios: [
		"pzprv3/mirrorbk/3/3/1 2 3 /4 5 6 /7 8 9 /3 0 /1 -1 /-2 2 /1 2 3 /0 3 -1 /",
		"pzprv3/mirrorbk/2/2/- . /. 4 /-1 /2 /3 -2 /",
		"pzprv3/mirrorbk/1/1/- //"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.55) {
				cell.qnum = -1;
			} else if (x < 0.7) {
				cell.qnum = -2;
			} else {
				cell.qnum = 1 + Math.floor(r.rnd() * maxnum);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.ques = 0;
			border.qans = 0;
			border.qsub = 0;
			if (x < 0.14) {
				border.ques = 1;
			} else if (x < 0.28) {
				border.qans = 1;
			} else if (x < 0.4) {
				border.qans = 1;
				border.qsub = 1;
			} else if (x < 0.52) {
				border.ques = 1;
				border.qsub = 1;
			} else if (x < 0.64) {
				border.qsub = 1;
			}
		});
	}
};

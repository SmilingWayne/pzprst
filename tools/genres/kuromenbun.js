// Fixture inputs for pzprjs pid "kuromenbun". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kuromenbun",
	dataset: null,
	urls: [
		"kuromenbun/7/7/a5294i52go1vg0fs0o3n2h3g4i5g2m3g4i3g2h1n0=",
		"https://puzz.link/p?kuromenbun/8/8/i4h4o61i8i4gg07n000eu00g2g5h5n5h2l1h6n6h4l4i2l2i2g0=",
		"https://puzz.link/p?kuromenbun/9/9/4gi289144gi2890se00e73u73g03hoi2m2q3h63h4h3i2k2i3h4h37h3q8m4i=",
		"kuromenbun/4/4/ggg0000b.h5o3",
		"https://puzz.link/p?kuromenbun/4/4/ggg0000b.h5o3",
		"http://pzv.jp/p.html?type=player&kuromenbun/4/4/ggg0000b.h5o3",
		"kuromenbun/z/4/4/ggg0000b.h5o3",
		"kuromenbun/3/3/00006i.i0",
		"kuromenbun/2/2/002i",
		"kuromenbun/1/1/1",
		"kuromenbun/1/5/g3i0",
		"kuromenbun/5/1/8h7h",
		"kuromenbun/4/4/g",
		"kuromenbun/4/4/",
		"kuromenbun/2/2/0",
		"kuromenbun/4/4/ggg0000b.h5o3zz",
		"kuromenbun/1/1/+3e7"
	],
	fileios: [
		"pzprv3/kuromenbun/4/4/1 0 0 /0 0 1 /0 0 0 /0 1 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 11 - . /. 5 . . /. . . . /. . . 3 /# + . . /. . # + /. . . . /. . . . /",
		"pzprv3/kuromenbun/3/3/0 0 /0 0 /0 0 /0 0 0 /0 0 0 /6 . . /. - . /. . 0 /. . # /+ . . /. . . /",
		"pzprv3/kuromenbun/1/1//1 /# /",
		"pzprv3/kuromenbun/2/2/0 /0 /0 0 /2 . /. . /. . /. . /"
	],
	randomize(p, r) {
		const maxnum = Math.round((p.board.cols * p.board.rows) / 1.5);
		const pool = [-1, -1, -1, -2, 0];
		for (let n = 1; n <= maxnum; n++) {
			pool.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(pool);
			const x = r.rnd();
			if (x < 0.25) {
				cell.qans = 1;
			} else if (x < 0.4) {
				cell.qsub = 1;
			}
		});
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.3 ? 1 : 0;
		});
	}
};

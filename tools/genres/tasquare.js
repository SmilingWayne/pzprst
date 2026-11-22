// Fixture inputs for pzprjs pid "tasquare". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tasquare",
	dataset: null,
	urls: [
		// real 10x10 puzzles (puzz.link, re-canonicalised unchanged by pzprjs)
		"https://puzz.link/p?tasquare/10/10/.n.j5l.h5l3k8t...j.p.x4g.j.g.g",
		"https://puzz.link/p?tasquare/10/10/h2i.h.v6o.i8.j.l4j41i.o.v4h5i2h",
		// canonical small board and full puzz.link / pzv.jp URLs
		"tasquare/4/4/1u",
		"https://puzz.link/p?tasquare/4/4/1u",
		"http://pzv.jp/p.html?type=player&tasquare/4/4/1u",
		// largest clue pzprjs allows (maxnum 999, written as the "+" escape)
		"tasquare/4/4/+3e7",
		// 1/1, 1/N and N/1 boards
		"tasquare/1/1/1",
		"tasquare/1/5/12345",
		"tasquare/5/1/12345",
		// question mark
		"tasquare/4/4/.",
		// empty body
		"tasquare/4/4/",
		// truncated body
		"tasquare/3/3/12",
		// trailing junk
		"tasquare/4/4/1zz",
		// unknown pflag, ignored by pzprjs
		"tasquare/x/4/4/1",
		// run-length fill up to the last cell
		"tasquare/10/10/0000"
	],
	fileios: [
		// question clues plus the shading "#" and sub mark "+" layers
		"pzprv3/tasquare/2/2/1 . /# + /",
		// the largest clue 999, "?" and both answer marks
		"pzprv3/tasquare/3/3/1 2 3 /4 - . /999 + # /",
		// a completely empty layer
		"pzprv3/tasquare/1/1/. /"
	],
	randomize(p, r) {
		// Cell.maxnum keeps pzprjs's default 999.
		const values = [-1, -1, -1, -1, -1, -2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 16, 99, 999];
		p.board.cell.each((cell) => {
			const qnum = r.pick(values);
			cell.qnum = qnum;
			// encodeCellQnumAns writes the number / "?" before the answer marks,
			// so only unnumbered cells may carry one of them.
			if (qnum === -1) {
				const x = r.rnd();
				if (x < 0.25) {
					cell.qans = 1;
				} else if (x < 0.32) {
					cell.qsub = 1;
				}
			}
		});
	}
};

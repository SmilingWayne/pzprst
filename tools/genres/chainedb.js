// Fixture inputs for pzprjs pid "chainedb". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "chainedb",
	dataset: null,
	urls: [
		// real 10x10 puzzles (puzz.link, re-canonicalised unchanged by pzprjs)
		"https://puzz.link/p?chainedb/10/10/4l3k4l4j4l3l.j4m3i.i3w3z3",
		"chainedb/10/10/3m4q3j2m3l2k3x4q.q52h.l",
		"https://puzz.link/p?chainedb/10/10/p4h4h4j4w4g4t2k4l42l4m3n4k",
		// full pzv.jp player URL
		"http://pzv.jp/p.html?type=player&chainedb/4/4/1234",
		// largest clue a 4x4 board allows (maxnum = 16 - 3 = 13 = 0xd)
		"chainedb/4/4/d",
		// 1/1, 1/N and N/1 boards
		"chainedb/1/1/1",
		"chainedb/1/5/12345",
		"chainedb/5/1/12345",
		// question mark
		"chainedb/4/4/.",
		// empty body
		"chainedb/4/4/",
		// truncated body
		"chainedb/3/3/12",
		// trailing junk
		"chainedb/4/4/1zz",
		// unknown pflag, ignored by pzprjs
		"chainedb/x/4/4/1234",
		// run-length fill up to the last cell, a run past the last cell, a two-digit run
		"chainedb/10/10/0000",
		"chainedb/2/1/q",
		"chainedb/6/6/1g"
	],
	fileios: [
		// question clues plus the shading "#" and sub mark "+" layers
		"pzprv3/chainedb/2/2/1 . /# + /",
		// every clue value the 3x3 board allows (maxnum 6), "?" and both answer marks
		"pzprv3/chainedb/3/3/1 2 3 /4 5 6 /- + # /",
		// a completely empty layer
		"pzprv3/chainedb/1/1/. /"
	],
	randomize(p, r) {
		// Cell.maxnum: this.board.cols * this.board.rows - 3.
		const maxnum = p.board.cols * p.board.rows - 3;
		const values = [-1, -1, -1, -1, -1, -2];
		for (let v = 1; v <= maxnum; v++) {
			values.push(v);
		}
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

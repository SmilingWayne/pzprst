// Fixture inputs for pzprjs pid "mrtile". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "mrtile",
	dataset: null,
	urls: [
		// canonical form
		"mrtile/4/4/12345678n",
		// the same board through a full puzz.link URL and a pzv.jp player URL
		"https://puzz.link/p?mrtile/4/4/12345678n",
		"http://pzv.jp/p.html?type=player&mrtile/4/4/12345678",
		// largest clue a 4x4 board allows (maxnum = (16 >> 1) - 1 = 7)
		"mrtile/4/4/7",
		// every cell carries a clue
		"mrtile/4/4/1234567890123456",
		// 1/1, 1/N and N/1 boards
		"mrtile/1/1/1",
		"mrtile/1/5/12345",
		"mrtile/5/1/12345",
		// question mark
		"mrtile/4/4/.",
		// empty body
		"mrtile/4/4/",
		// truncated body
		"mrtile/3/3/12",
		// trailing junk
		"mrtile/4/4/1zz",
		// unknown pflag, ignored by pzprjs
		"mrtile/x/4/4/1234",
		// run-length fill up to the last cell, a run past the last cell, a two-digit run
		"mrtile/10/10/0000",
		"mrtile/2/1/q",
		"mrtile/6/6/1g"
	],
	fileios: [
		// question clues plus the shading "#" and sub mark "+" layers
		"pzprv3/mrtile/2/2/1 . /# + /",
		// every clue value the 3x3 board allows (maxnum 3), "?" and both answer marks
		"pzprv3/mrtile/3/3/1 2 3 /3 2 1 /- + # /",
		// a completely empty layer
		"pzprv3/mrtile/1/1/. /"
	],
	randomize(p, r) {
		// Cell.maxnum: ((this.board.cols * this.board.rows) >> 1) - 1.
		const maxnum = ((p.board.cols * p.board.rows) >> 1) - 1;
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

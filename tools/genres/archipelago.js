// Fixture inputs for pzprjs pid "archipelago". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "archipelago",
	dataset: null,
	urls: [
		// canonical form (4x4, values 1-8, trailing blank run)
		"archipelago/4/4/12345678n",
		// the same board through a full puzz.link URL and a pzv.jp player URL
		"https://puzz.link/p?archipelago/4/4/12345678n",
		"http://pzv.jp/p.html?type=player&archipelago/4/4/12345678",
		// largest clue the 4x4 board allows (maxnum = ceil(sqrt(16)) + 1 = 5)
		"archipelago/4/4/5",
		// every cell carries a clue
		"archipelago/4/4/1234567890123456",
		// 1/1, 1/N and N/1 boards
		"archipelago/1/1/1",
		"archipelago/1/5/12345",
		"archipelago/5/1/12345",
		// question mark
		"archipelago/4/4/.",
		// empty body
		"archipelago/4/4/",
		// truncated body
		"archipelago/3/3/12",
		// trailing junk
		"archipelago/4/4/1zz",
		// unknown pflag, ignored by pzprjs
		"archipelago/x/4/4/1234",
		// run-length fill up to the last cell, and a run past the last cell
		"archipelago/10/10/0000",
		"archipelago/2/1/q"
	],
	fileios: [
		// question clues plus the shading "#" and sub mark "+" layers
		"pzprv3/archipelago/2/2/1 . /# + /",
		// every clue value the 3x3 board allows (maxnum 4), "?" and both answer marks
		"pzprv3/archipelago/3/3/1 2 3 /4 - . /# + . /",
		// a completely empty layer
		"pzprv3/archipelago/1/1/. /"
	],
	randomize(p, r) {
		// Cell.maxnum: Math.ceil(Math.sqrt(cols * rows)) + 1.
		const maxnum = Math.ceil(Math.sqrt(p.board.cols * p.board.rows)) + 1;
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

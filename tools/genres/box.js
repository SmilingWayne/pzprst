// Fixture inputs for pzprjs pid "box". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "box",
	dataset: null,
	urls: [
		"box/9/9/ala5egmtgkm1r9-11ige",
		"box/9/9/ogjm2l8pcccla7cncm",
		"box/6/6/c7g9b2ba3d5a",
		"https://puzz.link/p?box/9/9/ala5egmtgkm1r9-11ige",
		"http://pzv.jp/p.html?type=player&box/6/6/c7g9b2ba3d5a",
		"box/4/4/00000000",
		"box/4/4/00",
		"box/4/4/00000000zz",
		"box/1/1/00",
		"box/1/6/0000000",
		"box/6/1/0000000",
		"box/4/4/a0000000",
		"box/9/9/-1d",
		"box/9/9/1d00000000000000000"
	],
	fileios: [
		"pzprv3\nbox\n2\n3\n. 2 0 1 \n3 # + . \n0 . . . \n",
		"pzprv3\nbox\n4\n4\n. 10 0 4 1 \n5 # + . . \n0 . . . . \n2 . . . . \n0 . . . . \n"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.35) {
				cell.qsub = 1;
			}
		});
		p.board.excell.each((excell) => {
			// `ExCell@box.maxnum`: a top clue allows rows*(rows+1)/2, a left
			// clue cols*(cols+1)/2.
			const size = excell.by === -1 ? p.board.rows : p.board.cols;
			const max = ((size * (size + 1)) / 2) | 0;
			excell.qnum = r.pick([0, 0, 0, 0, 1, Math.floor(max / 2), max]);
		});
	}
};

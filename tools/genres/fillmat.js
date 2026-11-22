// Fixture inputs for pzprjs pid "fillmat". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "fillmat",
	dataset: null,
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"fillmat/11/10/..r3i4c1h.f4c.4d1q4c14o3f43b",
		"fillmat/8/8/o3j4i1d312k3b2e",
		"fillmat/10/12/i12f4k1b1a1f3g2b1a1d43d2a2b1g4f2a2b2k3f12i",
		"https://puzz.link/p?fillmat/11/10/..r3i4c1h.f4c.4d1q4c14o3f43b",
		"fillmat/4/4/4d1d2d3",
		"fillmat/4/4/c.l",
		"fillmat/1/1/1",
		"fillmat/1/1/.",
		"fillmat/1/5/b2b",
		"fillmat/5/1/c3a",
		"fillmat/3/3/4g4",
		"fillmat/4/4/4d1",
		"fillmat/4/4/4d1d2d3zz",
		"fillmat/4/4/",
		"http://pzv.jp/p.html?type=player&fillmat/3/3/4g4"
	],
	fileios: [
		"pzprv3\nfillmat\n2\n3\n3 . . \n. - . \n1 0 \n0 2 \n0 -1 0 \n",
		"pzprv3\nfillmat\n2\n2\n. . \n. 4 \n0 \n-1 \n0 0 \n"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, -2, 1, 2, 3, 4]);
		});
		p.board.border.each((border) => {
			border.qans = r.rnd() < 0.2 ? 1 : 0;
			border.qsub = r.rnd() < 0.1 ? 1 : 0;
		});
	}
};

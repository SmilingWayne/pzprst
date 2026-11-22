// Fixture inputs for pzprjs pid "mines". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "mines",
	dataset: "Minesweeper",
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"mines/6/6/2g1g2n1g1g3j2g3h1j3g3g",
		"mines/7/7/2k0h4g3i4i3j3j1i5i5g2h1k1",
		"mines/13/12/g2g2h2h2g2g2g3g2g3g3g3g2s2g111g2g3g3g3h101g3n101i222g22g111i202n3g202h2g4g3g4g222g3s3g4g3g4g4g2g2g2g1h2h2g2g",
		"https://puzz.link/p?mines/6/6/2g1g2n1g1g3j2g3h1j3g3g",
		"http://pzv.jp/p.html?type=player&mines/6/6/2g1g2n1g1g3j2g3h1j3g3g",
		"mines/6/6/2g1g2n1g1g3j2g3h1j3g3g/",
		"mines/6/6/2g1g2n1g1g3j2g3h1j3g3gzz",
		"mines/6/6/2g1g",
		"mines/3/2/1g2g.g",
		"mines/4/4/",
		"mines/2/2/8888",
		"mines/1/1/8",
		"mines/1/1/.",
		"mines/1/1/",
		"mines/1/5/01234",
		"mines/5/1/01234",
		"mines/f/4/4/8888",
		"mines/2/2/+008+008",
		"mines/2/2/=000@001",
		"mines/4/4"
	],
	fileios: [
		"pzprv3/mines/2/3/1 . 2 /. - . /# . + /. # . /",
		"pzprv3/mines/2/2/. . /. . /. + /. # /",
		"pzprv3/mines/1/1/8 /# /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, -2, 0, 1, 2, 3, 4, 5, 6, 7, 8]);
			const x = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.35) {
				cell.qsub = 1;
			}
		});
	}
};

// Fixture inputs for pzprjs pid "lollipops". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "lollipops",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"lollipops/11/11/d3b3n1e2c3d2h2d1h1h1d3h3d2c3e1n3c3c",
		"lollipops/8/8/1c1b1b1j1b1f1b1m1d1d1d1",
		"lollipops/7/7/g2c2j1d11b1l1c1",
		"https://puzz.link/p?lollipops/7/7/g2c2j1d11b1l1c1",
		"https://puzz.link/p?lollipops_edit/7/7/g2c2j1d11b1l1c1",
		"http://pzv.jp/p.html?type=player&lollipops/7/7/g2c2j1d11b1l1c1",
		// Largest allowed object (3) on a 1x1 board.
		"lollipops/1/1/3",
		// `?` in every shape: alone, in a row and on 1/N / N/1 boards.
		"lollipops/1/1/.",
		"lollipops/4/1/123.",
		"lollipops/1/4/1a",
		"lollipops/1/4/",
		// Base-36 run lengths and a run at the end.
		"lollipops/2/2/v",
		"lollipops/2/2/11",
		"lollipops/1/1/z",
		// Smallest legal object and a mixed 2x2 board.
		"lollipops/1/1/1",
		"lollipops/2/2/12.3",
		// Truncated body, trailing junk and empty bodies.
		"lollipops/3/3/12",
		"lollipops/3/3/123123123zz",
		"lollipops/3/3",
		"lollipops/3/3/"
	],
	fileios: [
		// Question objects 1-3 and `?`, answer objects, `cmp` and background marks.
		"pzprv3/lollipops/2/2/1 2 /3 - /1 c /+ - /",
		// 2x3 with a bar orientation, every background mark and a full answer row.
		"pzprv3/lollipops/2/3/2 - 1 /. 3 . /2 % c /= 1 + /",
		// 1x1 minimal file.
		"pzprv3/lollipops/1/1/1 /. /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qnum = -1;
			cell.anum = -1;
			cell.qcmp = 0;
			cell.qsub = 0;
			if (x < 0.55) {
				// empty
			} else if (x < 0.65) {
				cell.qnum = -2;
			} else if (x < 0.85) {
				cell.qnum = 1 + Math.floor(r.rnd() * 3);
			} else {
				cell.anum = 1 + Math.floor(r.rnd() * 3);
			}
			if (cell.qnum === -1 && r.rnd() < 0.15) {
				cell.qcmp = 1;
			}
			if (r.rnd() < 0.15) {
				cell.qsub = 1;
			}
		});
	}
};

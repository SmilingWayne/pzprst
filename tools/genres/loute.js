// Fixture inputs for pzprjs pid "loute". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "loute",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"loute/8/8/k5j53j5g2h35m3h2m52h2g5j35j5k",
		"loute/8/8/h2h4l4i5h3i1g5i5j4i4g1i5h3i5l4h1h",
		"loute/4/3/5g5l5g5",
		"https://puzz.link/p?loute/4/3/5g5l5g5",
		"https://puzz.link/p?loute_edit/4/3/5g5l5g5",
		"http://pzv.jp/p.html?type=player&loute/4/3/5g5l5g5",
		// 1x1: circle, `?`, arrow and an out-of-UI-range direction.
		"loute/1/1/5",
		"loute/1/1/.",
		"loute/1/1/2",
		"loute/1/1/f",
		// Base-36 run lengths on 1/N and N/1 boards.
		"loute/1/5/g",
		"loute/5/1/g",
		"loute/12/1/g",
		"loute/3/3/5",
		// Truncated body, trailing junk, skipped characters and empty bodies.
		"loute/3/3/2",
		"loute/4/3/5g5l5g5zz",
		"loute/1/1/@",
		"loute/3/3",
		"loute/3/3/"
	],
	fileios: [
		// 4x3 with circles, vertical/horizontal answer borders and every token.
		"pzprv3/loute/3/4/5 . 5 . /. . . . /. 5 . 5 /1 0 -1 /0 2 0 /0 0 0 /1 0 0 1 /-1 0 1 0 /",
		// 2x2 with `#` and `_` answer cells and a circle with an arrow.
		"pzprv3/loute/2/2/. # /_ 2 /1 /0 /0 0 /",
		// 1x1 minimal file.
		"pzprv3/loute/1/1/5 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qdir = 0;
			cell.qans = 0;
			cell.qsub = 0;
			if (x < 0.45) {
				// empty
			} else if (x < 0.55) {
				cell.qdir = -2;
			} else if (x < 0.7) {
				cell.qdir = 1 + Math.floor(r.rnd() * 4);
			} else if (x < 0.85) {
				cell.qdir = 5;
			} else if (r.rnd() < 0.5) {
				cell.qans = 1;
			} else {
				cell.qsub = 1;
			}
		});
		p.board.border.each((border) => {
			border.qans = 0;
			border.qsub = 0;
			const x = r.rnd();
			if (x < 0.2) {
				border.qans = 1;
			} else if (x < 0.3) {
				border.qsub = 1;
			} else if (x < 0.4) {
				border.qans = 1;
				border.qsub = 1;
			}
		});
	}
};

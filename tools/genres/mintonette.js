// Fixture inputs for pzprjs pid "mintonette". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "mintonette",
	dataset: null,
	urls: [
		// Hand-built 3x3 with clues 0-3.
		"mintonette/3/3/100200300",
		"https://puzz.link/p?mintonette/3/3/100200300",
		"https://puzz.link/p?mintonette_edit/3/3/100200300",
		"http://pzv.jp/p.html?type=player&mintonette/3/3/100200300",
		// Clue 0 and `?` on 1x1 boards.
		"mintonette/1/1/0",
		"mintonette/1/1/.",
		// Largest allowed clue (999) and the -/+ escapes.
		"mintonette/1/1/+3e7",
		"mintonette/1/1/-0f",
		"mintonette/2/2/+3e7.",
		// Base-36 run lengths, 1/N and N/1 boards.
		"mintonette/2/2/g",
		"mintonette/4/4/v",
		"mintonette/1/5/g",
		"mintonette/5/1/g",
		// Truncated body, trailing junk and empty bodies.
		"mintonette/3/3/1z",
		"mintonette/3/3/100200300zz",
		"mintonette/3/3",
		"mintonette/3/3/",
		"mintonette/4/4/.",
		// Real 8x8 and 12x12 sample URLs.
		"https://puzz.link/p?mintonette/8/8/l.21.p.5z5.p.31.l",
		"https://puzz.link/p?mintonette/12/12/s2i.j3i.i6h6y2n1h.j3h4j.hbn1y9h9i.i1j.ibs"
	],
	fileios: [
		// 3x3 with directions, 999, `?`, `#` and a loop with one cross.
		"pzprv3/mintonette/3/3/1,5 0,999 4,3 /# . 0,- /2,0 . . /0 0 /0 -1 /0 0 /0 1 0 /1 0 0 /",
		// 2x2 with `#` and `?`.
		"pzprv3/mintonette/2/2/1,5 # /2,- . /0 /-1 /0 /0 1 /",
		// 1x1 with no inner borders, so the border layer is empty.
		"pzprv3/mintonette/1/1/0,0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qdir = 0;
			const x = r.rnd();
			if (x < 0.4) {
				cell.qnum = -1;
			} else if (x < 0.5) {
				cell.qnum = -2;
				cell.qdir = Math.floor(r.rnd() * 5);
			} else if (x < 0.55) {
				cell.qnum = -3;
			} else {
				cell.qnum = Math.floor(r.rnd() * 1000);
				cell.qdir = Math.floor(r.rnd() * 5);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

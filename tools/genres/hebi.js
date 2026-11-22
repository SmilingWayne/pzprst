// Fixture inputs for pzprjs pid "hebi". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "hebi",
	dataset: null,
	urls: [
		// Hand-built 3x3: nine arrows with clues 0-3.
		"hebi/3/3/101112132021222330",
		"https://puzz.link/p?hebi/3/3/101112132021222330",
		"https://puzz.link/p?hebi_edit/3/3/101112132021222330",
		"http://pzv.jp/p.html?type=player&hebi/3/3/101112132021222330",
		// The old name resolves to hebi.
		"snakes/3/3/101112132021222330",
		// `+` (arrow without a number) and `?` (`.`), both 1x1.
		"hebi/1/1/+",
		"hebi/1/1/0.",
		// Largest allowed clue (5) on a 1x1 board.
		"hebi/1/1/05",
		// Base-36 run lengths, 1/N and N/1 boards.
		"hebi/1/5/g",
		"hebi/5/1/g",
		"hebi/4/4/v",
		"hebi/2/2/10111213",
		// Three-character escape and a two-digit number.
		"hebi/1/1/-14",
		"hebi/1/1/510",
		// Truncated body, trailing junk and empty bodies.
		"hebi/3/3/1011",
		"hebi/3/3/101112132021222330zz",
		"hebi/3/3",
		"hebi/3/3/",
		"hebi/4/4/.",
		// Real 10x10 sample URLs.
		"https://puzz.link/p?hebi/10/10/b25c42g0.c0.b34a0.k12d44b33c45d0.c21b14d12k0.a0.b25c15g0.c34b",
		"https://puzz.link/p?hebi/10/10/k3514b1524a0.i45a0.23n130.d2515n0.13a45i0.a2414b0.45k",
		"https://puzz.link/p?hebi/10/10/b34d43f12f13d34g35c0.a14a0.n25a25a31c0.g15d21f14f0.d0.b"
	],
	fileios: [
		// 3x3 with clues, `?`, `+`, answer numbers and the dot mark.
		"pzprv3/hebi/3/3/1,0 1,1 1,2 /2,- # 2,3 /3,4 0,5 2,2 /1 + . / . 2 . / 3 . . /",
		// 2x2 with every answer mark the shared codec reads.
		"pzprv3/hebi/2/2/0,- 1,2 /# 2,3 /3 c /- = /",
		// 1x1 with an arrow-only cell.
		"pzprv3/hebi/1/1/# /% /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qdir = 0;
			if (x < 0.4) {
				cell.qnum = -1;
			} else if (x < 0.5) {
				cell.qnum = -2;
				cell.qdir = 1 + Math.floor(r.rnd() * 4);
			} else if (x < 0.55) {
				cell.qnum = -3;
			} else {
				cell.qnum = Math.floor(r.rnd() * 6);
				cell.qdir = Math.floor(r.rnd() * 5);
			}
			const a = r.rnd();
			cell.anum = -1;
			cell.qsub = 0;
			if (a < 0.2) {
				cell.anum = 1 + Math.floor(r.rnd() * 5);
			} else if (a < 0.35) {
				cell.qsub = 1;
			}
		});
	}
};

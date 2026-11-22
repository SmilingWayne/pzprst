// Fixture inputs for pzprjs pid "yajikazu". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "yajikazu",
	dataset: null,
	urls: [
		// Hand-built 3x3: every direction, numbers 0-8, `?` and `+`.
		"yajikazu/3/3/101112131415161718",
		"https://puzz.link/p?yajikazu/3/3/101112131415161718",
		"https://puzz.link/p?yajikazu_edit/3/3/101112131415161718",
		"http://pzv.jp/p.html?type=player&yajikazu/3/3/101112131415161718",
		// `?`, `+`, an unknown flag, and the largest URL number (4095).
		"yajikazu/3/3/1.+",
		"yajikazu/x/3/3/10",
		"yajikazu/1/1/-1fff",
		// Two- and three-character numbers.
		"yajikazu/2/2/6107ff-12004.",
		"yajikazu/1/1/510",
		// 1x1, 1/N and N/1 boards.
		"yajikazu/1/1/00",
		"yajikazu/1/5/g",
		"yajikazu/5/1/g",
		// Base-36 run lengths.
		"yajikazu/4/4/v",
		// Truncated body, trailing junk and empty bodies.
		"yajikazu/3/3/1011",
		"yajikazu/3/3/101112131415161718zz",
		"yajikazu/3/3",
		"yajikazu/3/3/",
		// Real puzzle URLs (puzz.link).
		"https://puzz.link/p?yajikazu/10/10/e40g32k32f40123242b11b12a23a42c12e10e44e22a20k3113a43a42i13c",
		"https://puzz.link/p?yajikazu/10/10/a24b42k4231e31a33b11b23l43b3331b13b22a13e20a43d22d32b1311c14b32l3332",
		"https://puzz.link/p?yajikazu/10/10/h23g31d11c1142g11b22b12a11f31b3113314241d3122a1322l30a2121b32a33b21j"
	],
	fileios: [
		// 3x3 with clues, `?`, `+`, the answer layer and a border line layer.
		"pzprv3/yajikazu/3/3/1,0 1,1 1,2 /1,3 1,4 1,5 /1,6 1,7 1,8 /# + . /. # . /. . . /1 -1 /0 0 /0 0 /0 0 0 /0 0 0 /",
		// 2x2 with every number range, `?`, answer marks and a cross border.
		"pzprv3/yajikazu/2/2/1,16 2,255 /1,512 4,- /# . /. + /1 /0 /-1 0 /",
		// 1x1: no inner borders, so the border layer is omitted.
		"pzprv3/yajikazu/1/1/0,0 /./"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qdir = 0;
			if (x < 0.4) {
				cell.qnum = -1;
			} else if (x < 0.5) {
				cell.qnum = -2;
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			} else if (x < 0.55) {
				cell.qnum = -3;
			} else {
				cell.qnum = r.pick([0, 1, 2, 9, 15, 16, 99, 255, 256, 511, 999]);
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			}
			const a = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (a < 0.2) {
				cell.qans = 1;
			} else if (a < 0.35) {
				cell.qsub = 1;
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.line = 1;
			} else if (x < 0.3) {
				border.qsub = 2;
			}
		});
	}
};

// Fixture inputs for pzprjs pid "pencils". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "pencils",
	dataset: null,
	urls: [
		// Hand-built 3x3 with clues 1-9.
		"pencils/3/3/123456789",
		"https://puzz.link/p?pencils/3/3/123456789",
		"https://puzz.link/p?pencils_edit/3/3/123456789",
		"http://pzv.jp/p.html?type=player&pencils/3/3/123456789",
		// Every arrow direction and `?` in one 3x3.
		"pencils/3/3/ghij.",
		// Clue 0 and `?` on 1x1 boards.
		"pencils/1/1/0",
		"pencils/1/1/.",
		// Largest single-digit and two-digit clues.
		"pencils/2/2/fm",
		"pencils/1/1/-ff",
		// Base-36 run lengths, 1/N and N/1 boards.
		"pencils/4/4/z",
		"pencils/2/2/k",
		"pencils/4/1/1234",
		"pencils/1/4/1234",
		// Truncated body, trailing junk and empty bodies.
		"pencils/3/3/12",
		"pencils/3/3/123zz",
		"pencils/3/3",
		"pencils/3/3/",
		// Real sample URLs.
		"https://puzz.link/p?pencils/17/17/gkgkbv4k4zljmgq4n371lgp2ztglhmgw6q4zl1lhlhzp5qilhmgghk3p422k1k3mho4k22p7xhkhkgz2n4n2tao2l",
		"https://puzz.link/p?pencils/10/10/p4mi5zk3ggq3543k22kgk4zl2o1w1ljl1o",
		"https://puzz.link/p?pencils/10/10/m2ygnhn1r1m1kh3o3k1m3l4o4l2mhht4m3ni5n"
	],
	fileios: [
		// 3x3 with arrows, clues, border answers and a partially drawn loop.
		"pzprv3/pencils/3/3/. . . /1 1 1 /o4 o3 . /0 0 /2 0 /0 -1 /0 0 0 /1 0 0 /0 0 /1 0 /0 0 /0 0 0 /0 -1 0 /+2 -3 . /. . . /4 . . /",
		// 2x2 with an arrow, a clue and every answer mark.
		"pzprv3/pencils/2/2/1 2 /o3 . /0 /1 /0 0 /0 /0 /1 0 /+1 - /. . /",
		// 3x3 with a clue too large for the URL (encoded as `?` there).
		"pzprv3/pencils/3/3/o300 . . /. . . /. . . /0 0 /0 0 /0 0 /0 0 0 /0 0 0 /0 0 /0 0 /0 0 /0 0 0 /0 0 0 /. . . /. . . /. . . /",
		// 1x1 with no inner borders.
		"pzprv3/pencils/1/1/. /./"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = -1;
			cell.qdir = 0;
			const x = r.rnd();
			if (x < 0.4) {
				if (r.rnd() < 0.2) {
					cell.qnum = -2;
				}
			} else if (x < 0.65) {
				cell.qdir = 1 + Math.floor(r.rnd() * 4);
			} else {
				cell.qnum = Math.floor(r.rnd() * (Math.max(p.board.cols, p.board.rows) - 1 + 1));
			}
			// The URL drops either half of a clue/arrow pair, so keep them exclusive.
			if (cell.qnum !== -1) {
				cell.qdir = 0;
			}
			const a = r.rnd();
			cell.qans = a < 0.2 ? 1 + Math.floor(r.rnd() * 4) : 0;
			cell.qsub = a > 0.85 ? 1 + Math.floor(r.rnd() * 2) : 0;
		});
		p.board.border.each((border) => {
			border.ques = 0;
			border.line = 0;
			border.qans = 0;
			border.qsub = 0;
			const x = r.rnd();
			if (x < 0.2) {
				border.qans = 1;
				if (r.rnd() < 0.3) {
					border.qsub = 1;
				}
				if (r.rnd() < 0.25) {
					border.line = 1;
				}
			} else if (x < 0.35) {
				border.qsub = r.rnd() < 0.5 ? 1 : 2;
			} else if (x < 0.6) {
				border.line = 1;
			}
			// pzprjs stores a border mark 2 only in the line layer, so a line wins.
			if (border.qsub === 2) {
				border.line = 0;
			}
		});
	}
};

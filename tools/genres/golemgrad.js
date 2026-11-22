// Fixture inputs for pzprjs pid "golemgrad". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "golemgrad",
	dataset: null,
	urls: [
		// Real puzzle from pzprjs test/script/golemgrad.js.
		"golemgrad/6/6/30o0g0g4g0m1g0l.0",
		"https://puzz.link/p?golemgrad/6/6/30o0g0g4g0m1g0l.0",
		"https://puzz.link/p?golemgrad_edit/6/6/30o0g0g4g0m1g0l.0",
		"http://pzv.jp/p.html?type=player&golemgrad/6/6/30o0g0g4g0m1g0l.0",
		"golemgrad/x/6/6/30o0g0g4g0m1g0l.0",
		"golemgrad/6/6/30o0g0g4g0m1g0l.0/",
		"golemgrad/6/6/30o0g0g4g0m1g0l.0zz",
		"golemgrad/6/6/30o0",
		"golemgrad/6/6/",
		"golemgrad/6/6",
		"golemgrad/1/1/0",
		"golemgrad/1/1/.",
		"golemgrad/16/1/g",
		"golemgrad/1/16/g",
		"golemgrad/40/1/-28",
		"golemgrad/2/2/....",
		"golemgrad/3/3/012345678",
		"golemgrad/30/1/zzk"
	],
	fileios: [
		// Real 6x6 board from pzprjs test/script/golemgrad.js, with backgrounds,
		// answer lines and a cross.
		"pzprv3/golemgrad/6/6/3 0 . . . . /. . . . . 0 /. 0 . 4 . 0 /. . . . . . /. 1 . 0 . . /. . . . - 0 /1 0 0 0 2 0 /0 0 0 0 0 0 /0 0 2 0 0 0 /0 0 0 0 1 0 /0 0 0 0 0 0 /0 0 0 0 0 2 /1 0 0 0 0 /0 1 0 0 0 /0 0 1 -1 0 /0 0 0 0 1 /0 1 0 0 0 /0 0 0 1 0 /1 0 0 0 0 0 /0 1 0 1 0 0 /0 0 0 0 0 1 /0 0 1 0 0 0 /1 0 0 0 1 0 /",
		// Clues 0 and ?, backgrounds 1/2, full border layer.
		"pzprv3/golemgrad/2/2/2 0 /. - /0 1 /1 2 /0 0 /0 1 /1 0 /",
		// 1x1: no inner borders, so the border layer is one empty line.
		"pzprv3/golemgrad/1/1/0 /1 //"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		const clues = [-1, -1, -1, -1, -2];
		for (let n = 0; n <= Math.min(maxnum, 6); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
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

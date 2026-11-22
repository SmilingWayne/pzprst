// Fixture inputs for pzprjs pid "patchwork". See tools/gen_fixtures.js for the format.
// The real puzzles come from pzprjs test/script/patchwork.js and the native
// puzz.link samples; the dataset folder holds solved states but no puzzlink_url.
"use strict";

module.exports = {
	pid: "patchwork",
	dataset: "Patchwork",
	urls: [
		// Real 6x6 puzzle from pzprjs test/script/patchwork.js; canonicalises to itself.
		"patchwork/6/6/8j3i0m.i1h.i1.h0.g1",
		"https://puzz.link/p?patchwork/6/6/8j3i0m.i1h.i1.h0.g1",
		"http://pzv.jp/p.html?type=player&patchwork/6/6/8j3i0m.i1h.i1.h0.g1",
		"patchwork_edit/6/6/8j3i0m.i1h.i1.h0.g1",
		// Real puzz.link puzzles.
		"https://puzz.link/p?patchwork/9/9/3g2o.j4o0j.i0g1i1g0i.j0o2j.o1g3",
		"https://puzz.link/p?patchwork/8/8/s22g2j23x23j0g33j.m./",
		"https://puzz.link/p?patchwork/10/10/.h.k12h1l1j2o.g3g.1l.g..g.l2.g5g.o5j2l2h13k.h./",
		// 1x1, 1xN and Nx1 boards.
		"patchwork/1/1/0",
		"patchwork/1/3/0",
		"patchwork/3/1/0",
		// Truncated body, trailing junk, empty body.
		"patchwork/2/2/0",
		"patchwork/6/6/8j3i0m.i1h.i1.h0.g1zz",
		"patchwork/3/3/",
		"patchwork/6/6",
		// Every single-digit value, plus `?` cells (`.` in the number layer).
		"patchwork/4/4/0123456789abcdef",
		"patchwork/2/2/1.h"
	],
	fileios: [
		// The real 6x6 puzzle in its solved state: numbers, shading and border marks.
		"pzprv3/patchwork/6/6/8 # # . . 3 /# # # 0 . . /# # # . . - /. . # 1 . . /- . . . 1 - /. . 0 - . 1 /0 0 1 0 0 /0 0 1 0 0 /0 0 1 0 0 /0 1 0 1 0 /0 1 0 1 0 /0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /1 1 1 1 0 0 /0 0 0 0 0 0 /0 0 1 1 0 0 /",
		// 2x2: clue, `?`, shading, mark and every border answer token.
		"pzprv3/patchwork/2/2/1 - /# + /2 /-1 /0 1 /",
		// 1x1 has no inner borders; only the cell line and one empty border line.
		"pzprv3/patchwork/1/1/0 //"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		p.board.cell.each((cell) => {
			cell.qnum = -1;
			cell.qans = 0;
			cell.qsub = 0;
			const x = r.rnd();
			if (x < 0.4) {
				cell.qnum = Math.floor(r.rnd() * maxnum);
			} else if (x < 0.55) {
				cell.qnum = -2;
			} else if (x < 0.75) {
				cell.qans = 1;
			} else if (x < 0.9) {
				cell.qsub = 1;
			}
		});
		p.board.border.each((border) => {
			border.qans = r.rnd() < 0.2 ? 1 : 0;
			border.qsub = r.rnd() < 0.2 ? 1 : 0;
		});
	}
};

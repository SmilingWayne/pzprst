// Fixture inputs for pzprjs pid "cts". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "cts",
	// No Cross the Streams folder in ../puzzlekit-dataset/assets/data.
	dataset: null,
	urls: [
		// Every excell hex digit, then a truncated body and trailing junk.
		"cts/4/4/123456789abcdef",
		"cts/4/4/0001",
		"cts/4/4/0001zz",
		"cts/4/4/12",
		// `?` (`.`), the widest `-` escape and flushed column gaps.
		"cts/4/4/.",
		"cts/4/4/-ff",
		"cts/3/4/2g3g1g2g1g3i",
		// Small and thin boards.
		"cts/1/1/",
		"cts/1/4/l",
		"cts/4/1/l",
		"cts/3/4/1a3b5c7d9e",
		"cts/6/5/1g2g3g4g5g6g7g8g",
		// Real puzz.link boards.
		"https://puzz.link/p?cts/9/10/5j.j.j...h303h10i3..h11.h10i0j0j0j..i33i33i0j303h...h.3i",
		"https://puzz.link/p?cts/8/8/050g032g03.g1i1i030g04.g030g030g030g030g040g030g030g030g030g",
		"https://puzz.link/p?cts/15/15/016.j020201h02l.20k0m.70k1...20h....j10l0702j0m0220j0310j0m06060i..522i0m2140j0110j.....i2.l....j.1.k....j2..k0m.204j21030i.10k.2222.h"
	],
	fileios: [
		// Gaps in the clue lists (the URL has to flush them) and every answer
		// mark a cell can hold. FileIO cannot store a `?` clue (`+ca` of any
		// non-numeric token is NaN), so `?` appears in the URLs above only.
		"pzprv3/cts/4/3/. . 2 3 2 /. . . . 0 /3 1 # . . /. 2 . + . /. 3 . . . /. . . . . /",
		// 2x2 with every clue entry filled (nothing to flush) and shaded cells.
		"pzprv3/cts/2/2/. 1 2 /3 . . /4 . # /",
		// 1x1 with a `c` (`qcmp`) mark: pzprjs moves `qcmp` by object id while
		// `adjustExCellTopLeft_2` rewrites only `qnum`, so on boards larger than
		// 1x1 it is not equivariant under the transform group. The top-left
		// corner token belongs to no object.
		"pzprv3/cts/1/1/. c1 /0 # /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.35) {
				cell.qsub = 1;
			}
		});
		p.board.excell.each((excell) => {
			// `ExCell.maxnum`: a column's clue list allows rows, a row's allows
			// cols. `qcmp` is not set here: it is answer data that pzprjs moves
			// by object id while `adjustExCellTopLeft_2` rewrites only `qnum`,
			// so it is not equivariant under the transform group (see the
			// 1x1 FileIO below).
			const max = excell.by < 0 ? p.board.rows : p.board.cols;
			excell.qnum = r.pick([-1, -1, -1, 0, 1, Math.floor(max / 2), max]);
		});
	}
};

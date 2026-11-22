// Fixture inputs for pzprjs pid "cave". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "cave",
	dataset: "Cave",
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"cave/5/3/6l2l3",
		"bag/5/5/j39k3g4k25j",
		"cave/6/6/g3q2h3jbj3i3i2g",
		"https://puzz.link/p?cave/5/3/6l2l3",
		"http://pzv.jp/p.html?type=player&cave/5/3/6l2l3",
		"cave/5/3/6l2l3/",
		"cave/5/3/6l2l3//",
		"cave/5/3/6l2l3zz",
		"cave/4/4/",
		"cave/4/4/v",
		"cave/4/4/.",
		"cave/4/4/7",
		"cave/1/1/.",
		"cave/1/1/0",
		"cave/1/5/1",
		"cave/5/1/-19",
		"cave/5/5/1",
		"cave/5/5/-1",
		"cave/3/3/HZ",
		"cave/3/3/o",
		"cave/5/5/0123456789",
		"cave/2/2/=001",
		"cave/2/2/@001",
		"cave/2/2/*0001",
		"cave/2/2/$00001",
		"cave/30/1/zzk",
		"cave/25/1/0123456789012345678901234",
		"cave/1/25/0123456789012345678901234",
		"cave/x/4/4/v",
		"cave/4/4"
	],
	fileios: [
		// Real 6x6 answer board from pzprjs test/script/cave.js.
		"pzprv3/cave/6/6/. 3 . . . . /. . . . . . /. 2 . . 3 . /. . . 11 . . /. . 3 . . . /3 . . . 2 . /# . . . # . /# . # . # # /# . # . . # /. . . . . . /. # . . # # /. # # . . # /",
		// Question numbers, shaded/background cells and border crosses.
		"pzprv3/cave/2/5/- 2 3 4 5 /6 . 6 5 4 /+ . . . . /+ # . . . /0 0 0 -1 /0 0 0 -1 /0 0 0 -1 0 /",
		// Border lines and crosses.
		"pzprv3/cave/2/5/2 . 3 . 4 /. . 2 . 3 /. . . . . /. . . . . /1 0 -1 0 /0 1 0 -1 /0 1 0 0 0 /",
		// Question marks, two-digit clues and every layer.
		"pzprv3/cave/3/4/. - 2 6 /3 . 5 . /. . . . /+ # . + /# . # . /. . . . /0 0 -1 /0 0 0 /1 0 0 /0 1 0 0 /0 0 0 1 /",
		// An all-zero border layer is dropped by encodeBorderLineIfPresent.
		"pzprv3/cave/1/2/1 2 /. . /0 /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols + p.board.rows - 1;
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 2; n <= Math.min(maxnum, 6); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			const x = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.35) {
				cell.qsub = 1;
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

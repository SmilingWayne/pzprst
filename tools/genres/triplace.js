// Fixture inputs for pzprjs pid "triplace". See tools/gen_fixtures.js for the format.
// No Tri-place folder in ../puzzlekit-dataset/assets/data; the three puzz.link
// URLs are from the crawler log.
"use strict";

module.exports = {
	pid: "triplace",
	dataset: null,
	urls: [
		// A `[＼]` cell with no numbers (`_`) and the top/left clues.
		"triplace/3/3/_n123456",
		// Two `[＼]` cells; the truncated body below exercises pzprjs's quirk of
		// restarting the clue loops at the body start when the cell layer ends
		// without overrunning the last cell (here the body then holds exactly
		// the two top clues).
		"triplace/3/3/12h34k123456",
		"triplace/2/2/12",
		"triplace/3/3/12h34k123456zz",
		// Every escape: `-` (down clue >= 16), `+` (across clue >= 16) and `=`
		// (both), then the base-36 `$` (across only) and `@` (down only).
		"triplace/3/3/-112+121=1212l123456",
		"triplace/3/3/$5h@ak123456",
		// An outside clue of 20 (`-14`), outside the input range of a 3-wide or
		// 3-tall board but still what the URL codec stores.
		"triplace/3/3/o-14...",
		// Empty boards: canonical and small/thin sizes.
		"triplace/3/3/o......",
		"triplace/1/1/..",
		"triplace/1/4/o....",
		"triplace/4/1/o....",
		// Canonical forms of the FileIO boards below.
		"triplace/4/4/12_g34g45j_h26h3.527.92",
		"triplace/1/1/1432",
		"triplace/2/2/21i1234",
		// Real puzz.link boards.
		"https://puzz.link/p?triplace/6/5/zp11333144301",
		"https://puzz.link/p?triplace/4/4/k_p..2.1.0./",
		"https://puzz.link/p?triplace/11/11/g$1i_i_j$1i_j$1i_i_j_i$1j_i_i_j_i_j$1i_i_j_i$1j_i$1i$1j_i$1j_i_i_g............1........./"
	],
	fileios: [
		// Top `qnum2` / left `qnum` clues, `[＼]` cells with both numbers, every
		// border answer plus the cell sub marks. (Tri-place has no config flags,
		// so no flag line appears.)
		"pzprv3/triplace/4/4/0 3 -1 5 2 /7 2,1 -1,-1 . 4,3 /-1 . 5,4 . . /9 . . -1,-1 . /2 . 6,2 . . /0 1 2 /-1 0 1 /0 0 0 /2 -1 1 /0 1 2 0 /-1 0 1 2 /0 0 0 0 /+ . - . /. + . . /. . . - /. . . . /",
		// 1x1: one `[＼]` cell, no border layer (a 1x1 board has no inner border).
		"pzprv3/triplace/1/1/0 3 /2 4,1 /+ /",
		// 2x2 with a single `[＼]` cell and mixed borders.
		"pzprv3/triplace/2/2/0 1 2 /3 1,2 . /4 . . /0 1 /2 0 /0 1 2 /1 0 1 /+ . /. - /"
	],
	randomize(p, r) {
		const bd = p.board;
		bd.cell.each((cell) => {
			if (r.rnd() < 0.4) {
				const col = (cell.bx - 1) / 2;
				const row = (cell.by - 1) / 2;
				// `Cell.getmaxnum`: the across clue counts the cells to the
				// right, the down clue the cells below.
				const across = bd.cols - col - 1;
				const down = bd.rows - row - 1;
				cell.set51cell();
				cell.qnum = r.pick([-1, -1, 0, Math.min(1, across), across]);
				cell.qnum2 = r.pick([-1, -1, 0, Math.min(1, down), down]);
			}
			cell.qsub = r.pick([0, 0, 0, 0, 1, 2]);
		});
		bd.border.each((border) => {
			border.qans = r.rnd() < 0.2 ? 1 : 0;
			border.qsub = r.rnd() < 0.15 ? 1 : 0;
		});
		bd.excell.each((excell) => {
			// Top excells store their clue in `qnum2`, left ones in `qnum`; the
			// top-left corner stores nothing.
			if (excell.bx > 0) {
				excell.qnum2 = r.pick([-1, -1, 0, 1, Math.min(2, bd.rows), bd.rows]);
			} else if (excell.by > 0) {
				excell.qnum = r.pick([-1, -1, 0, 1, Math.min(2, bd.cols), bd.cols]);
			}
		});
	}
};

// Fixture inputs for pzprjs pid "firefly". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "firefly",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"firefly/10/10/a33j40e21a43f32e12b21a31b3.b3.h3.b33e3.f12d3.a41k42h3.b",
		"firefly/10/10/e35b31b45d2.b2.15l2.d4.c1.b3.c2111c32b42c41d34l2.12b3.d28b1.b31e",
		"firefly/10/10/41d24l24a43b1.c1.d1.b14c1.d21c42b17c42d1.c32b21d41c3.b33a2.l34d1./",
		"https://puzz.link/p?firefly/10/10/a33j40e21a43f32e12b21a31b3.b3.h3.b33e3.f12d3.a41k42h3.b",
		// No pflags exist; an unknown flag is ignored like pzprjs.
		"firefly/f/2/2/0.0.0.0.",
		// Truncated body (the last complete cell is followed by empty cells).
		"firefly/3/3/0.",
		// `+` is a cell with an arrow but no number.
		"firefly/3/3/+",
		// Question-mark cell.
		"firefly/2/2/3.",
		// The three clue number widths: 999 (`-` form), 100 (`5`-`9` form).
		"firefly/3/3/-33e7",
		"firefly/3/3/564",
		// A longer-than-canonical input (`-10f` = direction 1, clue 15, then a run).
		"firefly/2/2/-10f",
		// Blank run flushed at 26 cells, trailing junk.
		"firefly/6/6/z",
		"firefly/3/3/0.zz",
		// 1x1, 1xN and Nx1 boards.
		"firefly/1/1/+",
		"firefly/1/4/0.0.0.0.",
		"firefly/4/1/0.3.4.2.",
		// Empty 2x2 board.
		"firefly/2/2/0.0.0.0."
	],
	fileios: [
		// Every cell kind: clue with direction, `?`, `+` and empty; border lines.
		"pzprv3/firefly/4/4/0,3 3,- 4,1 . /# . 2,2 . /0,- . . 4,3 /. . . . /0 1 0 /-1 0 0 /0 0 1 /0 0 0 /1 0 0 0 /0 0 -1 0 /0 0 0 0 /",
		// 1x4 board with the largest clue.
		"pzprv3/firefly/1/4/3,2 0,- 0,999 # /0 -1 1 /",
		// 1x1 board.
		"pzprv3/firefly/1/1/# /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, -2, -3, 0, 1, 2, 999]);
			cell.qdir = cell.qnum === -3 ? 0 : r.pick([0, 0, 0, 1, 2, 3, 4]);
		});
		p.board.border.each((border) => {
			border.line = r.pick([0, 0, 0, 0, 1]);
			border.qsub = r.pick([0, 0, 0, 0, 2]);
		});
	}
};

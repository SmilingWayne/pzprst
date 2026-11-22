// Fixture inputs for pzprjs pid "coral". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "coral",
	// No Coral folder in ../puzzlekit-dataset/assets/data (only Nonogram).
	dataset: null,
	urls: [
		// Every excell hex digit, then a truncated body and trailing junk.
		"coral/4/4/123456789abcdef",
		"coral/4/4/0001",
		"coral/4/4/0001zz",
		// `?` (`.`), the widest `-` escape and flushed column gaps.
		"coral/4/4/.",
		"coral/4/4/-ff",
		"coral/3/4/2g3g1g2g1g3i",
		// Small and thin boards.
		"coral/1/1/",
		"coral/1/4/l",
		"coral/4/1/l",
		"coral/3/4/1a3b5c7d9e",
		"coral/6/5/1g2g3g4g5g6g7g8g",
		// Real puzz.link boards.
		"https://puzz.link/p?coral/8/8/61l33p43l31h41l33p211k321g",
		"https://puzz.link/p?coral/11/11/43j22p222i222o331i331o32j22j111o3211h33p4q22j2211n411i",
		"https://puzz.link/p?coral/6/6/32s41g32j21m22g"
	],
	fileios: [
		// Question and answer marks with gaps in the clue lists, so the URL has
		// to flush every column/row (`2g3g1g2g1g3i`).
		"pzprv3\ncoral\n4\n3\n. . 2 3 1 \n. . . . . \n. 2 # . . \n. 1 . . + \n. 3 . . . \n. . . . . \n",
		// `qcmp` (`c` in FileIO) is answer data; pzprjs `turnflipGroup` moves it
		// by object id while `adjustExCellTopLeft_2` rewrites only `qnum`, so on
		// boards larger than 1x1 it is not equivariant under the transform
		// group. Keep the property in fixtures on 1x1, where every reversal is
		// identity.
		"pzprv3\ncoral\n1\n1\n. c1 \n0 # \n",
		// A 2x2 board with a full clue list (nothing to flush) and a shaded
		// cell; the empty top-left corner token belongs to no object.
		"pzprv3\ncoral\n2\n2\n. 1 2 \n3 . . \n4 . # \n"
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
			// cols. `qcmp` marks are not set (see the 1x1 FileIO above).
			const max = excell.by < 0 ? p.board.rows : p.board.cols;
			excell.qnum = r.pick([-1, -1, -1, 0, 1, Math.floor(max / 2), max]);
		});
	}
};

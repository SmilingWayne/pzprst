// Fixture inputs for pzprjs pid "batten". See tools/gen_fixtures.js for the format.
// No Battenberg Painting folder in ../puzzlekit-dataset/assets/data and no
// native-derived batten links in the crawler log, so the URLs below are the
// hand-built edge cases (cross marks + excell clues) only.
"use strict";

module.exports = {
	pid: "batten",
	dataset: null,
	urls: [
		// Empty bodies on a square and on minimal boards.
		"batten/4/4/",
		"batten/4/4/9n",
		"batten/1/1/",
		"batten/1/4/",
		"batten/4/1/",
		// Cross marks: a mark at the first/last cross and base-36 skip runs.
		"batten/4/4/0",
		"batten/4/4/1",
		"batten/4/4/9",
		"batten/4/4/z",
		"batten/4/4/zz",
		// Every cross-layer digit, then the excell layer.
		"batten/3/3/0123456789abcdef",
		// A `?` clue (`.`), a truncated body and trailing junk.
		"batten/4/4/9.",
		"batten/4/4/-",
		"batten/4/4/9nzz",
		// Cross marks, excell clues and answer marks (canonical of the first
		// FileIO below).
		"batten/4/4/0723g01g24"
	],
	fileios: [
		// Cross symbols (1), explicit blanks (0) and a `?` cross (-), clues
		// with gaps, and every answer mark a cell can hold.
		"pzprv3/batten/4/4/- . . 0 . /. 1 . . . /. . . . . /0 . . 1 . /. . . . . /. 2 3 . 0 /1 . . . . /. . # . . /2 . . + . /4 . . . . /",
		// One-row board: the cross grid is 2 rows of 5.
		"pzprv3/batten/1/4/0 1 2 3 4 /- . 0 1 2 /. 1 2 3 4 /0 . # . + /",
		// 1x1 with a `c` (`qcmp`) mark: pzprjs moves `qcmp` by object id while
		// `adjustExCellTopLeft_2` rewrites only `qnum`, so on boards larger than
		// 1x1 it is not equivariant under the transform group.
		"pzprv3/batten/1/1/- 1 /0 0 /. c1 /0 # /"
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
		// `Cross.maxnum` is 1; -1 is empty and 0 is an explicit blank (the
		// FileIO value a native board can carry).
		p.board.cross.each((cross) => {
			const x = r.rnd();
			cross.qnum = x < 0.2 ? 1 : x < 0.3 ? 0 : -1;
		});
		p.board.excell.each((excell) => {
			// `ExCell`: minnum 0; the top row allows `rows`, the left column
			// `cols`. `qcmp` is answer data, so it stays unset (see the 1x1
			// FileIO above).
			const max = excell.by === -1 ? p.board.rows : p.board.cols;
			excell.qnum = r.pick([-1, -1, -1, 0, 1, Math.floor(max / 2), max]);
		});
	}
};

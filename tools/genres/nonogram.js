// Fixture inputs for pzprjs pid "nonogram". See tools/gen_fixtures.js for the format.
// The puzzlekit-dataset "Nonogram" folder stores problem texts, not
// `puzzlink_url` records, so dataset scrape adds nothing (parity: no dataset).
"use strict";

module.exports = {
	pid: "nonogram",
	dataset: "Nonogram",
	urls: [
		// Every excell hex digit, then a truncated body and trailing junk.
		"nonogram/4/4/123456789abcdef",
		"nonogram/4/4/0001",
		"nonogram/4/4/0001zz",
		"nonogram/4/4/12",
		// `?` (`.`), the widest `-` escape and flushed column gaps.
		"nonogram/4/4/.",
		"nonogram/4/4/-ff",
		"nonogram/3/4/2g3g1g2g1g3i",
		// Small and thin boards.
		"nonogram/1/1/",
		"nonogram/1/4/l",
		"nonogram/4/1/l",
		"nonogram/3/4/1a3b5c7d9e",
		"nonogram/6/5/1g2g3g4g5g6g7g8g",
		// A 10x10 with every clue entry filled.
		"nonogram/10/10/1g1g1g1g1g1g1g1g1g1g1g1g1g1g1g1g1g1g1g1g",
		// Real puzz.link boards.
		"https://puzz.link/p?nonogram/15/15/22l33l1221j231k251k11l9m311k272k2212j21511i12321i2112j21l22l3m12l222k1211j152k211113h21122i2112j21122i211113h252k1321j222k132k3m",
		"https://puzz.link/p?nonogram/15/15/7mdm1bl1bl135k1314j134k19l134k1314j135k181k171kdm7m11l113k114k1aldmdmfm414k41114i414kfm292k2332j11lbm",
		"https://puzz.link/p?nonogram/10/10/ajajajajajajajajajajajajajajajajajajajaj"
	],
	fileios: [
		// Gaps in the clue lists (the URL has to flush them) and every answer
		// mark a cell can hold. FileIO cannot store a `?` clue (`+ca` of any
		// non-numeric token is NaN), so `?` appears in the URLs above only.
		"pzprv3/nonogram/4/3/. . 2 3 2 /. . . . 0 /3 1 # . . /. 2 . + . /. 3 . . . /. . . . . /",
		// 2x2 with every clue entry filled (nothing to flush) and shaded cells.
		"pzprv3/nonogram/2/2/. 1 2 /3 . . /4 . # /",
		// 1x1 with a `c` (`qcmp`) mark: pzprjs moves `qcmp` by object id while
		// `adjustExCellTopLeft_2` rewrites only `qnum`, so on boards larger than
		// 1x1 it is not equivariant under the transform group. The top-left
		// corner token belongs to no object.
		"pzprv3/nonogram/1/1/. c1 /0 # /"
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
			// so it is not equivariant under the transform group.
			const max = excell.by < 0 ? p.board.rows : p.board.cols;
			excell.qnum = r.pick([-1, -1, -1, 0, 1, Math.floor(max / 2), max]);
		});
	}
};

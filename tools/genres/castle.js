// Fixture inputs for pzprjs pid "castle". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "castle",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"castle/12/12/224j234e122125g124f131r222b231e121h131b144h112e241b212r145f114g132142e244j214",
		"castle/11/11/023a042l023a04.a024c042l021a032a041p043g012n00.a013a042d044m00.a014a00.b043a012h",
		"castle/7/7/022j122c242h022h13.c212j012",
		"https://puzz.link/p?castle/7/7/022j122c242h022h13.c212j012",
		// No pflags exist; an unknown flag is ignored like pzprjs.
		"castle/f/2/2/d",
		// All three cell-number encodings, including the largest qnum (4095).
		"castle/3/3/22415340-0fffl",
		// Blank runs: one flush at 26 and a trailing run.
		"castle/6/6/z",
		// Truncated body, trailing junk, empty body.
		"castle/3/3/000",
		"castle/7/7/022j122c242h022h13.c212j012zz",
		"castle/3/3/",
		// 1x1, 1xN and Nx1 boards.
		"castle/1/1/000",
		"castle/1/4/000000000000",
		"castle/4/1/000000000000",
		// Plain empty 2x2 board.
		"castle/2/2/d"
	],
	fileios: [
		// Question data with every clue kind, `?`, shade 3 and the qcmp bit,
		// plus border lines and a cross.
		"pzprv3/castle/7/7/2,2,0 0,-,0 0,-1,0 1,-,5 0,-1,0 0,-1,0 0,-1,0 /0,-1,0 0,-1,0 0,-1,0 0,-1,0 2,2,1 0,-1,0 0,-1,0 /0,-1,0 4,2,2 0,-1,0 0,-1,0 0,-1,0 0,-1,0 0,-1,0 /0,-1,0 0,-1,0 0,-1,0 2,2,0 0,-1,0 0,-1,0 0,-1,0 /0,-1,0 0,-1,0 0,-1,0 0,-1,0 0,-1,0 3,-,1 0,-1,0 /0,-1,0 0,-1,0 1,2,2 0,-1,0 0,-1,0 0,-1,0 0,-1,0 /0,-1,0 0,-1,0 0,-1,0 0,-1,0 0,-1,0 0,-1,0 1,2,0 /1 0 0 0 0 0 /-1 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 1 0 0 0 0 0 /0 0 0 0 0 0 0 /0 0 0 0 0 0 0 /0 0 0 0 0 0 0 /0 0 0 0 -1 0 0 /0 0 0 0 0 0 0 /",
		// qcmp set on shaded cells and a `?` answer cell.
		"pzprv3/castle/3/3/2,4,2 0,52,6 0,4095,7 /0,-1,0 0,-,0 0,-1,0 /0,-1,0 0,-1,0 0,-1,0 /1 0 /-1 0 /0 0 /0 0 0 /0 1 0 /",
		// 1x1 board.
		"pzprv3/castle/1/1/0,0,0 /"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows) - 2;
		const clues = [-1, -1, -1, -1, -2];
		for (let n = 0; n <= maxnum; n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 1, 2]);
			cell.qdir = r.pick([0, 0, 0, 0, 1, 2, 3, 4]);
			cell.qnum = r.pick(clues);
			cell.qcmp = r.pick([0, 0, 0, 0, 1]);
		});
		p.board.border.each((border) => {
			border.line = r.pick([0, 0, 0, 0, 1]);
			border.qsub = r.pick([0, 0, 0, 0, 2]);
		});
	}
};

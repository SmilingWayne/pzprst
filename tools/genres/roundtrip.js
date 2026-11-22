// Fixture inputs for pzprjs pid "roundtrip". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "roundtrip",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"roundtrip/5/5/34l43i4h3i",
		"roundtrip/6/6/j23234j23456l",
		"https://puzz.link/p?roundtrip/5/5/34l43i4h3i",
		"http://pzv.jp/p.html?type=player&roundtrip/3/3/0123",
		// No pflags exist; an unknown flag is ignored like pzprjs.
		"roundtrip/x/2/2/n",
		// Plain clue values, `?` (`.`), the two-digit hex escape and its maximum.
		"roundtrip/3/3/0123",
		"roundtrip/3/3/.",
		"roundtrip/3/3/-0f",
		"roundtrip/3/3/-ff",
		// Truncated body, trailing junk, empty body.
		"roundtrip/3/3/012",
		"roundtrip/3/3/0123nzz",
		"roundtrip/3/3/",
		// 1x1, 1xN and Nx1 boards.
		"roundtrip/1/1/",
		"roundtrip/1/1/0",
		"roundtrip/1/4/",
		"roundtrip/4/1/",
		// Empty 2x2 board.
		"roundtrip/2/2/n"
	],
	fileios: [
		// Question (excell clues) and answer (border lines) data, with `?` (-2).
		"pzprv3/roundtrip/3/3/. 0 1 2 . /. . . . . /. . . . . /. . . . . /. 3 . -2 . /1 0 0/0 0 0/0 1 0/0 0 0/0 0 0/1 0 0/0 0 1",
		// A two-digit hex value (15) on the top row and a bottom-row clue.
		"pzprv3/roundtrip/2/2/. 15 . . /. . . . /. . . . /. 3 . . /1/-1/0 1",
		// 1x1 has no border objects at all.
		"pzprv3/roundtrip/1/1/. 0 . /. . . /. . . ",
		// 1x4: the border section starts with one blank line per row.
		"pzprv3\nroundtrip\n4\n1\n. . . \n. . . \n. . . \n. . . \n. . . \n. . . \n\n\n\n\n0 \n0 \n0 "
	],
	randomize(p, r) {
		p.board.excell.each((excell) => {
			// `ExCell.maxnum`: the top and bottom rows allow rows, the left and
			// right columns allow cols.
			const max = excell.by < 0 || excell.by > p.board.rows * 2 ? p.board.rows : p.board.cols;
			excell.qnum = r.pick([-1, -1, -1, -1, 0, 1, Math.floor(max / 2), max]);
		});
		p.board.border.each((border) => {
			border.line = r.pick([0, 0, 0, 0, 1]);
		});
	}
};

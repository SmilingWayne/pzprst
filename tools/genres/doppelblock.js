// Fixture inputs for pzprjs pid "doppelblock". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "doppelblock",
	dataset: "DoppelBlock",
	urls: [
		// Full-size boards built with pzprjs from random clues.
		"doppelblock/5/5/1h1416g32k3g3k2h1m2",
		"doppelblock/6/6/g94i6g88gat0i0j2g0m101",
		"doppelblock/3/3/012345",
		"https://puzz.link/p?doppelblock/5/5/1h1416g32k3g3k2h1m2",
		// No pflags exist; an unknown flag is ignored like pzprjs.
		"doppelblock/x/3/3/l",
		// Outer clues, `?` (`.`), the two-digit hex escape and its maximum.
		"doppelblock/3/3/0",
		"doppelblock/3/3/.",
		"doppelblock/3/3/-ff",
		// Cell clues are a second layer, present only when a cell has one.
		"doppelblock/5/5/0123456789abcdef",
		"doppelblock/1/1/01.",
		// Truncated body, trailing junk, empty bodies.
		"doppelblock/3/3/01234",
		"doppelblock/3/3/012345zz",
		"doppelblock/3/3/",
		"doppelblock/5/5/",
		// 1x1, 1xN and Nx1 boards.
		"doppelblock/1/1/",
		"doppelblock/1/1/01",
		"doppelblock/1/4/",
		"doppelblock/4/1/"
	],
	fileios: [
		// 3x3: outer clues, a cell `q` clue, an answer number, `+` (`qsub`)
		// and `-` (`qans`, black square).
		"pzprv3/doppelblock/3/3/. 0 1 2 /3 q2 . . /4 . 1 . /5 . + - ",
		// 5x5: every cell token kind and both bands.
		"pzprv3\ndoppelblock\n5\n5\n. 1 . . 1 4 \n1 . . 2 . . \n6 q3 . q3 + . \n. . - . q2 . \n3 . q1 . . . \n2 . . . . q2 ",
		// 1x1 boards: one `?` cell and one black square.
		"pzprv3/doppelblock/1/1/. 0 /1 q-2 ",
		"pzprv3/doppelblock/1/1/. 2 /0 - "
	],
	randomize(p, r) {
		const n = p.board.cols - 2;
		const emax = (n * (n + 1)) / 2;
		p.board.excell.each((excell) => {
			excell.qnum = r.pick([-1, -1, -1, -1, -2, 0, 1, Math.floor(emax / 2), emax]);
		});
		p.board.cell.each((cell) => {
			const qnum = r.pick([-1, -1, -1, -1, -2, 0, 1, Math.max(n - 1, 0)]);
			cell.qnum = qnum;
			// `q<n>` replaces every other cell token, so the answer marks are
			// only legal on cells without a question clue.
			if (qnum === -1) {
				const kind = r.pick([-1, -1, -1, -1, 1, 2, 3]);
				if (kind === 2) {
					cell.qsub = 1;
				} else if (kind === 3) {
					cell.qans = 1;
				} else if (kind === 1 && n >= 1) {
					cell.anum = r.pick([1, Math.max(n - 1, 1)]);
				}
			}
		});
	}
};

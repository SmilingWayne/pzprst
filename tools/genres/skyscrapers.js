// Fixture inputs for pzprjs pid "skyscrapers". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "skyscrapers",
	dataset: "Skyscraper",
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"skyscrapers/6/6/g2g2h35g2k425g4g322",
		"skyscrapers/5/5/i4k3i3335i",
		"skyscrapers/5/5/h4g3g1g2i3i4i",
		"https://puzz.link/p?skyscrapers/5/5/i4k3i3335i",
		// No pflags exist; an unknown flag is ignored like pzprjs.
		"skyscrapers/x/2/2/n",
		// Outer clues, `?` (`.`), the two-digit hex escape and its maximum.
		"skyscrapers/2/2/0123",
		"skyscrapers/2/2/.123",
		"skyscrapers/2/2/-ff",
		// Cell clues are a second layer, present only when a cell has one.
		"skyscrapers/2/2/01230123",
		"skyscrapers/2/2/0123012",
		"skyscrapers/2/2/0123zz",
		// Truncated body, empty body, 20-and-more empty excells (run flush).
		"skyscrapers/2/2/01",
		"skyscrapers/2/2/",
		"skyscrapers/6/6/",
		// 1x1, 1xN and Nx1 boards.
		"skyscrapers/1/1/",
		"skyscrapers/1/1/1i1",
		"skyscrapers/1/4/",
		"skyscrapers/4/1/"
	],
	fileios: [
		// Outer clues on all four sides, a `?` clue, a cell question clue and
		// answer numbers.
		"pzprv3/skyscrapers/3/3/. 3 . . . /. q2 . . . /. . 1 . . /. . . . 4 /. . -2 . .",
		// Answer numbers only (no cell question clues).
		"pzprv3/skyscrapers/3/3/. 3 . . . /. . 2 . . /. . . 1 . /. . . . 4 /. . -2 . .",
		// 1x1 board: one outer clue and one cell question clue.
		"pzprv3/skyscrapers/1/1/. 1 . /. q1 . /. . . "
	],
	randomize(p, r) {
		const max = Math.max(p.board.cols, p.board.rows);
		p.board.cell.each((cell) => {
			const qnum = r.pick([-1, -1, -1, -1, -2, 0, 1, Math.floor(max / 2), max]);
			cell.qnum = qnum;
			// pzprjs writes `q<n>` instead of an answer when `qnum` is set, so
			// the two values cannot coexist on one cell.
			cell.anum = qnum === -1 && r.rnd() < 0.4 ? r.pick([1, Math.max(Math.floor(max / 2), 1), max]) : -1;
		});
		p.board.excell.each((excell) => {
			// `ExCell.maxnum`: the top and bottom rows allow rows, the left and
			// right columns allow cols.
			const emax = excell.by < 0 || excell.by > p.board.rows * 2 ? p.board.rows : p.board.cols;
			excell.qnum = r.pick([-1, -1, -1, -1, -2, 0, 1, Math.floor(emax / 2), emax]);
		});
	}
};

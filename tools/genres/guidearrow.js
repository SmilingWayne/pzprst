// Fixture inputs for pzprjs pid "guidearrow". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "guidearrow",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"guidearrow/6/6/34kcdjeodbi",
		"guidearrow/9/7/94sbmdg676ckckcq",
		"guidearrow/12/12/67ldlchcgcn1cnbgchbznchcgcnbsbhblej",
		"https://puzz.link/p?guidearrow/6/6/34kcdjeodbi",
		// No pflags exist; an unknown flag is ignored like pzprjs.
		"guidearrow/f/2/2/11",
		// Empty body and a default goal with no clue layer.
		"guidearrow/3/3/",
		"guidearrow/3/3/11",
		// `?` cell and a truncated clue layer.
		"guidearrow/3/3/11.",
		"guidearrow/3/3/13g",
		// Clue 0 encoded by `decode4Cell`'s skip forms.
		"guidearrow/2/2/115.g",
		// The largest 4x4 goal (7,7) and an all-`?` clue layer.
		"guidearrow/4/4/44..........",
		// Blank-run skips and trailing junk.
		"guidearrow/3/3/22ag",
		"guidearrow/3/3/31zz",
		// 1x1, 1xN and Nx1 boards.
		"guidearrow/1/1/11",
		"guidearrow/1/4/11...",
		"guidearrow/4/1/11..."
	],
	fileios: [
		// Goal line, clues, `#` shading, `+` background marks and border lines.
		"pzprv3/guidearrow/4/4/5 3/2 . . . /. 3 . . /. . 4 . /. . . - /# . . + /. . # . /. + . . /0 1 0 /-1 0 0 /0 0 1 /0 0 0 /1 0 0 0 /0 0 -1 0 /0 0 0 0 /",
		// 6x6 real board without a border layer (`encodeBorderLineIfPresent`).
		"pzprv3/guidearrow/6/6/5 7/. . . . . 2 /. . 3 . . . /. . . 4 . . /. . . . . . /. . . 3 . . /1 . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /",
		// 1x1 board.
		"pzprv3/guidearrow/1/1/1 1/./# /"
	],
	randomize(p, r) {
		const cols = p.board.cols,
			rows = p.board.rows;
		p.board.goalpos.bx = 2 * (1 + Math.floor(r.rnd() * cols)) - 1;
		p.board.goalpos.by = 2 * (1 + Math.floor(r.rnd() * rows)) - 1;
		const goal = p.board.goalpos.getc();
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, -2, 1, 2, 3, 4]);
			// Shading is only legal on empty cells other than the goal.
			cell.qans = cell.qnum === -1 && !cell.equals(goal) ? r.pick([0, 0, 0, 1]) : 0;
			cell.qsub = cell.qans === 1 ? 0 : r.pick([0, 0, 0, 1]);
		});
		p.board.border.each((border) => {
			border.line = r.pick([0, 0, 0, 0, 1]);
			border.qsub = r.pick([0, 0, 0, 0, 2]);
		});
	}
};

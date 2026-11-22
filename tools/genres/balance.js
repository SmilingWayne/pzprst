// Fixture inputs for pzprjs pid "balance". See tools/gen_fixtures.js for the format.
// "lring" shares pzprjs's variety/balance.js but overrides Encode/FileIO, so it is a
// separate pid and is not covered here.
"use strict";

module.exports = {
	pid: "balance",
	dataset: "BalanceLoop",
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"balance/6/6/h7m7h7l7h7g7h7l7",
		"balance/13/13/x-109cnbkdx8mbj7m8xch0h1m-11i8l7g0i7g0v1g9g1gfg1gft",
		"balance/6/6/i1g9m1g1j1g8m8g0i",
		"https://puzz.link/p?balance/6/6/h7m7h7l7h7g7h7l7",
		"http://pzv.jp/p.html?type=player&balance/6/6/h7m7h7l7h7g7h7l7",
		"https://puzz.link/p?balance_edit/6/6/h7m7h7l7h7g7h7l7",
		// Rule flag alone and with an unknown flag.
		"balance/f/6/6/h7m7h7l7h7g7h7l7",
		"balance/xf/6/6/h7m7h7l7h7g7h7l7",
		"balance/f/1/1/.",
		// Truncated body, trailing junk, empty bodies, 1/N and N/1 boards.
		"balance/6/6/h7m7h7l7h",
		"balance/6/6/h7m7h7l7h7g7h7l7zz",
		"balance/6/6",
		"balance/6/6/",
		"balance/1/1/0",
		"balance/1/1/1",
		"balance/1/8/z1",
		"balance/8/1/z1",
		// `.` is an empty cell (-2 / 2 == -1); `--1` parses to -1 (no value);
		// `+100` is a large even clue; `A` is not a number16 character.
		"balance/1/1/.",
		"balance/2/2/--1",
		"balance/1/1/+100",
		"balance/4/4/A",
		"balance/4/4/d"
	],
	fileios: [
		// Real 6x6 question data with vertical/horizontal answer lines.
		"pzprv3/balance/6/6/. . 1,3 . . . /. . . . 1,3 . /. 1,3 . . . . /. . 1,3 . . 1,3 /. 1,3 . . 1,3 . /. . . . . 1,3 /1 0 -1 0 0 /0 0 0 1 0 /0 0 0 0 0 /-1 0 0 0 1 /0 0 0 0 0 /0 0 1 0 0 /1 0 0 -1 0 0 /0 0 0 0 0 1 /0 0 1 0 0 0 /0 0 0 0 -1 0 /0 1 0 0 0 0 /",
		// Config flag, blank circles in both colours and every border token.
		"pzprv3/balance/2/2/f/0,1 1,-2 /0,-2 1,3 /1 /0 /-1 0 /",
		// White and black question marks, largest 4x4 clue and answer lines.
		"pzprv3/balance/4/4/0,-2 1,-2 1,6 . /0,1 . . . /. . . . /. . . . /1 0 -1 /0 1 0 /0 0 0 /-1 0 0 /1 0 -1 0 /0 0 0 0 /0 0 0 0 /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols + p.board.rows - 2;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.5 || maxnum < 1) {
				cell.qnum = -1;
				cell.ques = 0;
			} else if (x < 0.62) {
				cell.qnum = -2;
				cell.ques = r.rnd() < 0.5 ? 1 : 0;
			} else {
				cell.qnum = 1 + Math.floor(r.rnd() * maxnum);
				cell.ques = r.rnd() < 0.4 ? 1 : 0;
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

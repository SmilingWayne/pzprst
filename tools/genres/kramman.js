// Fixture inputs for pzprjs pid "kramman". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kramman",
	dataset: null,
	urls: [
		// Real puzzles, already canonical with pzprjs.
		"kramman/6/6/150171101903ci64ih63l",
		"kramman/10/10/860f1776241480c0p0cai290l0042ai01li00073763f3i9",
		"kramman/10/10/51190e0511c775n0i9190iod3bi0m6g0c918gg30025jf1i9",
		"https://puzz.link/p?kramman/6/6/150171101903ci64ih63l",
		// pzprjs's own test URL.
		"kramman/5/5/32223i3f2fb99i",
		// Cross marks: a run of all unmarked crosses, then one dot.
		"kramman/3/3/4000",
		"kramman/3/3/03000",
		// An unknown flag letter is ignored.
		"kramman/x/3/3/4000",
		// Truncated bodies and trailing junk.
		"kramman/3/3/4",
		"kramman/3/3/4000zz",
		// The largest circle value 2.
		"kramman/2/2/1qi",
		// 1x1, 1xN and Nx1 boards.
		"kramman/1/1/00",
		"kramman/1/2/400",
		"kramman/2/1/00"
	],
	fileios: [
		// pzprjs's own answer-data fixture: a cross dot, border lines and marks.
		"pzprv3/kramman/5/5/2 . . . 1 /. 1 2 . . /. 2 1 2 . /1 . 2 1 . /. 1 . . 2 /. . . . . . /. . . . 1 . /. . . 1 . . /. . 1 . . . /. 1 . . . . /. . . . . . /1 1 -1 1 /1 1 -1 -1 /1 1 1 -1 /1 -1 1 0 /0 -1 1 0 /-1 -1 -1 -1 1 /1 1 1 -1 -1 /-1 -1 1 1 1 /0 1 1 1 1 /",
		// pzprjs's own 3x3 fixture: a center cross dot and mixed border answers.
		"pzprv3/kramman/3/3/. . . /. . . /. . . /. . . . /. . 1 . /. . . . /. . . . /1 0 /2 -1 /1 0 /0 0 0 /0 0 0 /",
		// Question marks on a circle and a cross.
		"pzprv3/kramman/2/2/- 1 /. 2 /1 . . /. - . /. . . /1 /0 /1 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qnum = x < 0.6 ? -1 : x < 0.85 ? r.pick([1, 2]) : -2;
		});
		p.board.cross.each((cross) => {
			const x = r.rnd();
			cross.qnum = x < 0.7 ? -1 : x < 0.8 ? -2 : r.pick([1, 2]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qans = x < 0.25 ? 1 : 0;
			border.qsub = x >= 0.25 && x < 0.45 ? 1 : 0;
		});
	}
};

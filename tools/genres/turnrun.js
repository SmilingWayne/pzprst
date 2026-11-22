// Fixture inputs for pzprjs pid "turnrun". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "turnrun",
	dataset: null,
	urls: [
		"turnrun/4/4",
		"turnrun/4/4/",
		"turnrun/4/4//",
		"turnrun/4/4/v",
		"https://puzz.link/p?turnrun/4/4/1234",
		"https://puzz.link/p?turnrun_edit/4/4/1234",
		"turnrun/f/4/4/1234",
		"turnrun/xf/4/4/1234",
		"turnrun/x/4/4/1234",
		"turnrun/4/4/1",
		"turnrun/3/2/1zz",
		"turnrun/4/4/0123",
		"turnrun/4/4/1234zz",
		"turnrun/4/4/.",
		"turnrun/4/4/..",
		"turnrun/4/4/-10",
		"turnrun/4/4/+100",
		"turnrun/4/4/$1",
		"turnrun/4/4/1234567890abcdef",
		"turnrun/4/4/r1",
		"turnrun/4/4/1234r1",
		"turnrun/4/4/g",
		"turnrun/1/1/0",
		"turnrun/1/1/v",
		"turnrun/1/5/01234",
		"turnrun/5/1/01234",
		"turnrun/25/1/00000",
		"turnrun/1/25/00000",
		"turnrun/8/8/k..g12g23i7g.g.j12h3m6h.42g3g15.i7h.m.h02000gq00008",
		"turnrun/6/8/g06g4k2g54h.3g1g7i6i.5g77.60.g514h1iij00200000",
		"turnrun/10/6/g.i..4g7.1j.h92h6.6h.g.h4k07g1.g3l.i5g56420001000404"
	],
	fileios: [
		"pzprv3/turnrun/3/2/f/1 2 3 /4 5 . /u 2 /0 3d /-3 1 0 /",
		"pzprv3/turnrun/4/4/0 # 1 . /2 3 . . /. . . 4 /. . . . /1 0 0 /0 1 0 /0 0 1 /0 0 0 /0 0 0 0 /0 0 0 0 /-2 0 0 0 /",
		"pzprv3/turnrun/1/2/- . /u /",
		"pzprv3/turnrun/1/1/# /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.15) {
				cell.ques = 7;
			} else if (x < 0.3) {
				cell.qnum = -2;
			} else if (x < 0.6) {
				cell.qnum = Math.floor(r.rnd() * Math.max(p.board.cols, p.board.rows));
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.35) {
				border.line = 1;
				border.qsub = 2;
			} else if (x < 0.45) {
				border.qsub = r.pick([11, 12, 13, 14]);
			}
		});
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

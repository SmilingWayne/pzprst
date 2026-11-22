// Fixture inputs for pzprjs pid "trainstations". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "trainstations",
	dataset: null,
	urls: [
		"trainstations/4/4",
		"trainstations/4/4/",
		"trainstations/4/4//",
		"trainstations/4/4/v",
		"https://puzz.link/p?trainstations/4/4/1234",
		"https://puzz.link/p?trainstations_edit/4/4/1234",
		"trainstations/f/4/4/1234",
		"trainstations/xf/4/4/1234",
		"trainstations/x/4/4/1234",
		"trainstations/4/4/1",
		"trainstations/3/2/1zz",
		"trainstations/4/4/1234zz",
		"trainstations/4/4/.",
		"trainstations/4/4/..",
		"trainstations/4/4/-10",
		"trainstations/4/4/+100",
		"trainstations/4/4/$1",
		"trainstations/4/4/1234567890abcdef",
		"trainstations/4/4/1234r1",
		"trainstations/4/4/r1",
		"trainstations/4/4/1234r1",
		"trainstations/4/4/g",
		"trainstations/1/1/0",
		"trainstations/1/1/v",
		"trainstations/1/5/01234",
		"trainstations/5/1/01234",
		"trainstations/25/1/00000",
		"trainstations/1/25/00000",
		"https://puzz.link/p?trainstations/8/8/p0h6i1g5o7j0o3g8i4h2p",
		"https://puzz.link/p?trainstations/6/6/l1l.g7j.i.h.i.j",
		"https://puzz.link/p?trainstations/12/14/r8g00g15g00g3zg00l46s0n00g0g0x0m0i0000g0j0h000j2g0h00h9j0h0h0i7nag"
	],
	fileios: [
		"pzprv3/trainstations/3/2/f/1 2 3 /4 5 . /u 2 /0 3d /-3 1 0 /",
		"pzprv3/trainstations/4/4/0 # 1 . /2 3 . . /. . . 4 /. . . . /1 0 0 /0 1 0 /0 0 1 /0 0 0 /0 0 0 0 /0 0 0 0 /-2 0 0 0 /",
		"pzprv3/trainstations/1/2/- . /u /",
		"pzprv3/trainstations/1/1/# /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.15) {
				cell.ques = 7;
			} else if (x < 0.3) {
				cell.qnum = -2;
			} else if (x < 0.6) {
				cell.qnum = Math.floor(r.rnd() * (p.board.cols * p.board.rows + 1));
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

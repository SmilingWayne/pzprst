// Fixture inputs for pzprjs pid "kurarin". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kurarin",
	dataset: null,
	urls: [
		"kurarin/6/6/g12mcl3h4m4i8m8g2m4l23h",
		"https://puzz.link/p?kurarin/6/6/g12mcl3h4m4i8m8g2m4l23h",
		"https://puzz.link/p?kurarin/8/8/n132v122u212v212u312v213o",
		"https://puzz.link/p?kurarin/8/8/h2k8j2k2k4rci8g8q8q8g4i8r8j2k3k8j2i",
		"kurarin/4/4/f",
		"kurarin/4/4/1",
		"kurarin/4/4/2fezpfzz",
		"kurarin/4/4/.",
		"kurarin/4/4/g",
		"kurarin/3/3/",
		"kurarin/3/3",
		"kurarin/1/1/e",
		"kurarin/1/1/c",
		"kurarin/1/4/em",
		"kurarin/4/1/em",
		"kurarin/2/2/i11",
		"kurarin/2/2/je",
		"kurarin/10/10/zz",
		"kurarin/4/4/A1"
	],
	fileios: [
		"pzprv3/kurarin/2/2/1.2/.../..3/# . /. + /1 /0 /-1 0 /",
		"pzprv3/kurarin/2/2/X.-/.../.../. # /. . /0 /-1 /1 0 /"
	],
	randomize(p, r) {
		for (let s = 0; s < p.board.dotsmax; s++) {
			const dot = p.board.dots[s];
			const x = r.rnd();
			if (x < 0.2) {
				dot.setDot(r.pick([1, 2, 3]));
			} else if (x < 0.26) {
				dot.setDot(-2);
			} else if (x < 0.32) {
				dot.piece.ques = 7;
			} else {
				dot.setDot(0);
			}
		}
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.3) {
				cell.qans = 1;
			} else if (x < 0.4) {
				cell.qsub = 1;
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.line = 1;
			} else if (x < 0.3) {
				border.qsub = 2;
			}
		});
	}
};

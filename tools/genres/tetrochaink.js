// Fixture inputs for pzprjs pid "tetrochaink". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tetrochaink",
	dataset: null,
	urls: [
		"tetrochaink/6/6/l44k3ict4m4k2j88n",
		"https://puzz.link/p?tetrochaink/6/6/l44k3ict4m4k2j88n",
		"tetrochaink/4/4/f",
		"tetrochaink/4/4/2fezpf",
		"tetrochaink/4/4/1",
		"tetrochaink/4/4/2fezpfzz",
		"tetrochaink/4/4/g",
		"tetrochaink/3/3/",
		"tetrochaink/3/3",
		"tetrochaink/1/1/e",
		"tetrochaink/1/1/f",
		"tetrochaink/1/4/em",
		"tetrochaink/4/1/em",
		"tetrochaink/2/2/i11",
		"tetrochaink/2/2/je",
		"tetrochaink/4/4/A1",
		"tetrochaink/10/10/zz"
	],
	fileios: [
		"pzprv3/tetrochaink/3/3/1.2../..3../...../...../.2.../# . . /. + . /. . . /",
		"pzprv3/tetrochaink/3/3/X.-../...../...../...../...../. . . /. . . /. . . /",
		"pzprv3/tetrochaink/6/6/.........../.1.1......./.....3...../.3........./.........../.........1./.........../...1......./.....2...../...2.2...../.........../# # # . . . /. # . . . . /. . # . # # /. . # . # # /. # # . . . /. . . # # # /"
	],
	randomize(p, r) {
		for (let s = 0; s < p.board.dotsmax; s++) {
			const dot = p.board.dots[s];
			const x = r.rnd();
			if (x < 0.22) {
				dot.setDot(r.pick([1, 2, 3]));
			} else if (x < 0.28) {
				dot.setDot(-2);
			} else if (x < 0.34) {
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
	}
};

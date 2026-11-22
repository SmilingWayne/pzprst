// Fixture inputs for pzprjs pid "sansaroad". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "sansaroad",
	dataset: null,
	urls: [
		"sansaroad/5/5/h4qch8h4x1g",
		"https://puzz.link/p?sansaroad/5/5/h4qch8h4x1g",
		"sansaroad/4/4/f",
		"sansaroad/4/4/2fezpf",
		"sansaroad/4/4/1",
		"sansaroad/4/4/2fezpfzz",
		"sansaroad/4/4/g",
		"sansaroad/3/3/",
		"sansaroad/3/3",
		"sansaroad/1/1/e",
		"sansaroad/1/1/f",
		"sansaroad/1/4/em",
		"sansaroad/4/1/em",
		"sansaroad/2/2/i11",
		"sansaroad/2/2/je",
		"sansaroad/4/4/A1",
		"sansaroad/10/10/zz"
	],
	fileios: [
		"pzprv3/sansaroad/3/3/1.2../..3../...../...../.2.../# . . /. + . /. . . /",
		"pzprv3/sansaroad/3/3/X.-../...../...../...../...../. . . /. . . /. . . /",
		"pzprv3/sansaroad/5/5/....1..../........./........./.3.....2./....1..../........./........./........./.......1./. . . . . /. # . # . /. . . # . /# # . . . /. . # # # /"
	],
	randomize(p, r) {
		for (let s = 0; s < p.board.dotsmax; s++) {
			const dot = p.board.dots[s];
			const x = r.rnd();
			if (x < 0.22) {
				dot.setDot(r.pick(dot.piece.group === "cell" ? [1] : [1, 2, 3]));
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

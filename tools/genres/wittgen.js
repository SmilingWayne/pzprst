// Fixture inputs for pzprjs pid "wittgen". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "wittgen",
	dataset: null,
	urls: [
		"wittgen/9/9/g1bibicjbhcjbjcjcjbhdjcibi0c",
		"wittgen/12/12/ahcibkbjdjbgblciddhblahblbhddiclbgcjbjckaibha",
		"wittgen/8/8/gckbgdkcibrckcgdka",
		"https://puzz.link/p?wittgen/9/9/g1bibicjbhcjbjcjcjbhdjcibi0c",
		"http://pzv.jp/p.html?type=player&wittgen/8/8/gckbgdkcibrckcgdka",
		"wittgen/9/9/g1bi",
		"wittgen/8/8/gckbgdkcibrckcgdkazz",
		"wittgen/2/2/4444",
		"wittgen/1/1/4",
		"wittgen/1/1/.",
		"wittgen/1/1/0",
		"wittgen/1/5/01234",
		"wittgen/5/1/01234",
		"wittgen/3/3/",
		"wittgen/4/4/z",
		"wittgen/3/3/g",
		"wittgen/10/10/000"
	],
	fileios: [
		"pzprv3/wittgen/2/2/4 4 /4 4 /9 1 /2 4 /1 /-1 /1 0 /",
		"pzprv3/wittgen/2/3/0 1 2 /3 4 - /5 0 0 /3 7 0 /1 -1 /1 1 /-1 1 1 /",
		"pzprv3/wittgen/1/1/- /8 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
			cell.line = r.pick([0, 0, 0, 1]);
			cell.qsub = r.pick([0, 0, 0, 0, 1, 2]);
			cell.qcmp = r.pick([0, 0, 0, 1]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

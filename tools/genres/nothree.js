// Fixture inputs for pzprjs pid "nothree". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "nothree",
	dataset: null,
	urls: [
		"nothree/9/9/ker48eu4eraeeaejei2esek4es446etel4el48228eq",
		"https://puzz.link/p?nothree/9/9/ker48eu4eraeeaejei2esek4es446etel4el48228eq",
		"https://puzz.link/p?nothree/13/6/geaa22eh6ehez24ejeme6c6ee4en6c42eiezieh6eh2ea2ae",
		"https://puzz.link/p?nothree/10/10/genceemeienei6eiemeeemeiemenemeiemeeemei6eieneiemecene",
		"nothree/4/4/f",
		"nothree/4/4/2fezpf",
		"nothree/4/4/1",
		"nothree/4/4/2fezpfzz",
		"nothree/4/4/g",
		"nothree/3/3/",
		"nothree/3/3",
		"nothree/1/1/e",
		"nothree/1/1/f",
		"nothree/1/4/em",
		"nothree/4/1/em",
		"nothree/2/2/i11",
		"nothree/2/2/je",
		"nothree/4/4/A1",
		"nothree/10/10/zz"
	],
	fileios: [
		"pzprv3/nothree/2/2/1.2/.../..2/# . /. + /1 /0 /0 0 /",
		"pzprv3/nothree/2/2/X.-/.../.../. # /. . /0 /-1 /0 0 /"
	],
	randomize(p, r) {
		for (let s = 0; s < p.board.dotsmax; s++) {
			const dot = p.board.dots[s];
			const x = r.rnd();
			if (x < 0.2) {
				dot.setDot(r.pick([1, 2]));
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

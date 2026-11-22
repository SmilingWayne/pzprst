// Fixture inputs for pzprjs pid "tentaisho". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tentaisho",
	dataset: null,
	urls: [
		"tentaisho/5/5/6fl94fi65en8dbf10000",
		"https://puzz.link/p?tentaisho/5/5/6fl94fi65en8dbf10000",
		"https://puzz.link/p?tentaisho/10/10/zne2ezjfgeufkejezzpfyfm0ehfmeydew2beveie",
		"https://puzz.link/p?tentaisho/6/6/ycezwezjcei",
		"https://puzz.link/p?tentaisho/13/13/ifgfzofmb2fzgegbenftefcftdeiezq7en5ezoezifgezkfmb3e4erffkfzsep3fzj9cfrfkb5eif",
		"tentaisho/4/4/f",
		"tentaisho/4/4/2fezpf",
		"tentaisho/4/4/1",
		"tentaisho/4/4/2fezpfzz",
		"tentaisho/4/4/g",
		"tentaisho/3/3/",
		"tentaisho/3/3",
		"tentaisho/1/1/e",
		"tentaisho/1/1/o",
		"tentaisho/1/4/mm",
		"tentaisho/4/1/mm",
		"tentaisho/2/2/i11",
		"tentaisho/2/2/z0",
		"tentaisho/4/4/A1",
		"tentaisho/2/2/p",
		"tentaisho/10/10/zz"
	],
	fileios: [
		"pzprv3/tentaisho/2/2/X../.1./.../1 /-1 /2 0 /0 0 /0 0 /1 0 /",
		"pzprv3/tentaisho/3/3/-..1./...../...../...../...../0 0 /1 0 /0 0 /0 0 0 /0 0 0 /0 0 0 /0 0 0 /",
		"pzprv3/tentaisho/5/5/1...2...X/........./2....1..2/........./.1...2..1/........./......1../..2....../2.....2../1 0 0 0 /1 0 0 0 /1 0 0 0 /1 0 0 0 /1 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /"
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
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.qans = 1;
				if (r.rnd() < 0.3) {
					border.qsub = 1;
				}
			} else if (x < 0.35) {
				border.qsub = 1;
			}
		});
		p.board.cell.each((cell) => {
			if (r.rnd() < 0.3) {
				cell.qsub = r.pick([1, 2]);
			}
		});
	}
};

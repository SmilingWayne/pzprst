// Fixture inputs for pzprjs pid "nuriuzu". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "nuriuzu",
	dataset: null,
	urls: [
		"nuriuzu/6/6/neyepezzoae",
		"https://puzz.link/p?nuriuzu/6/6/neyepezzoae",
		"https://puzz.link/p?nuriuzu/6/6/zznezvezg",
		"https://puzz.link/p?nuriuzu/10/10/iaaeztepexewezwepexewezzseezzj",
		"https://puzz.link/p?nuriuzu/15/15/znezzqezu6esezzzoezzzzzqereztcel0ezzkezzweztezzzzjezzezzzueu",
		"nuriuzu/c/6/6/neyepezzoae",
		"https://puzz.link/p?nuriuzu/c/6/6/neyepezzoae",
		"nuriuzu/4/4/f",
		"nuriuzu/4/4/2fezpf",
		"nuriuzu/4/4/1",
		"nuriuzu/4/4/2fezpfzz",
		"nuriuzu/4/4/g",
		"nuriuzu/3/3/",
		"nuriuzu/3/3",
		"nuriuzu/c/3/3/",
		"nuriuzu/1/1/e",
		"nuriuzu/1/4/em",
		"nuriuzu/4/1/em",
		"nuriuzu/2/2/i11",
		"nuriuzu/4/4/A1",
		"nuriuzu/10/10/zz"
	],
	fileios: [
		"pzprv3/nuriuzu/2/2/c/.../.1./.../# . /. + /",
		"pzprv3/nuriuzu/6/6/........1../.........../.........../..1......../.........1./.........../.........../.........../.........../.........../1.....1..../. . . . . . /. . . . . . /. . . . . . /. . . . . . /# # # # # # /. # . . . # /",
		"pzprv3/nuriuzu/2/2/X.-/.../.../. # /. + /"
	],
	randomize(p, r) {
		for (let s = 0; s < p.board.dotsmax; s++) {
			const dot = p.board.dots[s];
			const x = r.rnd();
			if (dot.piece.group === "cross") {
				// nuriuzu's setDot refuses cross values; store one directly so the
				// fixture proves both decoders drop it.
				if (x < 0.25) {
					dot.piece.qnum = r.pick([1, -2]);
				}
			} else if (x < 0.22) {
				dot.setDot(1);
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
		p.setConfig("nuriuzu_connect", r.rnd() < 0.3);
	}
};

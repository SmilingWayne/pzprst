// Fixture inputs for pzprjs pid "takoyaki". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "takoyaki",
	dataset: null,
	urls: [
		"takoyaki/5/5/bn8dna",
		"takoyaki/15/15/ag.icl.i.i.h.i.h.h.p.k.hd9.g.g.i.s.g.j.j.k.k.g...g.k.k.j.j.g.s.i.g.g.gad.k.p.h.h.i.h.i.i.m..ibgc",
		"takoyaki/6/6/m.ch..gcl...n",
		"https://puzz.link/p?takoyaki/5/5/bn8dna",
		"http://pzv.jp/p.html?type=player&takoyaki/5/5/bn8dna",
		"takoyaki/5/5/bn8",
		"takoyaki/5/5/bn8dnazz",
		"takoyaki/2/2/4444",
		"takoyaki/1/1/4",
		"takoyaki/1/1/.",
		"takoyaki/1/1/0",
		"takoyaki/1/5/01234",
		"takoyaki/5/1/01234",
		"takoyaki/3/3/",
		"takoyaki/4/4/z",
		"takoyaki/3/3/g",
		"takoyaki/10/10/000"
	],
	fileios: [
		"pzprv3/takoyaki/2/2/4 4 /4 4 /31 5 /2 9 /1 /-1 /1 -1 /",
		"pzprv3/takoyaki/2/3/0 1 2 /3 4 - /5 0 0 /3 7 0 /1 -1 /1 1 /-1 1 1 /",
		"pzprv3/takoyaki/1/1/- /8 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
			cell.qans = r.pick([0, 0, 0, 1]);
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

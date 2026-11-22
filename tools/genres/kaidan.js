// Fixture inputs for pzprjs pid "kaidan". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kaidan",
	dataset: null,
	urls: [
		"kaidan/7/7/ndk7cl.gck6b.kb",
		"kaidan/6/6/..j.lbjcjdl",
		"kaidan/8/8/kaci.j.k.g...s.h..d.m.i./",
		"https://puzz.link/p?kaidan/7/7/ndk7cl.gck6b.kb",
		"http://pzv.jp/p.html?type=player&kaidan/7/7/ndk7cl.gck6b.kb",
		"kaidan/7/7/ndk",
		"kaidan/7/7/ndk7cl.gck6b.kbzz",
		"kaidan/2/2/4444",
		"kaidan/2/2/..4",
		"kaidan/1/1/4",
		"kaidan/1/1/.",
		"kaidan/1/1/0",
		"kaidan/1/5/01234",
		"kaidan/5/1/01234",
		"kaidan/3/3/",
		"kaidan/4/4/z",
		"kaidan/3/3/g",
		"kaidan/10/10/000"
	],
	fileios: [
		"pzprv3/kaidan/2/2/4 4 /4 4 /31 5 /2 9 /1 /-1 /1 -1 /",
		"pzprv3/kaidan/2/3/0 1 2 /3 4 - /5 0 0 /3 7 0 /1 -1 /1 1 /-1 1 1 /",
		"pzprv3/kaidan/1/1/- /8 /"
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

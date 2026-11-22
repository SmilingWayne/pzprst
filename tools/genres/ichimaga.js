// Fixture inputs for pzprjs pid "ichimaga". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "ichimaga",
	dataset: null,
	urls: [
		"ichimaga/4/4/0a1cn",
		"https://puzz.link/p?ichimaga/4/4/0a1cn",
		"http://pzv.jp/p.html?type=player&ichimaga/4/4/0a1cn",
		"ichimaga/z/4/4/0a1cn",
		"ichimaga/4/4/0a",
		"ichimaga/4/4/0a1cnzz",
		"ichimaga/4/4/.g.g.g.g.g.g.g.g",
		"ichimaga/4/4/444444444444444e",
		"ichimaga/1/1/e",
		"ichimaga/1/5/123e",
		"ichimaga/5/1/123e",
		"ichimaga/10/10/zzzzz",
		"https://puzz.link/p?ichimaga/9/9/g667.hdj..gbg.h.hbh.jbh.jb.h.i6.bjb666b",
		"https://puzz.link/p?ichimaga/4/4/gbi2dic",
		"https://puzz.link/p?ichimaga/10/10/bgbbh46bk3dlbk2bld9e897c2belbbn6d"
	],
	fileios: [
		"pzprv3/ichimaga/3/4/def/0 . 1 4 /- 2 . 3 /. . 4 - /1 0 1 /0 0 -1 /0 1 0 /1 0 0 0 /0 0 0 0 /",
		"pzprv3/ichimaga/2/3/def/1 4 - /. 2 3 /0 1 /-1 0 /1 0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 1, 2, 3, 4, -2]);
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

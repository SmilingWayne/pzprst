// Fixture inputs for pzprjs pid "ichimagax". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "ichimagax",
	dataset: null,
	urls: [
		"ichimagax/4/4/0a1cn",
		"https://puzz.link/p?ichimagax/4/4/0a1cn",
		"http://pzv.jp/p.html?type=player&ichimagax/4/4/0a1cn",
		"ichimagax/z/4/4/0a1cn",
		"ichimagax/4/4/0a",
		"ichimagax/4/4/0a1cnzz",
		"ichimagax/4/4/.g.g.g.g.g.g.g.g",
		"ichimagax/4/4/444444444444444e",
		"ichimagax/1/1/e",
		"ichimagax/1/5/123e",
		"ichimagax/5/1/123e",
		"ichimagax/10/10/zzzzz",
		"https://puzz.link/p?ichimagax/9/9/tbjbjbgekbkbgcjejbr",
		"https://puzz.link/p?ichimagax/8/8/m.g.tbmbw.g161bg",
		"https://puzz.link/p?ichimagax/10/10/hbbg7d6bhdbo6b881c69cgciehbcgend1cg78b"
	],
	fileios: [
		"pzprv3/ichimagax/3/4/cross/0 . 1 4 /- 2 . 3 /. . 4 - /1 0 1 /0 0 -1 /0 1 0 /1 0 0 0 /0 0 0 0 /",
		"pzprv3/ichimagax/2/3/cross/1 4 - /. 2 3 /0 1 /-1 0 /1 0 0 /"
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

// Fixture inputs for pzprjs pid "ichimagam". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "ichimagam",
	dataset: null,
	urls: [
		"ichimagam/4/4/0a1cn",
		"https://puzz.link/p?ichimagam/4/4/0a1cn",
		"http://pzv.jp/p.html?type=player&ichimagam/4/4/0a1cn",
		"ichimagam/z/4/4/0a1cn",
		"ichimagam/4/4/0a",
		"ichimagam/4/4/0a1cnzz",
		"ichimagam/4/4/.g.g.g.g.g.g.g.g",
		"ichimagam/4/4/444444444444444e",
		"ichimagam/1/1/e",
		"ichimagam/1/5/123e",
		"ichimagam/5/1/123e",
		"ichimagam/10/10/zzzzz",
		"https://puzz.link/p?ichimagam/10/10/77chcj8c6bl3b68bg1dl3eg7.gb3cl8e7bjch66b",
		"https://puzz.link/p?ichimagam/10/10/idci717dgbj7dg1dg77d6d7b87bg2dg7djcg636bicbg",
		"https://puzz.link/p?ichimagam/9/9/hcobjeh7ehejbjdh8dhcjeod"
	],
	fileios: [
		"pzprv3/ichimagam/3/4/mag/0 . 1 4 /- 2 . 3 /. . 4 - /1 0 1 /0 0 -1 /0 1 0 /1 0 0 0 /0 0 0 0 /",
		"pzprv3/ichimagam/2/3/mag/1 4 - /. 2 3 /0 1 /-1 0 /1 0 0 /"
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

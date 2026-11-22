// Fixture inputs for pzprjs pid "wblink". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "wblink",
	dataset: null,
	urls: [
		"wblink/9/9/9k5j6b6a7b6a3b3j3k6k3b3j6a6",
		"wblink/9/9/7k1b3k3j3j6a242ob6b3b3jij64",
		"wblink/9/9/b3j6bfj624j5b4169fj22362bjj",
		"https://puzz.link/p?wblink/9/9/9k5j6b6a7b6a3b3j3k6k3b3j6a6",
		"http://pzv.jp/p.html?type=player&wblink/9/9/b3j6bfj624j5b4169fj22362bjj",
		"wblink/2/2/12",
		"wblink/9/9/9k5j",
		"wblink/9/9/b3j6bfj624j5b4169fj22362bjjzz",
		"wblink/2/2/.",
		"wblink/2/2/2",
		"wblink/2/2/1",
		"wblink/1/1/0",
		"wblink/1/1/1",
		"wblink/1/3/120",
		"wblink/3/1/120",
		"wblink/3/3/z",
		"wblink/10/10/000"
	],
	fileios: [
		"pzprv3/wblink/2/2/1 2 /- . /1 /-1 /1 0 /",
		"pzprv3/wblink/3/3/. . 1 /. 2 . /- . . /0 1 /-1 0 /0 0 /1 0 0 /0 1 0 /",
		"pzprv3/wblink/1/1/- /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 1, 2, -2]);
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

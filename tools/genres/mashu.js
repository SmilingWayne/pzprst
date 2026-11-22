// Fixture inputs for pzprjs pid "mashu". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "mashu",
	dataset: "Masyu",
	urls: [
		"mashu/6/6/00016f769000",
		"masyu/6/6/00016f769000",
		"pearl/6/6/00016f769000",
		"https://puzz.link/p?masyu/6/6/00016f769000",
		"mashu/f/6/6/00016f769000",
		"mashu/6/6/0001",
		"mashu/6/6/00016f769000zz",
		"mashu/6/6/1063000i3000",
		"http://pzv.jp/p.html?type=player&mashu/3/3/a",
		"mashu/3/3/z1",
		"mashu/3/3/A1",
		"mashu/3/3/",
		"mashu/1/1/0",
		"mashu/1/1/i",
		"mashu/2/1/q",
		"mashu/10/10/000j5m0i03603a020000603and1900010i",
		"mashu/11/9/0106900044020201090571i0000001923"
	],
	fileios: [
		"pzprv3/mashu/2/2/- 1 /2 . /1 /0 /-1 1 /",
		"pzprv3/mashu/2/2/f/. . /. . /0 /0 /0 0 /"
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
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

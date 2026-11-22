// Fixture inputs for pzprjs pid "hitori". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "hitori",
	dataset: "Hitori",
	urls: [
		"https://puzz.link/p?hitori/8/8/0452137432741116746252121816354754317268872136453541672161253484",
		"https://puzz.link/p?hitori/8/8/1273364412433856411651216924834756524915253515748369676237817293",
		"https://puzz.link/p?hitori/8/8/2871145262411735111522438254361137364511562471384563487473845326",
		"hitori/3/3/123456789",
		"https://puzz.link/p?hitori/3/3/123456789",
		"http://pzv.jp/p.html?type=player&hitori/3/3/123456789",
		"hitori/z/3/3/123456789",
		"hitori/2/2/@%12",
		"hitori/2/2/-zz..",
		"hitori/2/2/00..",
		"hitori/2/2/AB12",
		"hitori/1/1/1",
		"hitori/1/5/12345",
		"hitori/5/1/12345",
		"hitori/3/3/12",
		"hitori/3/3/123456789zz",
		"hitori/3/3/",
		"hitori/4/4/aZ12-zz0"
	],
	fileios: [
		"pzprv3/hitori/3/3/1 2 - /0 3 36 /1295 5 . /# . . /. + . /. . . /1 0 /0 0 /0 -1 /0 0 0 /0 0 0 /",
		"pzprv3/hitori/2/2/1 - /. . /. . /# + /",
		"pzprv3/hitori/3/2/- . /1295 0 /35 . /. # /+ # /. + /1 /0 /0 /0 -1 /1 0 /"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows);
		const pool = [-1, -1, -1, -2];
		for (let n = 1; n <= maxnum; n++) {
			pool.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(pool);
			const x = r.rnd();
			if (x < 0.25) {
				cell.qans = 1;
			} else if (x < 0.4) {
				cell.qsub = 1;
			}
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

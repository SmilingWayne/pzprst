// Fixture inputs for pzprjs pid "fracdiv". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "fracdiv",
	dataset: null,
	urls: [
		"fracdiv/3/5/4.h23h2.j.k3.h32h5./",
		"fracdiv/6/5/3.2.1.4.5.9.zn.g3w",
		"https://puzz.link/p?fracdiv/8/8/p1517h3e17l1517h3e.y1.1.r1.1.x1.1.h1.1.l1.1.h1.1.p",
		"fracdiv/3/3/12a37.0...099.+3e7+3e7h",
		"https://puzz.link/p?fracdiv/3/3/12a37.0...099.+3e7+3e7h",
		"http://pzv.jp/p.html?type=player&fracdiv/3/3/12a37.0...099.+3e7+3e7h",
		"fracdiv/z/3/3/12a37.0...099.+3e7+3e7h",
		"fracdiv/1/1/+3e7+3e7",
		"fracdiv/1/1/0.",
		"fracdiv/1/1/.",
		"fracdiv/1/1/a.",
		"fracdiv/1/5/0.0.0.0.0.",
		"fracdiv/5/1/0.0.0.0.0.",
		"fracdiv/2/2/zz",
		"fracdiv/2/2/1",
		"fracdiv/2/2/010.5...",
		"fracdiv/3/3/12a37.0...099.+3e7+3e7hzz"
	],
	fileios: [
		"pzprv3/fracdiv/3/3/1,2 10,3 d7 /# d-2 0,9 /d9 999,999 . /1 0 /0 1 /0 -1 /0 2 0 /0 0 0 /",
		"pzprv3/fracdiv/2/2/0,1 # /d5 d-2 /-1 /1 /2 0 /",
		"pzprv3/fracdiv/1/1/-1,-1 /0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.45) {
				cell.ques = 51;
				cell.qnum = r.pick([0, 1, 2, 3, 4, 5, 9, 10, 99, 255, 256, 999]);
				cell.qnum2 = r.pick([0, 1, 2, 3, 4, 5, 9, 10, 99, 255, 256, 999]);
			} else if (x < 0.6) {
				cell.ques = 7;
			} else {
				cell.qnum = r.pick([-1, -1, -1, -2, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
			}
		});
		p.board.border.each((border) => {
			border.qans = r.rnd() < 0.3 ? 1 : 0;
			border.qsub = r.rnd() < 0.3 ? 1 : 0;
		});
	}
};

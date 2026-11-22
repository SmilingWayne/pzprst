// Fixture inputs for pzprjs pid "topo". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "topo",
	dataset: null,
	urls: [
		"topo/5/5/2h5i8jczmg3k9k-11p-63",
		"https://puzz.link/p?topo/5/5/2h5i8jczmg3k9k-11p-63",
		"topo/5/5/zz0zi0",
		"topo/4/4/01-10-11-62y",
		"topo/3/3/.qj.j",
		"topo/5/5/2h5i8",
		"topo/5/5/2h5i8zz",
		"topo/1/1/-63",
		"topo/1/1/.",
		"topo/1/1/0",
		"topo/1/5/12345",
		"topo/5/1/12345",
		"topo/4/4/000g",
		"topo/4/4/",
		"http://pzv.jp/p.html?type=player&topo/3/3/.qj.j"
	],
	fileios: [
		"pzprv3.1\ntopo\n2\n2\n3 \n. \n. 98 \n0 . \n. 99 \nc 5 \n+ . \n0 \n1 \n-1 0 \n",
		"pzprv3.1\ntopo\n2\n3\nt\n-2 . \n. . \n. . . \n. . - \n. . . \n. . . \n7 c - \n0 0 \n0 0 \nX -X 0 \n",
		"pzprv3\ntopo\n2\n2\n3 \n. \n. 98 \nc 5 \n+ . \n0 \n1 \n-1 0 \n"
	],
	randomize(p, r) {
		p.board.border.each((border) => {
			border.qnum = r.pick([-1, -1, -1, -1, -1, -2, 0, 1, 3, 12, 16, 17, 98]);
			border.qans = 0;
			border.qsub = 0;
			border.ques = 0;
			const x = r.rnd();
			if (x < 0.1) {
				border.qans = 1;
			} else if (x < 0.2) {
				border.qsub = 1;
			} else if (x < 0.28) {
				border.ques = 1;
			} else if (x < 0.34) {
				border.ques = 1;
				border.qsub = 1;
			}
		});
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, -2, 0, 1, 5, 12, 15, 16, 17, 99]);
			cell.anum = -1;
			cell.qcmp = 0;
			cell.qsub = 0;
			const x = r.rnd();
			if (x < 0.12) {
				cell.anum = r.pick([0, 1, 7, 99]);
			} else if (x < 0.24) {
				cell.qcmp = 1;
			} else if (x < 0.36) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
		p.setConfig("fillomino_tri", r.rnd() < 0.3);
	}
};

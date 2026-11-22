// Fixture inputs for pzprjs pid "rectslider". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "rectslider",
	dataset: null,
	urls: [
		"https://puzz.link/p?rectslider/8/8/p..l.3g1.l.2h2.l2.g5.l.1p",
		"https://puzz.link/p?rectslider/8/9/o.1.1j1i2i0i3i....j.g3k1h2j0i3p",
		"https://puzz.link/p?rectslider/10/10/l4m4m4j.h4j.m.j4h.j.m.j.h4j.m4m.l",
		"http://pzv.jp/p.html?type=player&rectslider/4/4/1u",
		"rectslider/4/4/1u",
		"rectslider/4/4/",
		"rectslider/4/4/1",
		"rectslider/4/4/1uzz",
		"rectslider/4/4/.u",
		"rectslider/7/7/6zzn",
		"rectslider/1/1/g",
		"rectslider/1/5/1j",
		"rectslider/5/1/1j",
		"rectslider/z/4/4/1u"
	],
	fileios: [
		"pzprv3/rectslider/4/4/1 . . . /. . . . /. . . . /. . . . /0 0 0 17 /32 0 0 0 /0 33 0 0 /0 0 0 48 /1 0 0 /0 -1 0 /0 0 0 /0 0 0 /0 0 0 0 /0 0 0 0 /-1 0 0 0 /",
		"pzprv3/rectslider/3/2/1 . /. . /. . /0 64 /0 0 /16 0 /1 /0 /0 /-1 0 /0 0 /",
		"pzprv3/rectslider/1/5/1 . . . 3 /0 0 0 0 32 /1 0 -1 0 /"
	],
	randomize(p, r) {
		const cols = p.board.cols;
		const rows = p.board.rows;
		p.board.cell.each((cell) => {
			const c = cell.bx >> 1;
			const d = cell.by >> 1;
			const maxnum = Math.max(c, cols - 1 - c, d, rows - 1 - d);
			cell.qnum = r.pick([-1, -1, -1, -1, -2, 0, Math.min(1, maxnum), maxnum]);
			cell.qsub = r.pick([0, 0, 0, 0, 1, 2, 15]);
			cell.qcmp = r.pick([0, 0, 0, 1]);
			cell.anum = r.pick([-1, -1, -1, 0]);
		});
		p.board.border.each((border) => {
			const y = r.rnd();
			border.line = y < 0.3 ? 1 : 0;
			border.qsub = y >= 0.3 && y < 0.45 ? 2 : 0;
		});
	}
};

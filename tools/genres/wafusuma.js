// Fixture inputs for pzprjs pid "wafusuma". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "wafusuma",
	dataset: null,
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"wafusuma/10/10/u78g.h.h78m69m7am99p.i.zo.i886m475zs.iab95la976u",
		"https://puzz.link/p?wafusuma/10/10/u78g.h.h78m69m7am99p.i.zo.i886m475zs.iab95la976u",
		"wafusuma/6/6/g.g.h.g.h.g.h.g.h.g.h.g.m4.....l...5.5l",
		"wafusuma/10/10/r.h.i.o.i..i...p.j...n..k.j.k.j-64j.h.i.k.h.h.q.q.l.h.o.r.j.g.g",
		"wafusuma/4/4/3h8i-10vg2k9k-10g1",
		"wafusuma/1/1/1",
		"wafusuma/1/1/./",
		"wafusuma/3/3/.q3i.j",
		"wafusuma/5/5/3456789azrh7zh",
		"wafusuma/6/6/g.g.h.g.h.g.h.g",
		"wafusuma/6/6/g.g.h.g.h.g.h.g.h.g.h.g.m4.....l...5.5lzz",
		"wafusuma/1/5/3456",
		"wafusuma/5/1/3456",
		"wafusuma/4/4/",
		"http://pzv.jp/p.html?type=player&wafusuma/4/4/3h8i-10vg2k9k-10g1"
	],
	fileios: [
		"pzprv3.1\nwafusuma\n2\n2\n3 \n. \n. 4 \n1 . \n. 4 \nc 2 \n= . \n0 \n1 \n-1 0 \n",
		"pzprv3.1\nwafusuma\n2\n3\nt\n-2 . \n. . \n. . . \n. . - \n. . . \n. . . \n3 c % \n0 0 \n0 0 \nX -X 0 \n",
		"pzprv3\nwafusuma\n2\n2\n3 \n. \n. 4 \nc 2 \n= . \n0 \n1 \n-1 0 \n"
	],
	randomize(p, r) {
		const tri = r.rnd() < 0.3;
		p.setConfig("fillomino_tri", tri);
		const maxcell = tri ? 3 : p.board.cols * p.board.rows;
		const maxborder = p.board.cols * p.board.rows;
		const cellvals = [-1, -1, -1, -1, -1, -2];
		for (let n = 1; n <= Math.min(maxcell, 16); n++) {
			cellvals.push(n);
		}
		const bordervals = [-1, -1, -1, -1, -1, -2];
		for (let n = 3; n <= Math.min(maxborder, 20); n++) {
			bordervals.push(n);
		}
		p.board.border.each((border) => {
			border.qnum = r.pick(bordervals);
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
			cell.qnum = r.pick(cellvals);
			cell.anum = -1;
			cell.qcmp = 0;
			cell.qsub = 0;
			const x = r.rnd();
			if (x < 0.12) {
				cell.anum = r.pick([0, 1, 2, maxcell]);
			} else if (x < 0.24) {
				cell.qcmp = 1;
			} else if (x < 0.36) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
	}
};

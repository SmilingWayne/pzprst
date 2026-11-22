// Fixture inputs for pzprjs pid "shakashaka". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "shakashaka",
	dataset: "Shakashaka",
	urls: [
		"shakashaka/3/3/00aj",
		"shakashaka/10/10/cl.scj.n.kbicjch2bh.ibtbk.g",
		"https://puzz.link/p?shakashaka/10/10/cl.scj.n.kbicjch2bh.ibtbk.g",
		"shakashaka/17/13/q.j.znblagcjboczmcockbgc.icwckegbs.m.lcpcj",
		"https://puzz.link/p?shakashaka/17/13/q.j.znblagcjboczmcockbgc.icwckegbs.m.lcpcj",
		"shakashaka/10/10/g.ibvclbzdibgc.ibzdb",
		"https://puzz.link/p?shakashaka/10/10/g.ibvclbzdibgc.ibzdb",
		"shakashaka/2/2/4444",
		"shakashaka/2/2/....",
		"shakashaka/1/1/0",
		"shakashaka/1/1/.",
		"shakashaka/1/1/4",
		"shakashaka/1/1/000",
		"shakashaka/1/5/00000",
		"shakashaka/5/1/00000",
		"shakashaka/4/4/00",
		"shakashaka/x/3/3/00aj",
		"shakashaka/3/3/00ajzz",
		"shakashaka/3/3/",
		"shakashaka/3/3",
		"shakashaka/3/3/g0",
		"shakashaka/2/2/ZZ",
		"shakashaka/2/2/z",
		"shakashaka/4/4/0123",
		"shakashaka/3/3/e0",
		"shakashaka_edit/3/3/00aj",
		"http://pzv.jp/p.html?type=player&shakashaka/3/3/000"
	],
	fileios: [
		"pzprv3/shakashaka/2/2/0 5 /. 4 /. + /- 2 /",
		"pzprv3/shakashaka/3/3/. 0 5 /4 . 2 /3 . 4 /+ 5 . /- + 2 /. - 3 /",
		"pzprv3/shakashaka/1/1/5 /+ /",
		"pzprv3/shakashaka/4/4/0 1 2 3 /4 . 5 . /. . . . /2 2 . 4 /2 3 4 5 /. . + - /5 . 2 . /+ - . 3 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
			cell.qans = r.pick([0, 0, 0, 0, 2, 3, 4, 5]);
			// The three answer properties share one FileIO token; pzprjs writes
			// qans first, then qsub, then qcmp.
			if (cell.qans === 0) {
				cell.qsub = r.pick([0, 0, 0, 1]);
				cell.qcmp = r.pick([0, 0, 0, 1]);
			} else {
				cell.qsub = 0;
				cell.qcmp = 0;
			}
		});
	}
};

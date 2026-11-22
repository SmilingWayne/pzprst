// Fixture inputs for pzprjs pid "kakuru". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kakuru",
	dataset: "Kakkuru",
	urls: [
		"kakuru/5/5/3.a+4+mD.S.bm+g+A.3",
		"kakuru/6/6/e.eB._.KmBkK.hK.Uk9mH.i.ed.d",
		"kakuru/5/5/9.a.8.c.e.7.a.a.c.a.8.b.3",
		"https://puzz.link/p?kakuru/6/6/e.eB._.KmBkK.hK.Uk9mH.i.ed.d",
		"http://pzv.jp/p.html?type=player&kakuru/4/4/Y",
		"kakuru/1/1/1",
		"kakuru/1/1/_",
		"kakuru/2/2/12",
		"kakuru/3/3/",
		"kakuru/3/3/1.2",
		"kakuru/3/3/1.2zz",
		"kakuru/1/3/2",
		"kakuru/3/1/2",
		"kakuru/3/3/++++",
		"kakuru/4/4/Y",
		"kakuru/2/2/+_"
	],
	fileios: [
		"pzprv3/kakuru/5/5/3 . 10 b 4 /b . . . . /23 . 38 . 11 /. . . . b /16 b 20 . 3 /. 2 . . . /. 1 4 3 1 /. 6 . 5 . /7 9 8 2 . /. . . 1 . /",
		"pzprv3/kakuru/5/5/3 . 10 b 4 /b . . . . /23 . 38 . 11 /. . . . b /16 b 20 . 3 /. 0 . . . /. 0 0 0 0 /. 0 . 0 . /0 0 0 0 . /. . . 0 . /",
		"pzprv3/kakuru/2/2/? b /5 . /. . /. + /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 1, 2, 5, 10, 19, 20, 44, -2]);
			cell.ques = cell.qnum === -1 && r.rnd() < 0.15 ? 1 : 0;
			cell.anum = r.pick([-1, -1, -1, 1, 2, 3, 9]);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
		});
	}
};

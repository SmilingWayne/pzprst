// Fixture inputs for pzprjs pid "numrope". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "numrope",
	dataset: null,
	urls: [
		"numrope/4/4/p2gg98l2lael+l",
		"numrope/7/7/a89554gigasg1409qgeqA.el+nbk+k8n+lB.iq8",
		"numrope/7/7/pkkk199co02la2la00l3lbnbqA.g.gqAnelbl",
		"https://puzz.link/p?numrope/6/6/r09g4aiq8aa9pek+m+mbm8kD.+n",
		"http://pzv.jp/p.html?type=player&numrope/2/2/00Ql",
		"numrope/3/3/0000r",
		"numrope/1/1/./",
		"numrope/2/2/00Ql",
		"numrope/3/3/k0a0cq",
		"numrope/2/2/00_+5./",
		"numrope/2/2/003l",
		"numrope/4/4/",
		"numrope/3/3/0000",
		"numrope/3/3/0000rzz",
		"numrope/1/3/0l",
		"numrope/3/1/0l",
		"numrope/10/10/"
	],
	fileios: [
		"pzprv3/numrope/3/3/12 . . /. . . /. . . /1 0 /1 0 /0 0 /0 1 0 /1 0 0 /. 0 0 /0 0 0 /0 0 0 /",
		"pzprv3/numrope/2/2/? b /5 . /0 /0 /0 0 /. . /. + /",
		"pzprv3/numrope/3/3/. . . /. . . /. . . /0 0 /0 0 /0 0 /0 0 0 /0 0 0 /0 0 0 /0 0 0 /0 0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 1, 2, 5, 10, 19, 20, 36, -2]);
			cell.ques = cell.qnum === -1 && r.rnd() < 0.15 ? 1 : 0;
			cell.anum = r.pick([-1, -1, -1, 1, 2, 3, 9]);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
		});
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.25 ? 1 : 0;
		});
	}
};

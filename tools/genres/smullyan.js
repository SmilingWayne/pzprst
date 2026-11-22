// Fixture inputs for pzprjs pid "smullyan". See tools/gen_fixtures.js for the format.
// "context" shares pzprjs's variety/context.js but with Cell.maxnum 4, so it is
// a separate pid and is not covered here. No dataset or sample URLs exist for
// this pid; the URL inputs are hand-written grids validated by pzprjs.
"use strict";

module.exports = {
	pid: "smullyan",
	dataset: null,
	urls: [
		"smullyan/8/8/k1j2i3h2h3n1j3n1h2h2i2j2k",
		"https://puzz.link/p?smullyan/6/6/i2h0i2h1i1h2l3h1i0g",
		"smullyan/9/9/j21g1g13k2l2k3k21i1i11k3k2l2k11g1g11j",
		"http://pzv.jp/p.html?smullyan/8/8/k1j2i3h2h3n1j3n1h2h2i2j2k",
		"smullyan_edit/4/4/0",
		"http://pzv.jp/p.html?type=player&smullyan/4/4/0",
		"smullyan/4/4/0",
		"smullyan/4/4",
		"smullyan/8/8/k1j",
		"smullyan/8/8/k1j2i3h2h3n1j3n1h2h2i2j2kzz",
		"smullyan/4/4/9999999999999999",
		"smullyan/3/3/...",
		"smullyan/4/4/....",
		"smullyan/3/3/Z",
		"smullyan/9/9/-50",
		"smullyan/3/3/012345678",
		"smullyan/1/1/0",
		"smullyan/1/4/0",
		"smullyan/4/1/0",
		"smullyan/16/16/0000000000000000000000000000"
	],
	fileios: [
		"pzprv3/smullyan/2/2/0 - /9 . /# + /. # /0 /1 /0 -1 /",
		"pzprv3/smullyan/4/4/0 1 2 3 /4 5 6 7 /8 9 . 0 /1 2 3 4 /# + . # /+ # + . /. # . + /# . # + /0 1 -1 /1 0 0 /-1 0 1 /0 1 0 /1 0 -1 1 /0 1 0 0 /1 0 1 -1 /",
		"pzprv3/smullyan/6/6/. . . 2 . . /0 . . . 9 . /. 1 . . . 1 /. . 2 . . . /. . . 3 . . /1 . . . 0 . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -2, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
			cell.qans = r.rnd() < 0.25 ? 1 : 0;
			cell.qsub = r.rnd() < 0.25 ? 1 : 0;
		});
		p.board.border.each((border) => {
			border.line = 0;
			border.qsub = 0;
			const x = r.rnd();
			if (x < 0.2) {
				border.line = 1;
			} else if (x < 0.35) {
				border.qsub = 2;
			}
		});
	}
};

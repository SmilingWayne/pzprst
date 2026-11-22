// Fixture inputs for pzprjs pid "context". See tools/gen_fixtures.js for the format.
// "smullyan" shares pzprjs's variety/context.js but raises Cell.maxnum to 9, so
// it is a separate pid and is not covered here.
"use strict";

module.exports = {
	pid: "context",
	dataset: null,
	urls: [
		"context/8/8/k1j2i3h2h3n1j3n1h2h2i2j2k",
		"https://puzz.link/p?context/9/9/j21g1g13k2l2k3k21i1i11k3k2l2k11g1g11j",
		"https://puzz.link/p?context/6/6/i2h0i2h1i1h2l3h1i0g",
		"http://pzv.jp/p.html?context/8/8/k1j2i3h2h3n1j3n1h2h2i2j2k",
		"context_edit/4/4/0",
		"http://pzv.jp/p.html?type=player&context/4/4/0",
		"context/4/4/0",
		"context/4/4",
		"context/8/8/k1j",
		"context/8/8/k1j2i3h2h3n1j3n1h2h2i2j2kzz",
		"context/4/4/4444444444444444",
		"context/3/3/...",
		"context/4/4/....",
		"context/3/3/Z",
		"context/9/9/-50",
		"context/3/3/012345678",
		"context/1/1/0",
		"context/1/4/0",
		"context/4/1/0",
		"context/16/16/0000000000000000000000000000"
	],
	fileios: [
		"pzprv3/context/2/2/0 - /2 . /# + /. # /0 /1 /0 -1 /",
		"pzprv3/context/4/4/0 1 2 3 /4 . 0 1 /2 3 4 . /. 0 1 2 /# + . # /+ # + . /. # . + /# . # + /0 1 -1 /1 0 0 /-1 0 1 /0 1 0 /1 0 -1 1 /0 1 0 0 /1 0 1 -1 /",
		"pzprv3/context/6/6/. . . 2 . . /0 . . . 2 . /. 1 . . . 1 /. . 2 . . . /. . . 3 . . /1 . . . 0 . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -2, 0, 1, 2, 3, 4]);
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

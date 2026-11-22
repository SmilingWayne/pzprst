// Fixture inputs for pzprjs pid "norinuri". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "norinuri",
	dataset: "Norinori",
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"norinuri/10/10/iazranah9mazsbo",
		"norinuri/10/10/g6p8l.n-14zzhan4lap.g",
		"norinuri/10/10/.wak9k6x.s5r8h7g2r4k",
		"https://puzz.link/p?norinuri/10/10/iazranah9mazsbo",
		"http://pzv.jp/p.html?type=player&norinuri/10/10/iazranah9mazsbo",
		"norinuri/10/10/iazranah9mazsbo/",
		"norinuri/10/10/iazranah9mazsbozz",
		"norinuri/10/10/iazra",
		"norinuri/3/2/1g2g.3",
		"norinuri/3/3/1.2",
		"norinuri/4/4/",
		"norinuri/1/1/1",
		"norinuri/1/1/.",
		"norinuri/1/1/",
		"norinuri/1/5/12345",
		"norinuri/5/1/12345",
		"norinuri/4/4/+010",
		"norinuri/f/4/4/1234",
		"norinuri/3/3/zzz",
		"norinuri/6/6"
	],
	fileios: [
		"pzprv3/norinuri/2/3/1 . 2 /. - 3 /# . + /. # . /",
		"pzprv3/norinuri/2/2/. . /. - /. + /. # /",
		"pzprv3/norinuri/1/1/1 /+ /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 1; n <= Math.min(maxnum, 8); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			const x = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.35) {
				cell.qsub = 1;
			}
		});
	}
};

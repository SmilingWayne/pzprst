// Fixture inputs for pzprjs pid "shugaku". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "shugaku",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"shugaku/17/13/82c29082c3d1r463a3a2e2g25g3d5c2a4m5721362o1j3c4a1b1c",
		"shugaku/17/13/a2j2a365185c5w3a38393a37481d595c2g58182p2816565b3e5817293652p",
		"shugaku/17/13/2n283p273751c3l2f31859365n2j172c12o3e539593j1a29561b",
		"https://puzz.link/p?shugaku/17/13/82c29082c3d1r463a3a2e2g25g3d5c2a4m5721362o1j3c4a1b1c",
		"http://pzv.jp/p.html?type=player&shugaku/17/13/82c29082c3d1r463a3a2e2g25g3d5c2a4m5721362o1j3c4a1b1c",
		// Question marks, the maximum clue, and run lengths from `6` to `z`.
		"shugaku/2/2/0123",
		"shugaku/4/4/5",
		"shugaku/4/4/z",
		"shugaku/4/4/5555",
		"shugaku/2/2/0123zz",
		"shugaku/6/6/0123",
		"shugaku/1/1/4",
		"shugaku/1/5/01234",
		"shugaku/5/1/01234",
		"shugaku/3/3/0",
		"shugaku/z/4/4/0"
	],
	fileios: [
		// Every question token and both background marks.
		"pzprv3/shugaku/2/3/0 a 5 /- 4 # /",
		// All five pillow/cushion answers without a pillow and a cushioned one.
		"pzprv3/shugaku/2/2/f j /b - /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = -1;
			cell.qans = 0;
			cell.qsub = 0;
			const x = r.rnd();
			if (x < 0.2) {
				cell.qnum = -2;
			} else if (x < 0.4) {
				cell.qnum = r.pick([0, 1, 2, 3, 4]);
			} else if (x < 0.55) {
				cell.qans = 1;
			} else if (x < 0.8) {
				cell.qans = r.pick([41, 42, 43, 44, 45, 46, 47, 48, 49, 50]);
			} else if (x < 0.9) {
				cell.qsub = 1;
			}
		});
	}
};

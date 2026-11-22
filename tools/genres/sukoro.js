// Fixture inputs for pzprjs pid "sukoro". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "sukoro",
	dataset: "Sukoro",
	urls: [
		"https://puzz.link/p?sukoro/10/10/1b3e1f14e2f1e3b1a2j3b2o1g1f3a1b2h",
		"sukoro/10/10/1b3e1f14e2f1e3b1a2j3b2o1g1f3a1b2h",
		"https://puzz.link/p?sukoro/10/10/1l3c2c1c2c2b3c3c1b4m2a1e2b2h3k31c1a3c",
		"sukoro/6/6/i3c3f3b3i3b",
		"http://pzv.jp/p.html?type=player&sukoro/6/6/i3c3f3b3i3b",
		"sukoro/6/6/i3c3f3b3i3b/",
		"sukoro/6/6/i3c3f3b3i3b//",
		"sukoro/6/6/i3c3f3b3i3bzz",
		"sukoro/10/10/1b3e1f14e2f1e3b1a2j3b2o1g1f3a1b2hzz",
		"sukoro/6/6/i3c3",
		"sukoro/4/4/....",
		"sukoro/4/4/4444",
		"sukoro/4/4/1234",
		"sukoro/4/4/0000000000000000",
		"sukoro/3/3/A1",
		"sukoro/3/3/q1",
		"sukoro/2/2/99",
		"sukoro/1/1/1",
		"sukoro/1/1/",
		"sukoro/1/10/41",
		"sukoro/10/1/1234",
		"sukoro/9/9/123456789123456789123456789123456789123456789123456789123456789123456789123456789"
	],
	fileios: [
		"pzprv3/sukoro/2/2/1 . /. 4 /c . /. + /",
		"pzprv3/sukoro/3/2/1 . /- 4 /. 2 /c + /. 3 /- % /",
		"pzprv3/sukoro/1/1/. /0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 1, 2, 3, 4, -2]);
			const x = r.rnd();
			if (x < 0.2) {
				cell.anum = Math.floor(r.rnd() * 6);
			} else if (x < 0.35) {
				cell.qcmp = 1;
			} else if (x < 0.5) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
	}
};

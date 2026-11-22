// Fixture inputs for pzprjs pid "anglers". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "anglers",
	dataset: null,
	urls: [
		"anglers/10/10/zl1p1t1p1s1zl-15h-15j-15r-15-15s",
		"anglers/10/10/-17z9m1p1g-117r11g.p9m1z1zz",
		"anglers/8/8/i1l1w1k1l1p1n1h8l87q6k.h..g",
		"https://puzz.link/p?anglers/8/8/i1l1w1k1l1p1n1h8l87q6k.h..g",
		"http://pzv.jp/p.html?type=player&anglers/10/10/zl1p1t1p1s1zl-15h-15j-15r-15-15s",
		"anglers/4/4/",
		"anglers/4/4/0000000000000000",
		"anglers/4/4/0000000000000000zz",
		"anglers/4/4/-10",
		"anglers/4/4/.",
		"anglers/1/1/",
		"anglers/1/4/",
		"anglers/4/1/",
		"anglers/2/2/",
		"anglers/3/3/000000000000000000"
	],
	fileios: [
		"pzprv3\nanglers\n3\n3\n. 2 1 0 . \n1 # 0 . 3 \n2 . 1 # 2 \n. . . . . \n. 1 2 0 . \n1 0 0 1 \n0 -1 0 0 \n1 0 1 0 \n0 1 0 \n1 0 0 \n0 -1 0 \n0 0 1 \n",
		"pzprv3\nanglers\n2\n2\n. 1 . . \n. # # . \n2 # . 1 \n. . 2 . \n0 1 0 \n0 0 1 \n0 1 \n0 0 \n1 0 \n"
	],
	randomize(p, r) {
		const max = p.board.cols * p.board.rows;
		const clues = [-1, -1, -1, -1, -3, 0, 0, 1, Math.floor(max / 2), max];
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
		});
		p.board.excell.each((excell) => {
			excell.qnum = r.pick(clues);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.line = 1;
			} else if (x < 0.3) {
				border.qsub = 2;
			}
		});
	}
};

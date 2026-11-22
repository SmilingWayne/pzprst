// Fixture inputs for pzprjs pid "sashikazune". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "sashikazune",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"sashikazune/9/9/h43j34m14j3k42g1u4g31k3j31m23j33h",
		"sashikazune/8/8/i3g5h1g2g4h6zk3n3k6h3h6g",
		"sashikazune/10/10/4l34g2r2k37j2h2g6k3l3k2g1h4j33k2r4g22l6",
		"https://puzz.link/p?sashikazune/8/8/i3g5h1g2g4h6zk3n3k6h3h6g",
		"http://pzv.jp/p.html?type=player&sashikazune/8/8/i3g5h1g2g4h6zk3n3k6h3h6g",
		"sashikazune/8/8/i3g5h1g2g4h6zk3n3k6h3h6g/",
		"sashikazune/4/4/1zz",
		"sashikazune/8/8/i3g5h1g2g4h6zk3n3k",
		"sashikazune/4/4/.",
		"sashikazune/6/6/6",
		"sashikazune/4/4/+3e7",
		"sashikazune/6/6/-12",
		"sashikazune/1/1/1",
		"sashikazune/1/5/5",
		"sashikazune/5/1/5",
		"sashikazune/6/6",
		"sashikazune/3/3/z1",
		"sashikazune/4/4/1G",
		"sashikazune/x/4/4/1",
		"sashikazune/f/4/4/1"
	],
	fileios: [
		// Real 8x8 puzzle with answer borders and direction-aux marks.
		"pzprv3/sashikazune/8/8/. . . 3 . 5 . . /1 . 2 . 4 . . 6 /. . . . . . . . /. . . . . . . . /. . . . . . . . /. 3 . . . . . . /. . 3 . . . . . /6 . . 3 . . 6 . /0 2 0 0 0 0 0 /0 0 0 1 0 0 0 0 /0 0 0 0 0 0 0 0 /0 0 0 0 0 0 0 0 /-1 0 0 0 0 0 0 0 /0 0 0 0 0 0 0 0 /0 0 0 0 0 0 0 0 /0 0 1 0 0 0 -1 0 /0 0 0 1 0 0 0 /0 0 0 0 0 -1 0 0 0 /0 2 0 0 0 0 0 0 /0 0 0 0 0 0 0 0 /1 0 0 0 0 0 0 0 /0 0 0 0 0 0 0 /0 0 0 0 0 0 0 0 /0 0 0 2 0 0 0 0 /0 0 0 0 0 0 0 0 /2 0 0 4 /0 0 0 0 0 0 0 0 /0 16 0 0 0 0 0 0 /0 0 0 0 0 0 0 0 /0 0 0 0 0 0 0 0 /0 0 0 0 0 0 0 0 /0 0 2 0 0 0 0 0 /0 0 0 0 4 0 0 0 /0 0 0 0 0 0 0 0 /",
		// Question mark, the largest clue, every answer token and qsub bits.
		"pzprv3/sashikazune/2/3/. - 3 /2 . 3 /2 1 /-1 0 /1 2 -1 /2 0 4 /0 16 0 /",
		// Empty board: every layer present and zero.
		"pzprv3/sashikazune/3/3/. . . /. . . /. . . /0 0 /0 0 /0 0 /0 0 0 /0 0 0 /0 0 0 /0 0 0 /0 0 0 /"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows);
		const clues = [-1, -1, -1, -1, -1, 1, 2, 3, -2];
		for (let n = 4; n <= Math.min(maxnum, 12); n++) {
			clues.push(n);
		}
		const qsubs = [0, 0, 0, 0, 0, 4, 8, 12, 16, 20, 32, 48, 60];
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			cell.qsub = r.pick(qsubs);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qans = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.qans = 1;
			} else if (x < 0.3) {
				border.qsub = 1;
			} else if (x < 0.4) {
				border.qans = 1;
				border.qsub = 1;
			}
		});
	}
};

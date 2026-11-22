// Fixture inputs for pzprjs pid "tents". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tents",
	dataset: null,
	urls: [
		"tents/6/6/g1h2i1io282308",
		"tents/11/11/h4h3g2k4h3g2il16533343474343335614070414",
		"tents/6/6/rk242333520",
		"https://puzz.link/p?tents/6/6/g1h2i1io282308",
		"http://pzv.jp/p.html?type=player&tents/6/6/rk242333520",
		"tents/6/6/",
		"tents/6/6/0000000000",
		"tents/6/6/g1h2i1io282308zz",
		"tents/1/1/",
		"tents/1/1/00",
		"tents/1/6/",
		"tents/6/1/",
		"tents/4/4/44444444",
		"tents/4/4/",
		"tents/2/3/"
	],
	fileios: [
		"pzprv3\ntents\n4\n4\n. 0 0 0 0 \n0 1 A - . \n1 . - A . \n0 . . . . \n0 . . . . \n1 0 -1 \n0 1 0 \n2 0 0 \n-1 0 1 \n0 0 0 0 \n2 0 0 1 \n0 1 0 0 \n",
		// `qcmp` moves with the excell object while `adjustExCellTopLeft_2`
		// rewrites only `qnum`; pzprjs is not equivariant under the transform
		// group on boards larger than 1x1, so keep `c` on a 1x1 board.
		"pzprv3\ntents\n1\n1\n. c1 \n1 . \n\n",
		"pzprv3\ntents\n2\n3\n. 2 0 0 \n1 1 A . \n0 . - 1 \n0 1 \n0 -1 \n0 1 0 \n"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.3) {
				cell.qnum = 1;
			} else if (x < 0.45) {
				cell.anum = 2;
			} else if (x < 0.55) {
				cell.qsub = 1;
			}
		});
		p.board.excell.each((excell) => {
			// `ExCell@tents.maxnum`: top excells allow rows, left ones cols.
			const max = excell.by === -1 ? p.board.rows : p.board.cols;
			excell.qnum = r.pick([-1, -1, -1, -1, 0, 1, Math.floor(max / 2), max]);
			excell.qcmp = p.board.cols === 1 && p.board.rows === 1 && r.rnd() < 0.5 ? 1 : 0;
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.qans = 1;
			} else if (x < 0.3) {
				border.qsub = 1;
			}
		});
	}
};

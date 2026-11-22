// Fixture inputs for pzprjs pid "magnets". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "magnets",
	dataset: null,
	urls: [
		"magnets/8/8/h22h23h21h4242h22h13h31hbdrtfmvbtrd0upnpupnpupng",
		"magnets/a/8/8/h22h23h21h4242h22h13h31hbdrtfmvbtrd0upnpupnpupng",
		"https://puzz.link/p?magnets/8/8/h22h23h21h4242h22h13h31hbdrtfmvbtrd0upnpupnpupng",
		"http://pzv.jp/p.html?type=player&magnets/a/8/8/h22h23h21h4242h22h13h31hbdrtfmvbtrd0upnpupnpupng",
		"magnets/4/4/v000000fffff9",
		"magnets/4/4/v0000009000000g00",
		"magnets/a/3/3/r0000ddd",
		"magnets/4/4/",
		"magnets/4/4/v000000",
		"magnets/4/4/v",
		"magnets/4/4/v000000zz",
		"magnets/1/1/",
		"magnets/1/4/",
		"magnets/4/1/",
		"magnets/a/4/4/v000000"
	],
	fileios: [
		"pzprv3\nmagnets\n4\n4\n1 0 0 \n1 0 0 \n0 0 0 \n0 0 0 \n0 0 0 0 \n0 0 0 0 \n0 0 0 0 \n. . 2 . . . \n. . 1 3 . 4 \n. . q1 q2 # s2 \n. . a0 q1 a1 s1 \n. . . . . . \n. . . . . . \n",
		"pzprv3\nmagnets\n2\n2\na\n0 \n1 \n0 0 \n. . 2 . \n. . 1 1 \n2 . q1 q2 \n. . # s1 \n",
		// `qcmp` moves with the excell object while `adjustExCellTopLeft_2`
		// rewrites only `qnum`; keep `c` on a 1x1 board, where no excell moves.
		"pzprv3\nmagnets\n1\n1\n\n. . 1 \n. . c1 \n1 1 q2 \n"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.2) {
				cell.ques = 7;
			} else if (x < 0.5) {
				cell.qnum = r.pick([1, 2]);
			} else if (x < 0.65) {
				cell.qsub = r.pick([1, 2]);
			} else if (x < 0.75) {
				cell.anum = r.pick([0, 1]);
			}
		});
		p.board.excell.each((excell) => {
			// `ExCell@magnets.maxnum`: top clues allow (rows+1)/2, left clues
			// (cols+1)/2.
			const max =
				excell.by < 0
					? ((p.board.rows + 1) / 2) | 0
					: ((p.board.cols + 1) / 2) | 0;
			excell.qnum = r.pick([-1, -1, -1, -1, 0, 1, Math.floor(max / 2), max]);
			excell.qcmp = p.board.cols === 1 && p.board.rows === 1 && r.rnd() < 0.5 ? 1 : 0;
		});
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.3 ? 1 : 0;
		});
		p.setConfig("magnets_anti", r.rnd() < 0.3);
	}
};

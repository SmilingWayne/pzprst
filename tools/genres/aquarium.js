// Fixture inputs for pzprjs pid "aquarium". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "aquarium",
	dataset: null,
	urls: [
		"aquarium/9/9/96i5kpursvp6l50btavaik2l3l3hmo/o545545545",
		"aquarium/r/9/9/96i5kpursvp6l50btavaik2l3l3hmo/o545545545",
		"aquarium/10/10/ljan8t4r28r4n2tapl085tni1udmfg9tnk20/4g4g7h7g77g4g747h4",
		"aquarium/r/8/8/23o0oaqf3q0053b7q4q407b0/g4g4g3g4g3h5g6g",
		"aquarium/8/8/23o0oaqf3q0053b7q4q407b0/g4g4g3g4g3h5g6g",
		"https://puzz.link/p?aquarium/r/9/9/96i5kpursvp6l50btavaik2l3l3hmo/o545545545",
		"http://pzv.jp/p.html?type=player&aquarium/8/8/23o0oaqf3q0053b7q4q407b0/g4g4g3g4g3h5g6g",
		"aquarium/4/4/",
		"aquarium/r/4/4/000000/c4m",
		"aquarium/4/4/000000/4zz",
		"aquarium/4/4/000000",
		"aquarium/1/1//00",
		"aquarium/1/1/",
		"aquarium/1/4/0/k",
		"aquarium/4/1/0/k"
	],
	fileios: [
		"pzprv3\naquarium\n4\n4\nr\n0 0 0 \n0 0 0 \n0 1 0 \n0 0 0 \n0 0 0 0 \n1 0 0 0 \n0 0 0 0 \n. 4 . . . \n. # + . . \n. . . . . \n. . + . . \n. . . . . \n",
		"pzprv3\naquarium\n2\n2\n0 \n0 \n0 0 \n. 0 . \n1 # . \n. . . \n",
		// `qcmp` moves with the excell object while `adjustExCellTopLeft_2`
		// rewrites only `qnum`; pzprjs is not equivariant under the transform
		// group on boards larger than 1x1, so keep `c` on a 1x1 board.
		"pzprv3\naquarium\n1\n1\n\n. c1 \n1 . \n"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.35) {
				cell.qsub = 1;
			}
		});
		p.board.excell.each((excell) => {
			// `ExCell@aquarium.maxnum`: top excells allow rows, left ones cols.
			const max = excell.by === -1 ? p.board.rows : p.board.cols;
			excell.qnum = r.pick([-1, -1, -1, -1, 0, 1, Math.floor(max / 2), max]);
			// See the 1x1 fileio below: `qcmp` is only group-equivariant there.
			excell.qcmp = p.board.cols === 1 && p.board.rows === 1 && r.rnd() < 0.5 ? 1 : 0;
		});
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.3 ? 1 : 0;
		});
		p.setConfig("aquarium_regions", r.rnd() < 0.3);
	}
};

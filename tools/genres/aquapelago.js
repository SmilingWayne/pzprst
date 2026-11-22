// Fixture inputs for pzprjs pid "aquapelago". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "aquapelago",
	dataset: null,
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"aquapelago/6/6/r3j4r1j2",
		"aquapelago/11/11/zn7m2z5k6z5m3zn",
		"aquapelago/11/11/k2k6o3zzzzh7k3o6",
		"https://puzz.link/p?aquapelago/6/6/r3j4r1j2",
		"aquapelago/4/4/8j1g4m2",
		"aquapelago/4/4/i.r",
		"aquapelago/1/1/1",
		"aquapelago/1/1/.",
		"aquapelago/1/5/h2h",
		"aquapelago/5/1/i3g",
		"aquapelago/6/6/-12zt-12",
		"aquapelago/6/6/r3j4r1",
		"aquapelago/6/6/r3j4r1j2zz",
		"aquapelago/4/4/",
		"http://pzv.jp/p.html?type=player&aquapelago/4/4/8j1g4m2"
	],
	fileios: [
		"pzprv3\naquapelago\n2\n3\n3 # + \n- # . \n1 0 \n-1 0 \n1 0 0 \n",
		"pzprv3\naquapelago\n2\n2\n1 . \n. 2 \n"
	],
	randomize(p, r) {
		const maxnum = Math.ceil((p.board.cols * p.board.rows) / 2);
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 1; n <= maxnum; n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			cell.qans = r.rnd() < 0.15 ? 1 : 0;
			cell.qsub = r.rnd() < 0.1 ? 1 : 0;
		});
		p.board.border.each((border) => {
			border.line = r.rnd() < 0.2 ? 1 : 0;
			border.qsub = r.rnd() < 0.1 ? 2 : 0;
		});
	}
};

// Fixture inputs for pzprjs pid "mochikoro". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "mochikoro",
	dataset: null,
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"mochikoro/10/10/6m4j3zl3j3zh4k6u4n2",
		"mochikoro/10/10/q4h6k6x3zt6t44j4j",
		"mochikoro/10/10/4i3i8zi5m6v4q4s4v",
		"https://puzz.link/p?mochikoro/10/10/6m4j3zl3j3zh4k6u4n2",
		"http://pzv.jp/p.html?type=player&mochikoro/10/10/6m4j3zl3j3zh4k6u4n2",
		"mochikoro/10/10/6m4j3zl3j3zh4k6u4n2/",
		"mochikoro/10/10/6m4j3zl3j3zh4k6u4n2zz",
		"mochikoro/10/10/6m4j",
		"mochikoro/3/2/1g2g.3",
		"mochikoro/3/3/1.2",
		"mochikoro/4/4/",
		"mochikoro/1/1/1",
		"mochikoro/1/1/.",
		"mochikoro/1/1/",
		"mochikoro/1/5/12345",
		"mochikoro/5/1/12345",
		"mochikoro/4/4/+010",
		"mochikoro/f/4/4/1234",
		"mochikoro/3/3/zzz",
		"mochikoro/6/6"
	],
	fileios: [
		"pzprv3/mochikoro/2/3/1 . 2 /. - 3 /# . + /. # . /",
		"pzprv3/mochikoro/2/2/. . /. - /. + /. # /",
		"pzprv3/mochikoro/1/1/1 /+ /"
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

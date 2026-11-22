// Fixture inputs for pzprjs pid "slashpack". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "slashpack",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"slashpack/9/9/g2g2h2h1l2k2t3h3j3t3h1h1i3h1h1j",
		"slashpack/6/6/j2g1j3g2g4j3i1h4i5h5g",
		"slashpack/7/7/.n5k5h1m.g.n4g1j3g3h",
		"https://puzz.link/p?slashpack/6/6/j2g1j3g2g4j3i1h4i5h5g",
		"http://pzv.jp/p.html?type=player&slashpack/6/6/j2g1j3g2g4j3i1h4i5h5g",
		// Empty board and the largest clue (16 on a 4x4 board).
		"slashpack/4/4/",
		"slashpack/4/4/-10",
		"slashpack/2/2/11",
		"slashpack/1/1/1",
		"slashpack/1/5/12345",
		"slashpack/5/1/12345",
		"slashpack/3/3/.12",
		"slashpack/6/6/j2g1j3",
		"slashpack/4/4/12zz",
		"slashpack/z/4/4/1"
	],
	fileios: [
		// Numbers 1/2 plus the slash layer: corner bits, both slash answers.
		"pzprv3/slashpack/2/2/1 2 /. . /o1 2 /. u /",
		// The largest clue and every slash-layer token shape.
		"pzprv3/slashpack/2/3/16 . 3 /4 . 5 /o1 u . /. 2 1 /"
	],
	randomize(p, r) {
		const maxnum = p.board.rows * p.board.cols;
		const numbers = [];
		for (let n = 1; n <= Math.min(maxnum, 16); n++) {
			numbers.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.rnd() < 0.5 ? -1 : r.pick([-1, -1, -2].concat(numbers));
			cell.qans = r.pick([0, 0, 0, 31, 31, 32]);
			cell.qsub = r.pick([0, 0, 0, 1, 2, 3, 4, 5, 6, 7]);
		});
	}
};

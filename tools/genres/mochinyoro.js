// Fixture inputs for pzprjs pid "mochinyoro". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "mochinyoro",
	dataset: null,
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"mochinyoro/6/6/o3j3i3w",
		"mochinyoro/12/12/h9kazn2v1v78v4v6zn3k5h",
		"mochinyoro/17/17/hdzmenajfzh71zw4zu6i5zu3zw-108zh2jcn9zmbh",
		"https://puzz.link/p?mochinyoro/6/6/o3j3i3w",
		"http://pzv.jp/p.html?type=player&mochinyoro/6/6/o3j3i3w",
		"mochinyoro/6/6/o3j3i3w/",
		"mochinyoro/6/6/o3j3i3wzz",
		"mochinyoro/6/6/o3j3",
		"mochinyoro/3/2/1g2g.3",
		"mochinyoro/3/3/1.2",
		"mochinyoro/4/4/",
		"mochinyoro/1/1/1",
		"mochinyoro/1/1/.",
		"mochinyoro/1/1/",
		"mochinyoro/1/5/12345",
		"mochinyoro/5/1/12345",
		"mochinyoro/4/4/+010",
		"mochinyoro/f/4/4/1234",
		"mochinyoro/3/3/zzz",
		"mochinyoro/6/6"
	],
	fileios: [
		"pzprv3/mochinyoro/2/3/1 . 2 /. - 3 /# . + /. # . /",
		"pzprv3/mochinyoro/2/2/. . /. - /. + /. # /",
		"pzprv3/mochinyoro/1/1/1 /+ /"
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

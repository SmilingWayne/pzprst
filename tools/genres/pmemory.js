// Fixture inputs for pzprjs pid "pmemory". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "pmemory",
	dataset: null,
	urls: [
		"https://puzz.link/p?pmemory/3/3/0000700",
		"pmemory/3/3/0000700",
		"pmemory/2/2/00200",
		"pmemory/2/2/00020",
		"pmemory/1/1/00",
		"pmemory/2/2/00",
		"pmemory/2/2/00200zz",
		"pmemory/2/2/0000",
		"pmemory/2/2/0020z",
		"pmemory/3/3/0000700zz",
		"pmemory/3/3/0",
		"pmemory/3/3/hh",
		"pmemory/3/2/00000",
		"pmemory/2/3/00000",
		"pmemory/1/4/0",
		"pmemory/4/1/0",
		"pmemory/2/3/000030",
		"pmemory/3/2/000030"
	],
	fileios: [
		"pzprv3/pmemory/2/2/1 /0 /0 1 /3 0 /0 2 /1 /-1 /0 1 /0 1 /2 0 /",
		"pzprv3/pmemory/2/3/1 0 /0 1 /0 0 1 /3 0 0 /0 3 2 /1 /-1 /0 1 0 /0 1 2 /2 0 0 /",
		"pzprv3/pmemory/1/1//2 //1 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 0, 0, 6]);
			cell.qnum = r.pick([-1, -1, -1, -1, 1]);
			cell.qsub = r.pick([0, 0, 0, 0, 1, 2]);
		});
		p.board.border.each((border) => {
			border.ques = r.pick([0, 0, 0, 0, 1]);
			if (r.rnd() < 0.2) {
				border.line = 1;
			} else if (r.rnd() < 0.15) {
				border.qsub = 2;
			}
		});
	}
};

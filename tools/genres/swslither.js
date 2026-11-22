// Fixture inputs for pzprjs pid "swslither". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "swslither",
	dataset: null,
	urls: [
		"https://puzz.link/p?swslither/6/6/16b3b2a5a63b2a65a3b15a6a2b1b53",
		"swslither/6/6/16b3b2a5a63b2a65a3b15a6a2b1b53",
		"https://puzz.link/p?swslither/9/9/b2c1b2651a36535a16a52a66b5a6b535a6a5a62a3a516a3b2b5b1c1a6a3b0c5c2",
		"https://puzz.link/p?swslither/10/10/a12c31b2b36c321d21d21d55a1b62d33d02b2a60d35d22d331c05b2b63c13a",
		"http://pzv.jp/p.html?type=player&swslither/6/6/16b3b2a5a63b2a65a3b15a6a2b1b53",
		"swslither/6/6/16b3b2a5a63b2a65a3b15a6a2b1b53/",
		"swslither/6/6/16b3b2a5a63b2a65a3b15a6a2b1b53zz",
		"swslither/6/6/16b3",
		"swslither/4/4/",
		"swslither/4/4/6666666666666666",
		"swslither/4/4/....",
		"swslither/4/4/0123456012345601",
		"swslither/3/3/789",
		"swslither/3/3/A1",
		"swslither/f/4/4/0123456012345601",
		"swslither/x/4/4/0123456012345601",
		"swslither/fx/4/4/0123456012345601",
		"swslither/1/1/6",
		"swslither/1/1/",
		"swslither/1/8/12345678",
		"swslither/8/1/12345678"
	],
	fileios: [
		"pzprv3.1/swslither/2/2/f/1 2 /- . /0 1 /2 0 /1 0 0 /0 -1 0 /0 1 /1 0 /-1 0 /",
		"pzprv3/swslither/2/3/1 2 3 /4 5 6 /0 0 1 0 /-1 0 0 1 /0 0 0 /1 0 -1 /0 0 1 /",
		"pzprv3.1/swslither/1/1/6 /0 /0 0 /0 /0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, -2]);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.line = 1;
			} else if (x < 0.35) {
				border.qsub = 2;
			}
		});
		p.setConfig("slither_full", r.rnd() < 0.3);
	}
};

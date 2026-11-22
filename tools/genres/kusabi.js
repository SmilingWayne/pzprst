// Fixture inputs for pzprjs pid "kusabi". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kusabi",
	dataset: null,
	urls: [
		"kusabi/9/9/..j2d.3i.a.a3a2c1a1b2.a2d3a.a2l2.f.d",
		"https://puzz.link/p?kusabi/9/9/d.l.l.a.e1b2e.b.f3c.d2g.a.b1a",
		"https://puzz.link/p?kusabi/9/9/.e.f1a.1..c2b2e1n2a1c.1b2b.f31a.a.i",
		"kusabi/3/3/123456789",
		"kusabi/3/3/.....",
		"kusabi/2/1/0",
		"kusabi/2/1/9",
		"kusabi/2/1/3",
		"kusabi/1/1/.",
		"kusabi/1/1/",
		"kusabi/2/3/1",
		"kusabi/3/3/123456789zz",
		"kusabi/3/3/12A",
		"kusabi/3/3/z1",
		"kusabi/9/3/z1",
		"kusabi/3/3/123456789/",
		"kusabi/x/3/3/123456789",
		"kusabi/1/2/12",
		"kusabi/4/4/1b2b3",
		"kusabi/4/4/3.2.1."
	],
	fileios: [
		"pzprv3/kusabi/2/2/1 . /2 3 /0 /1 /1 0 /",
		"pzprv3/kusabi/2/2/- . /. 2 /-1 /1 /0 -1 /",
		"pzprv3/kusabi/3/2/1 2 3 /- . 1 /0 0 /1 0 /-1 1 0 /",
		"pzprv3/kusabi/1/1/3 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 1, 2, 3, -2]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

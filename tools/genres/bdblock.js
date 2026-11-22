// Fixture inputs for pzprjs pid "bdblock". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "bdblock",
	dataset: null,
	urls: [
		"bdblock/4/4",
		"bdblock/1/1",
		"bdblock/1/4",
		"bdblock/4/1",
		"bdblock/1/1/0/v",
		"bdblock/1/1/0000",
		"bdblock/3/3/9/.",
		"bdblock/3/3/./9",
		"bdblock/4/4/0o",
		"bdblock/4/4/0o/vzz",
		"bdblock/4/4/p/.",
		"bdblock/2/2/p/+2e7i",
		"bdblock/2/2/p/-3",
		"bdblock/4/4//v",
		"bdblock/6/5/3.2/zp",
		"bdblock/2/3/03",
		"bdblock/3/2/0",
		"https://puzz.link/p?bdblock/10/10/84c2i35ejn3/q1h2s3h4g5g2g5x6o1g3i6q4i7m4i",
		"https://puzz.link/p?bdblock/10/10/524p427d6hq/q1i2m3i3i4i4m5j2h6j2m5i3i3i6m1i1q",
		"https://puzz.link/p?bdblock/10/10/32g1d4c2hub/q1h2v3h4p5h1h6h4p3h5v2h6q"
	],
	fileios: [
		"pzprv3/bdblock/4/4/. . . . /. . . . /. . . . /. . . . /. . 1 . . /. . . . 1 /. . . . . /. 1 . - . /. . 5 1 . /1 -1 0 /0 1 2 /0 -1 1 /0 0 0 /1 -1 0 0 /1 0 0 -1 /1 0 0 0 /",
		"pzprv3/bdblock/3/2/3 . /. - /. 12 /1 . . /. 1 - /. . . /. . . /1 /-1 /0 /2 0 /0 0 /",
		"pzprv3/bdblock/1/1/. /1 . /. . /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, -1, 1, 2, 3, -2, 12, 255, 999]);
		});
		p.board.cross.each((cross) => {
			cross.qnum = r.pick([-1, -1, -1, -1, -1, 1, 1, -2, 5, 42]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.qans = 1;
			} else if (x > 0.8) {
				border.qsub = 1;
			} else if (x > 0.45 && x < 0.55) {
				border.qans = 1;
				border.qsub = 1;
			}
		});
	}
};

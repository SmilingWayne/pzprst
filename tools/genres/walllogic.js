// Fixture inputs for pzprjs pid "walllogic". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "walllogic",
	dataset: null,
	urls: [
		"walllogic/4/4",
		"walllogic/4/4/",
		"walllogic/4/4//",
		"https://puzz.link/p?walllogic/4/4",
		"https://puzz.link/p?walllogic_edit/4/4/0123",
		"walllogic/x/4/4/0123",
		"walllogic/4/4/0",
		"walllogic/4/4/0123456",
		"walllogic/4/4/0123zz",
		"walllogic/4/4/.",
		"walllogic/4/4/-0f",
		"walllogic/4/4/-10",
		"walllogic/4/4/+",
		"walllogic/4/4/0123456789abcdef",
		"walllogic/4/4/g",
		"walllogic/4/4/0123456789abcdefg",
		"walllogic/1/1/6",
		"walllogic/1/5/01234",
		"walllogic/5/1/01234",
		"walllogic/25/1/00000",
		"walllogic/1/25/00000",
		"https://puzz.link/p?walllogic/6/6/g3i33h3j3n3h3g3j3h",
		"https://puzz.link/p?walllogic/6/6/g2i14i1i2l3i3i51i4g",
		"https://puzz.link/p?walllogic/9/9/j5j4i3i3g2h2h4i3i2l3l4i4i3h3h3g3i3i4j4j"
	],
	fileios: [
		"pzprv3/walllogic/3/2/# 0 1 /2 . - /1 2 . /- 3 4 /",
		"pzprv3/walllogic/4/4/# 0 1 . /2 . - 3 /. . 4 . /5 . . . /1 . 2 . /- . 3 4 /. 1 . 2 /. 3 . 4 /",
		"pzprv3/walllogic/1/1/# /- /",
		"pzprv3/walllogic/1/2/. 3 /2 . /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.12) {
				cell.ques = 1;
			} else if (x < 0.25) {
				cell.qnum = -2;
			} else if (x < 0.6) {
				cell.qnum = Math.floor(r.rnd() * (p.board.cols + p.board.rows - 1));
			}
			const y = r.rnd();
			if (y < 0.25) {
				cell.anum = r.pick([1, 2, 3, 4]);
			} else if (y < 0.35) {
				cell.qcmp = 1;
			}
		});
	}
};

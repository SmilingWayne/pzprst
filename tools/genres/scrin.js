// Fixture inputs for pzprjs pid "scrin". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "scrin",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"scrin/10/10/g3l6zn3m1n2m4zn4l4g",
		"scrin/8/8/i4j4k3q2p2q4k3j4i",
		"scrin/12/12/o5t8l3x5h.s3n.s.h3x1l8t8o",
		"https://puzz.link/p?scrin/10/10/g3l6zn3m1n2m4zn4l4g",
		"https://puzz.link/p?scrin_edit/8/8/i4j4k3q2p2q4k3j4i",
		"http://pzv.jp/p.html?type=player&scrin/12/12/o5t8l3x5h.s3n.s.h3x1l8t8o",
		// Every token class: `?`, one hex digit, both two- and three-digit
		// escapes, and empty runs.
		"scrin/4/4/",
		"scrin/3/3/1.2g3",
		"scrin/2/2/.",
		"scrin/2/2/-ff",
		"scrin/4/4/+fff",
		"scrin/4/4/=10",
		"scrin/4/4/g",
		"scrin/1/1/1",
		"scrin/1/5/1g",
		"scrin/5/1/1g",
		// Truncated body and trailing junk.
		"scrin/3/3/1",
		"scrin/3/3/1zz",
		"scrin/3/3"
	],
	fileios: [
		// Question layer plus an answer (shading/background) layer.
		"pzprv3/scrin/3/3/1 - 2 /. 3 . /. . . /. . . /. . . /. + . /",
		// `?` on a 2x2 board, `#` and `+` answer marks.
		"pzprv3/scrin/2/2/- . /. . /# + /. # /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const maxnum = cell.getmaxnum();
			cell.qnum = -1;
			cell.qans = 0;
			cell.qsub = 0;
			const x = r.rnd();
			if (x < 0.3) {
				cell.qnum = -2;
			} else if (x < 0.6 && maxnum >= 1) {
				cell.qnum = 1 + Math.floor(r.rnd() * maxnum);
			} else if (x < 0.8) {
				cell.qans = 1;
			} else if (x < 0.9) {
				cell.qsub = 1;
			}
		});
	}
};

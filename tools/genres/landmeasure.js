// Fixture inputs for pzprjs pid "landmeasure". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "landmeasure",
	dataset: null,
	urls: [
		// pzprjs's own test URL (also its extended-variety state fixture).
		"landmeasure/6/6/n7h.g2u9i3o0h2h",
		"https://puzz.link/p?landmeasure/6/6/n7h.g2u9i3o0h2h",
		// The URL built from the 6x6 FileIO below: the interior cross 35 is the
		// `-23` escape form of a number above 15.
		"landmeasure/6/6/n-23h.g2u9i3o0h2h",
		// An empty cross grid, a single 0, trailing junk and an unknown flag.
		"landmeasure/3/3/v",
		"landmeasure/3/3/0",
		"landmeasure/3/3/vzz",
		"landmeasure/x/3/3/v",
		// Empty bodies.
		"landmeasure/3/3",
		"landmeasure/3/3/",
		// 1x1, 1xN and Nx1 boards.
		"landmeasure/1/1/0",
		"landmeasure/1/1/1i",
		"landmeasure/1/1/",
		"landmeasure/1/2/200",
		"landmeasure/2/1/200",
		"landmeasure/2/2/00003j",
		// A truncated body on a wider board (the interior maximum 15).
		"landmeasure/4/4/lfx"
	],
	fileios: [
		// pzprjs's own 6x6 answer fixture (shaded cells) with cross 7, 9, 3, 2, 0
		// and an infinity (`-`).
		"pzprv3/landmeasure/6/6/. . . . . . . /. 7 . . - . 2 /. . . . . . . /. . . . . . . /. 9 . . . 3 . /. . . . . . . /. 0 . . 2 . . /. # # # . # /# . # . # # /# # # # . . /# . . # . # /. # # # # # /. . . # # . /",
		// pzprjs's own 3x4 fixture: cross numbers up to the interior maximum 11.
		"pzprv3/landmeasure/3/4/. . . . . /. 9 9 11 . /. 3 7 7 . /. . . . . /. . . . /. . . . /. . . . /",
		// A number above 15 (`35`, the 6x6 interior maximum), two infinities and
		// both cell answer tokens.
		"pzprv3/landmeasure/6/6/. . . . . . . /. 35 . . - . 2 /. . . . . . . /. . . . . . . /. 9 . . . 3 . /. . . . . . . /. 0 . . 2 . . /# . . . . . /. . . . . . /. . + . . . /. . . . . . /. . . . . . /. . . . . # /",
		// A 3x3 board with the corner/edge maxima 1 and 2 and a cell mark.
		"pzprv3/landmeasure/3/3/1 . 2 . /. 0 . 3 /2 . 5 . /. . . . /# . + /. . . /# # . /"
	],
	randomize(p, r) {
		p.board.cross.each((cross) => {
			const max = cross.getmaxnum();
			const x = r.rnd();
			if (x < 0.55) {
				cross.qnum = -1;
			} else if (max >= 3 && x < 0.65) {
				// `-2` is the infinity sign; pzprjs only enters it on crosses
				// whose maximum allows the longer cycle.
				cross.qnum = -2;
			} else {
				cross.qnum = r.pick([0, 1, 2, max]);
			}
		});
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qans = x < 0.3 ? 1 : 0;
			cell.qsub = x >= 0.3 && x < 0.45 ? 1 : 0;
		});
	}
};

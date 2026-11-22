// Fixture inputs for pzprjs pid "araf". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "araf",
	dataset: "Araf",
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"araf/9/9/44g644g14h9i9h33h4h61h1i9h9h395h1g4k1g6g4i7g1g2g9g1g4i9g5g9h",
		"araf/7/7/n437bah5i6h6i8h2i4h34456n",
		"araf/8/8/h65k678ak51zjf5k871-18ka-19h",
		"https://puzz.link/p?araf/7/7/n437bah5i6h6i8h2i4h34456n",
		"http://pzv.jp/p.html?type=player&araf/7/7/n437bah5i6h6i8h2i4h34456n",
		"araf/7/7/n437bah5i6h6i8h2i4h34456n/",
		"araf/4/4/1zz",
		"araf/7/7/n437bah5i6h6i8h",
		"araf/4/4/.",
		"araf/4/4/+3e7",
		"araf/2/2/-ff",
		"araf/1/1/0",
		"araf/1/5/1",
		"araf/5/1/-19",
		"araf/4/4",
		"araf/3/3/z1",
		"araf/4/4/1G",
		"araf/x/4/4/1",
		"araf/f/4/4/1"
	],
	fileios: [
		// Real 7x7 puzzle with answer borders and crosses.
		"pzprv3/araf/7/7/. . . . . . . /. 4 3 7 11 10 . /. 5 . . . 6 . /. 6 . . . 8 . /. 2 . . . 4 . /. 3 4 4 5 6 . /. . . . . . . /0 1 0 2 0 -1 /0 0 0 0 0 0 /2 0 0 0 1 0 /0 0 -1 0 0 0 /0 0 0 0 0 1 /0 2 0 0 0 0 /0 0 0 0 0 0 /0 0 1 0 0 2 0 /0 0 0 0 0 0 0 /1 0 0 0 0 -1 0 /0 0 0 2 0 0 0 /0 0 0 0 0 0 0 /0 -1 0 0 0 0 0 /",
		// The clue 0, a question mark, the largest clue and every answer token.
		"pzprv3/araf/2/3/0 - 999 /16 1 . /2 1 /-1 0 /1 2 -1 /",
		// Empty board: no clue and no answer data.
		"pzprv3/araf/3/3/. . . /. . . /. . . /0 0 /0 0 /0 0 /0 0 0 /0 0 0 /"
	],
	randomize(p, r) {
		const clues = [-1, -1, -1, -1, -1, 0, 1, 2, 3, 15, 16, 255, 256, 999, -2];
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qans = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.qans = 1;
			} else if (x < 0.3) {
				border.qsub = 1;
			} else if (x < 0.4) {
				border.qans = 1;
				border.qsub = 1;
			}
		});
	}
};

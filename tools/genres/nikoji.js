// Fixture inputs for pzprjs pid "nikoji". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "nikoji",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"nikoji/8/8/o75d9e9i1h-14j-14h1jfh-12j-10h-14i93953-14o",
		"nikoji/8/8/1k2h2i3j1g3l-18l3g2j1i1h2k1o",
		"nikoji/6/7/-14bp-10-15-1a-1ac5h3c-152n-14o",
		"https://puzz.link/p?nikoji/6/7/-14bp-10-15-1a-1ac5h3c-152n-14o",
		"http://pzv.jp/p.html?type=player&nikoji/6/7/-14bp-10-15-1a-1ac5h3c-152n-14o",
		"nikoji/6/7/-14bp-10-15-1a-1ac5h3c-152n-14o/",
		"nikoji/4/4/1zz",
		"nikoji/6/7/-14bp-10-15-1a-1a",
		"nikoji/4/4/.",
		"nikoji/4/4/-34",
		"nikoji/2/2/-ff",
		"nikoji/1/1/1",
		"nikoji/1/5/1",
		"nikoji/5/1/-19",
		"nikoji/4/4",
		"nikoji/3/3/z1",
		"nikoji/4/4/1G",
		"nikoji/x/4/4/1",
		"nikoji/f/4/4/1"
	],
	fileios: [
		// Real 6x7 puzzle with answer borders and crosses.
		"pzprv3/nikoji/7/6/20 11 . . . . /. . . . . . /16 21 26 26 12 5 /. . 3 12 21 2 /. . . . . . /. . 20 . . . /. . . . . . /0 0 2 0 0 /1 0 0 -1 0 /0 0 0 0 2 /0 1 0 0 0 /-1 0 0 1 0 /0 0 0 0 0 /2 0 0 0 -1 /0 0 0 1 0 0 /0 2 0 0 0 -1 /0 0 0 0 0 0 /1 0 0 2 0 0 /0 0 -1 0 0 1 /0 0 0 0 0 0 /",
		// Largest clue, a question mark, two-digit clues and every answer token.
		"pzprv3/nikoji/2/3/. - 52 /16 2 . /2 1 /-1 0 /1 2 -1 /",
		// Empty board: no clue and no answer data.
		"pzprv3/nikoji/3/3/. . . /. . . /. . . /0 0 /0 0 /0 0 /0 0 0 /0 0 0 /"
	],
	randomize(p, r) {
		const clues = [-1, -1, -1, -1, -1, 1, 2, 3, 10, 16, 31, 52, -2];
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

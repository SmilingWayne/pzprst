// Fixture inputs for pzprjs pid "nagare". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "nagare",
	dataset: null,
	urls: [
		// Real puzz.link boards.
		"https://puzz.link/p?nagare/15/15/p9d559d5c5d5a3b5d8b3a5d9d5d9d5d8c15d9e5c5a4a7d3b6555557b2h9h1c4a5n82d3b4a2a4a5n8h1a4b559u",
		"https://puzz.link/p?nagare/13/13/55k5f4a55b5k6c9d77e59b6d7f4i8d4d5f7b8b5a5a66d8e1q8b52c9b5g6h9b6",
		"https://puzz.link/p?nagare/10/10/k5a5a5l6c8a6a4a52b5a8g8a521a4i5a16b8a4d52b1a9f6e",
		"nagare_edit/10/10/k5a5a5l6c8a6a4a52b5a8g8a521a4i5a16b8a4d52b1a9f6e",
		"http://pzv.jp/p.html?type=player&nagare/10/10/k5a5a5l6c8a6a4a52b5a8g8a521a4i5a16b8a4d52b1a9f6e",
		// Unknown flag and trailing junk.
		"nagare/x/10/10/k5a5a5l6c8a6a4a52b5a8g8a521a4i5a16b8a4d52b1a9f6e",
		"nagare/10/10/k5a5a5l6c8a6a4a52b5a8g8a521a4i5a16b8a4d52b1a9f6e/zz",
		// Every clue value 1-9 on a pair of 2x2 boards.
		"nagare/2/2/1",
		"nagare/2/2/6",
		"nagare/2/2/9",
		"nagare/2/2/1234",
		"nagare/2/2/6789",
		// Characters outside 1-9 and a-z consume one empty cell.
		"nagare/2/2/.",
		"nagare/2/2/0",
		"nagare/2/2/#",
		// Base-36 run lengths, 1/N and N/1 boards.
		"nagare/2/2/a",
		"nagare/5/5/z",
		"nagare/1/5/z",
		"nagare/5/1/z",
		// Truncated body, empty body and 1/1.
		"nagare/4/4/1",
		"nagare/4/4",
		"nagare/4/4/",
		"nagare/1/1",
		"nagare/1/1/"
	],
	fileios: [
		// 2x2: arrows, walls with an arrow, a plain wall and border lines.
		"pzprv3/nagare/2/2/u R /N . /1 /0 /0 -2 /",
		// 3x2: every cell token plus border aux marks (`u`, `3u`) and a cross.
		"pzprv3/nagare/2/3/l d r /N U . /u 3u /0 0 /1 -2 0 /",
		// 1x1: a wall cell, then the empty inner-border section.
		"pzprv3/nagare/1/1/N /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 1]);
			cell.qdir = r.pick([0, 0, 1, 2, 3, 4]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			if (x < 0.15) {
				border.line = r.pick([1, 2, 3]);
			}
			const y = r.rnd();
			if (y < 0.12) {
				border.qsub = 2;
			} else if (y < 0.4) {
				border.qsub = r.pick([11, 12, 13, 14]);
			}
		});
	}
};

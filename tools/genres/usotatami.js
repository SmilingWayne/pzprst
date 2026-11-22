// Fixture inputs for pzprjs pid "usotatami". See tools/gen_fixtures.js for the format.
// There is no Uso-tatami folder under ../puzzlekit-dataset/assets/data, so the
// three real puzzles below come from the pzprst sample list (pzprjs-canonical).
"use strict";

module.exports = {
	pid: "usotatami",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"usotatami/6/6/-+3e7+3e7+3e7+3e7+3e7+3e7zp",
		"usotatami/8/8/44a4d4a444e4b4b44a4b44a4d4a4f4e4e44c",
		"usotatami/8/8/7b23b6b4b2f2d4a21a2b4b3a3e8e5b3b2b32b3",
		"https://puzz.link/p?usotatami/6/6/-+3e7+3e7+3e7+3e7+3e7+3e7zp",
		"http://pzv.jp/p.html?type=player&usotatami/3/3/123",
		// Base 10 (no leading '-') and base 16 after the leading '-'.
		"usotatami/3/3/123",
		"usotatami/3/3/12.3",
		"usotatami/2/2/-0f",
		"usotatami/2/2/-+3e7+3e7",
		"usotatami/2/2/-=7ff",
		"usotatami/2/2/-@000",
		"usotatami/2/2/-*0000",
		"usotatami/2/2/-$00000",
		// Unexpected character, trailing junk, truncated body.
		"usotatami/2/2/A1",
		"usotatami/3/3/123zz",
		"usotatami/4/4/1",
		// Smallest and lopsided boards, empty bodies.
		"usotatami/1/1/0",
		"usotatami/1/4/1234",
		"usotatami/4/1/1234",
		"usotatami/4/4",
		"usotatami/1/1/",
		// An unknown pflag is dropped, and 'x' falls back to the default size.
		"usotatami/f/3/3/123",
		"usotatami/x/4/4/1"
	],
	fileios: [
		// The real 8x8 puzzle with answer border lines and crosses.
		"pzprv3/usotatami/8/8/7 . . 2 3 . . 6 /. . 4 . . 2 . . /. . . . 2 . . . /. 4 . 2 1 . 2 . /. 4 . . 3 . 3 . /. . . . 8 . . . /. . 5 . . 3 . . /2 . . 3 2 . . 3 /0 1 0 0 0 0 0 /0 0 2 0 0 0 0 /0 0 0 0 0 -1 0 /0 0 0 0 0 0 1 /1 0 0 0 0 0 0 /0 0 0 0 2 0 0 /0 0 0 0 0 0 0 /0 0 0 0 0 0 1 /0 0 1 0 0 0 0 0 /0 0 0 0 0 0 0 0 /1 0 0 2 0 0 0 0 /0 0 0 0 0 0 0 0 /0 0 1 0 0 0 0 0 /0 0 0 0 0 0 0 0 /0 0 0 0 0 0 0 1 /",
		// The largest clue, a question mark, the smallest clue and every answer token.
		"pzprv3/usotatami/3/3/999 0 1 /2 - . /. . . /1 0 /0 2 /0 0 /0 -1 0 /1 0 0 /",
		// 1x1 has no inner borders, so only the cell line and one empty border line.
		"pzprv3/usotatami/1/1/7 //"
	],
	randomize(p, r) {
		const clues = [-1, -1, -1, -1, -1, 1, 2, 3, 15, 16, 255, 256, 999, -2];
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

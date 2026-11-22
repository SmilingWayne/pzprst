// Fixture inputs for pzprjs pid "myopia". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "myopia",
	dataset: null,
	urls: [
		// Real puzz.link boards.
		"https://puzz.link/p?myopia/8/8/g5oai9g5iakaia9i9k5i6gai6o9g",
		"https://puzz.link/p?myopia/9/9/gbj66j55sejfhbi5nci55g3gagar9g9j3g",
		"https://puzz.link/p?myopia/10/10/j6iag9ketbk9j9g9j7kfjak9l6k5g7l5k3k",
		"myopia_edit/8/8/g5oai9g5iakaia9i9k5i6gai6o9g",
		"http://pzv.jp/p.html?type=player&myopia/8/8/g5oai9g5iakaia9i9k5i6gai6o9g",
		// The `f` flag, an unknown flag and trailing junk.
		"myopia/f/8/8/g5oai9g5iakaia9i9k5i6gai6o9g",
		"myopia/x/8/8/g5oai9g5iakaia9i9k5i6gai6o9g",
		"myopia/f/8/8/g5oai9g5iakaia9i9k5i6gai6o9g/zz",
		// A `0` clue, `?` and the largest clue 15.
		"myopia/2/2/0i",
		"myopia/2/2/.",
		"myopia/2/2/f.",
		"myopia/2/2/1f45",
		// Hex escapes (values above maxnum still round-trip through the URL).
		"myopia/2/2/-ff.i",
		// Base-36 run lengths, 1/N and N/1 boards.
		"myopia/5/5/z",
		"myopia/4/4/zn",
		"myopia/1/5/z",
		"myopia/5/1/z",
		// Truncated body and empty body.
		"myopia/4/4/1",
		"myopia/4/4",
		"myopia/4/4/"
	],
	fileios: [
		// 2x2: flag line, clues (including 0 and `?`), background marks and border lines.
		"pzprv3/myopia/2/2/f/1 15 /. - /4 0 /0 1 /0 1 0 /-1 0 0 /0 0 /0 0 /0 0 /",
		// 3x2: a 0 clue, `?`, 15, background marks and border lines.
		"pzprv3/myopia/2/3/0 - 15 /. 1 . /4 0 1 /0 0 0 /0 0 0 1 /-1 0 0 0 /0 0 0 /0 0 0 /0 0 0 /",
		// 1x1: a single 0 clue.
		"pzprv3/myopia/1/1/0 /0 /0 0 0 /0 /0 /0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([
				-1, -1, -1, -2, 0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 12, 15
			]);
			cell.qsub = r.pick([0, 0, 0, 1, 2, 3, 4]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.line = r.pick([1, 2]);
			} else if (x < 0.3) {
				border.qsub = 2;
			}
		});
		p.setConfig("slither_full", r.rnd() < 0.3);
	}
};

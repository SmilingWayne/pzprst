// Fixture inputs for pzprjs pid "sashigane". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "sashigane",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"sashigane/10/10/n5ojq5rjlhmjp5k5x5w5l5lgpjpingl",
		"sashigane/10/10/qin54qiy5.mjlimigmjljm43yjq5.ngq",
		"sashigane/10/10/o5lhmjqj4nhk4m5zl5ngrimin.kjm5zkjk",
		"https://puzz.link/p?sashigane/10/10/n5ojq5rjlhmjp5k5x5w5l5lgpjpingl",
		"http://pzv.jp/p.html?type=player&sashigane/10/10/n5ojq5rjlhmjp5k5x5w5l5lgpjpingl",
		// Every token class: arrows, circles, the two-digit clue and empty runs.
		"sashigane/4/4/",
		"sashigane/2/2/g",
		"sashigane/2/2/j",
		"sashigane/2/2/%",
		"sashigane/2/2/-ff",
		"sashigane/2/2/f",
		"sashigane/2/2/k",
		"sashigane/2/2/-10",
		"sashigane/2/2/gzz",
		"sashigane/6/6/n5ojq",
		"sashigane/1/1/g",
		"sashigane/1/5/gijgk",
		"sashigane/5/1/gijgk",
		"sashigane/f/4/4/g"
	],
	fileios: [
		// Numbered and unnumbered circles, both background marks, answer borders.
		"pzprv3/sashigane/2/2/o5 5 /_ # /1 /-1 /0 2 /",
		// All four arrows, `?`, and every border answer token.
		"pzprv3/sashigane/3/3/1 o o /4 - . /. . . /1 0 /0 -1 /2 0 /0 1 0 /0 0 1 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qdir = 0;
			cell.qnum = -1;
			cell.qans = 0;
			cell.qsub = 0;
			const kind = r.rnd();
			if (kind < 0.2) {
				cell.qdir = r.pick([1, 2, 3, 4]);
			} else if (kind < 0.35) {
				cell.qdir = 5;
				const maxnum = cell.getmaxnum();
				const clues = [3, 4, 5, 9, 10, 15, 16].filter((n) => n <= maxnum);
				if (clues.length && r.rnd() < 0.6) {
					cell.qnum = r.pick(clues);
				}
			} else if (kind < 0.45) {
				cell.qdir = -2;
			} else {
				if (r.rnd() < 0.25) {
					cell.qans = 1;
				} else if (r.rnd() < 0.25) {
					cell.qsub = 1;
				}
			}
		});
		p.board.border.each((border) => {
			border.qans = r.rnd() < 0.2 ? 1 : 0;
			border.qsub = r.rnd() < 0.2 ? 1 : 0;
		});
	}
};

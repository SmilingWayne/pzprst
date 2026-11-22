// Fixture inputs for pzprjs pid "sashikabe". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "sashikabe",
	dataset: null,
	urls: [
		// Sashikabe shares `Encode@sashigane,sashikabe`, so these canonical bodies
		// (the pzprst sample list has no sashikabe URLs) exercise every token of
		// the same codec: real sashigane puzzles, then hand-written edge cases.
		"sashikabe/10/10/n5ojq5rjlhmjp5k5x5w5l5lgpjpingl",
		"sashikabe/10/10/qin54qiy5.mjlimigmjljm43yjq5.ngq",
		"sashikabe/10/10/o5lhmjqj4nhk4m5zl5ngrimin.kjm5zkjk",
		"https://puzz.link/p?sashikabe/10/10/n5ojq5rjlhmjp5k5x5w5l5lgpjpingl",
		"http://pzv.jp/p.html?type=player&sashikabe/10/10/n5ojq5rjlhmjp5k5x5w5l5lgpjpingl",
		// Every token class: arrows, circles, the two-digit clue and empty runs.
		"sashikabe/4/4/",
		"sashikabe/2/2/g",
		"sashikabe/2/2/j",
		"sashikabe/2/2/%",
		"sashikabe/2/2/-ff",
		"sashikabe/2/2/f",
		"sashikabe/2/2/k",
		"sashikabe/2/2/-10",
		"sashikabe/2/2/gzz",
		"sashikabe/6/6/n5ojq",
		"sashikabe/1/1/g",
		"sashikabe/1/5/gijgk",
		"sashikabe/5/1/gijgk",
		"sashikabe/f/4/4/g"
	],
	fileios: [
		// Numbered and unnumbered circles and both background marks; no border layer.
		"pzprv3/sashikabe/2/2/o5 5 /_ # /",
		// All four arrows, `?` and answer marks.
		"pzprv3/sashikabe/3/3/1 o o /4 - . /. . . /"
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
			} else if (r.rnd() < 0.25) {
				cell.qans = 1;
			} else if (r.rnd() < 0.25) {
				cell.qsub = 1;
			}
		});
	}
};

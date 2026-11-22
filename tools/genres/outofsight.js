// Fixture inputs for pzprjs pid "outofsight". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "outofsight",
	dataset: null,
	urls: [
		// Everything in the 2x2 URL vocabulary: arrows, `?`, `+` and clues 0-6.
		"outofsight/2/2/10c",
		"outofsight/2/2/+c",
		"outofsight/2/2/1.c",
		"outofsight/2/2/16c",
		"outofsight/3/2/10263.45+a",
		"outofsight/2/2/0123b",
		// Two-digit and three-digit hex escapes.
		"outofsight/2/2/5a0c",
		"outofsight/2/2/6ffc",
		"outofsight/2/2/-0abcc",
		"outofsight/2/2/-1fffc",
		// URL forms: edit, player, unknown flag and trailing junk.
		"outofsight_edit/2/2/10c",
		"http://pzv.jp/p.html?type=player&outofsight/2/2/10c",
		"outofsight/x/2/2/10c",
		"outofsight/2/2/10c/zz",
		// Base-36 run lengths, 1/1, 1/N and N/1 boards.
		"outofsight/5/5/z",
		"outofsight/1/1",
		"outofsight/1/1/",
		"outofsight/1/5/z",
		"outofsight/5/1/z",
		// Truncated body and empty body.
		"outofsight/4/4/16",
		"outofsight/4/4",
		"outofsight/4/4/"
	],
	fileios: [
		// 2x2: `dir,qnum`, `+`, answer shade/background marks and border lines.
		"pzprv3/outofsight/2/2/1,0 # /2,- . /+ . /# . /1 /0 /0 1 /",
		// 3x2: every question form and answer marks, with no border line layer.
		"pzprv3/outofsight/2/3/1,0 2,6 3,- /4,5 # . /+ # . /. + . /",
		// 1x1: one clue and an answer mark, no border layer.
		"pzprv3/outofsight/1/1/1,6 /# /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.08) {
				cell.qnum = -3;
				cell.qdir = 0;
			} else if (x < 0.16) {
				cell.qnum = -2;
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			} else if (x < 0.7) {
				cell.qnum = r.pick([-1, -1, 0, 1, 2, 3, 4, 5, 6]);
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			}
			cell.qans = r.pick([0, 0, 0, 1]);
			cell.qsub = r.pick([0, 0, 0, 1]);
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
	}
};

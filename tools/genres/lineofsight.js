// Fixture inputs for pzprjs pid "lineofsight". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "lineofsight",
	dataset: null,
	urls: [
		// Hand-built 3x3: every direction 1-3 and clues 0-3.
		"lineofsight/3/3/101112132021222330",
		"https://puzz.link/p?lineofsight/3/3/101112132021222330",
		"https://puzz.link/p?lineofsight_edit/3/3/101112132021222330",
		"http://pzv.jp/p.html?type=player&lineofsight/3/3/101112132021222330",
		// Rule flag alone, with an unknown flag, and on a 1x1 board.
		"lineofsight/f/3/3/101112132021222330",
		"lineofsight/xf/3/3/101112132021222330",
		"lineofsight/f/1/1/10",
		// `+` (arrow without number) and `?` (`.`), both 1x1.
		"lineofsight/1/1/+",
		"lineofsight/1/1/0.",
		// Largest allowed clue on a 4x1 board (max(cols, rows) = 4).
		"lineofsight/4/1/10111213",
		// Base-36 run lengths, 1/N and N/1 boards.
		"lineofsight/1/5/g",
		"lineofsight/5/1/g",
		"lineofsight/4/4/v",
		"lineofsight/2/2/1414",
		// Three-character escape and a two-digit number.
		"lineofsight/1/1/-14",
		"lineofsight/1/1/5a",
		// Truncated body, trailing junk and empty bodies.
		"lineofsight/3/3/1011",
		"lineofsight/3/3/101112132021222330zz",
		"lineofsight/3/3",
		"lineofsight/3/3/",
		"lineofsight/4/4/."
	],
	fileios: [
		// 3x3 with clue numbers, the config flag and vertical/horizontal lines.
		"pzprv3/lineofsight/3/3/f/1,0 1,1 1,2 /1,3 2,0 2,1 /2,2 2,3 3,0 /1 0 -1 0 /0 1 0 0 /1 0 1 0 /0 0 0 /0 1 0 /-1 0 0 /0 0 0 /",
		// 2x2 with `?`, `+` and a cross on an outer border.
		"pzprv3/lineofsight/2/2/0,- 1,2 /# 2,3 /1 0 1 /0 -1 0 /0 1 /0 0 /0 0 /",
		// 1x1: one cell and the two border layers.
		"pzprv3/lineofsight/1/1/. /0 0 /0 /0 /"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows);
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qdir = 0;
			if (x < 0.45) {
				cell.qnum = -1;
			} else if (x < 0.55) {
				cell.qnum = -2;
				cell.qdir = Math.floor(r.rnd() * 5);
			} else if (x < 0.6) {
				cell.qnum = -3;
			} else {
				cell.qnum = Math.floor(r.rnd() * (maxnum + 1));
				cell.qdir = Math.floor(r.rnd() * 5);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
		p.setConfig("slither_full", r.rnd() < 0.3);
	}
};

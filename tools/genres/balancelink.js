// Fixture inputs for pzprjs pid "balancelink". See tools/gen_fixtures.js for the format.
// No native puzz.link URLs for this pid exist in the dataset crawl, so the long URLs
// below were produced by pinned pzprjs itself (getURL of a filled board).
"use strict";

module.exports = {
	pid: "balancelink",
	dataset: null,
	urls: [
		// pzprjs-native canonical forms.
		"balancelink/3/3/1c0.g7h3",
		"balancelink/6/6/0831g058g10.g830g.5801c.g031g.08g1c0",
		"balancelink/10/10/0.bg2cg0.g62g90gb6gc90.bg2cg0.g62g90gb6gc90.bg2cg0.g62g90gb6gc90.bg2cg0.g62g90gb6gc90.bg2cg0.g62g90g",
		"balancelink/6/6/.g3gcg.g7g3g.g1g7g.gcg1g.g3gcg.g7g3g",
		"https://puzz.link/p?balancelink/6/6/.g3gcg.g7g3g.g1g7g.gcg1g.g3gcg.g7g3g",
		"http://pzv.jp/p.html?type=player&balancelink/3/3/1c0.g7h3",
		"https://puzz.link/p?balancelink_edit/3/3/1c0.g7h3",
		// Single cells: shaded mark 0, clue 1, largest clue 12, `?`, empty.
		"balancelink/1/1/0",
		"balancelink/1/1/1",
		"balancelink/1/1/c",
		"balancelink/1/1/.",
		"balancelink/1/1/-2",
		// `-1` is the ordinary escape for value 1; `--1` parses to -1, which
		// pzprjs treats as "no value" and skips one character.
		"balancelink/2/2/-1",
		"balancelink/2/2/--1",
		// Run lengths, truncated bodies, trailing junk, 1/N and N/1 boards.
		"balancelink/1/8/z1",
		"balancelink/8/1/z1",
		"balancelink/4/4/z",
		"balancelink/3/3/1c0",
		"balancelink/3/3/1c0.g7h3zz",
		"balancelink/3/3",
		"balancelink/3/3/",
		"balancelink/4/4/A"
	],
	fileios: [
		// 3x3 with every cell token and answer borders, crosses and empty rows.
		"pzprv3/balancelink/3/3/1 12 # /-2 . 7 /. . 3 /0 1 /-1 0 /1 0 /0 0 0 /0 0 0 /",
		// 2x2 with the same tokens on the smallest bordered board.
		"pzprv3/balancelink/2/2/# 1 /-2 12 /1 /0 /-1 0 /",
		// Empty board with no answer data.
		"pzprv3/balancelink/2/3/. . /. . /0 0 /0 0 /0 0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.ques = 0;
			if (x < 0.5) {
				cell.qnum = -1;
			} else if (x < 0.62) {
				cell.qnum = -2;
			} else if (x < 0.7) {
				cell.qnum = -1;
				cell.ques = 7;
			} else {
				cell.qnum = 1 + Math.floor(r.rnd() * 12);
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
	}
};

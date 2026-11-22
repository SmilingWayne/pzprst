// Fixture inputs for pzprjs pid "familyphoto". See tools/gen_fixtures.js for the format.
// There is no Family Photo folder under ../puzzlekit-dataset/assets/data, so the
// three real puzzles come from .pzprst-samples.js (native, already canonical).
"use strict";

module.exports = {
	pid: "familyphoto",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"familyphoto/8/8/j4g8o5b8g70143u4p5y3q5j",
		"familyphoto/10/10/gl0a304g4jp4140oa0l1i2m3w11l1k2r4k2l43w3m2i",
		"familyphoto/6/6/059baamgl1l2l3l4l5g",
		"https://puzz.link/p?familyphoto/8/8/j4g8o5b8g70143u4p5y3q5j",
		"http://pzv.jp/p.html?type=player&familyphoto/8/8/j4g8o5b8g70143u4p5y3q5j",
		"familyphoto_edit/8/8/j4g8o5b8g70143u4p5y3q5j",
		"familyphoto/8/8/j4g8o5b8g70143u4p5y3q5j/",
		"familyphoto/8/8/j4g8o5b8g70143u4p5y3q5jzz",
		// Truncated ice layer; pzprjs fills the rest with empties.
		"familyphoto/8/8/j4g8",
		// `?` (`.`) in the clue layer and the ice layer on one cell.
		"familyphoto/4/4/0000.u",
		"familyphoto/4/4/v000.",
		// The largest clue on a 4x4 board is 16, a `-` hex escape.
		"familyphoto/4/4/0000-10",
		// Ice bits (v = cells 0-4) followed by a clue.
		"familyphoto/4/4/v0001",
		// 1/1, 1/N and N/1 boards.
		"familyphoto/1/1/0g",
		"familyphoto/1/1/01",
		"familyphoto/1/1/v",
		"familyphoto/1/5/01",
		"familyphoto/5/1/01",
		"familyphoto/2/2/uj",
		// Missing body, trailing slash and an unknown character.
		"familyphoto/4/4",
		"familyphoto/4/4/",
		"familyphoto/4/4/0000A",
		"familyphoto/x/4/4/0000v"
	],
	fileios: [
		// Question and answer layers: photo marks, `?`, two-digit clues, every
		// border answer token (1, 2 and -1) and the cross-only mark.
		"pzprv3/familyphoto/4/4/#5 . . . /. #- . . /. . 16 . /. . . . /1 0 2 /0 -1 0 /1 0 0 /0 0 1 /0 1 0 0 /2 0 0 0 /0 0 -1 0 /",
		// 3x2: `#` on an empty cell, photo plus `?`, and answer borders.
		"pzprv3/familyphoto/3/2/#- . /. #. /# 1 /1 /0 /-1 /0 1 /-1 2 /",
		// 1x1 has a single empty border line.
		"pzprv3/familyphoto/1/1/#- /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.ques = 0;
			if (x < 0.25) {
				cell.ques = 6;
			}
			const y = r.rnd();
			if (y < 0.15) {
				cell.qnum = -2;
			} else if (y < 0.6 && maxnum >= 1) {
				cell.qnum = 1 + Math.floor(r.rnd() * maxnum);
			} else {
				cell.qnum = -1;
			}
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

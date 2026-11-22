// Fixture inputs for pzprjs pid "circuitwalk". See tools/gen_fixtures.js for the format.
// There is no Circuit Walk folder under ../puzzlekit-dataset/assets/data, and
// .pzprst-samples.js finds no native-derived URLs for this pid in the crawl log.
"use strict";

module.exports = {
	pid: "circuitwalk",
	dataset: null,
	urls: [
		"circuitwalk/3/3/1g1",
		"https://puzz.link/p?circuitwalk/3/3/1g1",
		"http://pzv.jp/p.html?type=player&circuitwalk/3/3/1g1",
		"circuitwalk_edit/3/3/1g1",
		"circuitwalk/f/3/3/1g1",
		"https://puzz.link/p?circuitwalk/f/3/3/1g1",
		"circuitwalk/xf/3/3/1g1",
		"circuitwalk/2/2/0.",
		"circuitwalk/f/2/2/0.i",
		"circuitwalk/2/2/c1",
		"circuitwalk/3/3/00123456789",
		"circuitwalk/4/4/0000f2f2",
		"circuitwalk/4/4/+010",
		"circuitwalk/6/5/000000-1e",
		"circuitwalk/1/1/01",
		"circuitwalk/1/4/01",
		"circuitwalk/4/1/01",
		"circuitwalk/3/3/1g1n/",
		"circuitwalk/3/3/1g1nzz",
		"circuitwalk/3/3/1",
		"circuitwalk/2/2/0zz",
		"circuitwalk/2/2/0",
		"circuitwalk/2/2/H",
		"circuitwalk/2/2/0!",
		"circuitwalk/5/5/0210h10h210",
		"circuitwalk/7/6/000000000000000001",
		"circuitwalk/f/8/3/01000abcdef"
	],
	fileios: [
		// 2x2 with a clue, the two ice answer arcs and inner border lines.
		"pzprv3/circuitwalk/2/2/2 A /. B /0 1 0 /0 0 0 /0 0 /1 0 /0 0 /",
		// 3x3 with the loop_full flag, a crossing arc, ice cells and inner lines.
		"pzprv3/circuitwalk/3/3/f/1 C . /. # # /. . . /0 0 0 0 /0 1 0 0 /0 0 0 0 /0 0 0 /0 1 0 /0 0 0 /0 0 0 /",
		// 1x1: the whole frame is answer border.
		"pzprv3/circuitwalk/1/1/A /-1 0 /0 /0 /",
		// 4x1 (one row) with the loop_full flag, a clue, an inner line and crosses.
		"pzprv3/circuitwalk/1/4/f/# C . 2 /0 1 0 -1 0 /0 0 0 0 /0 0 0 0 /"
	],
	randomize(p, r) {
		const bd = p.board;
		bd.cell.each((cell) => {
			cell.qnum = -1;
			cell.ques = 0;
			cell.qans = 0;
			const x = r.rnd();
			if (x < 0.08) {
				cell.qnum = -2;
			} else if (x < 0.4) {
				cell.ques = 6;
				if (r.rnd() < 0.3) {
					cell.qans = r.pick([1, 2, 3]);
				}
			} else if (x < 0.75) {
				cell.qnum = 1 + Math.floor(r.rnd() * bd.cols * bd.rows);
			}
		});
		// A line on a frame border is illegal (`Border.isLineNG`), and pzprjs
		// crashes rebuilding the line graph of a one-column board after a
		// transform when it contains a horizontal line, so lines are limited to
		// the inner borders of a board with at least two rows and columns.
		// Cross marks are legal everywhere, frame included.
		const canLine = bd.cols >= 2 && bd.rows >= 2;
		bd.border.each((border) => {
			border.line = 0;
			border.qsub = 0;
			const x = r.rnd();
			if (canLine && border.inside && x < 0.25) {
				border.line = 1;
			} else if (x < 0.45) {
				border.qsub = 2;
			}
		});
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

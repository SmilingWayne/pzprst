// Fixture inputs for pzprjs pid "energywalk". See tools/gen_fixtures.js for the format.
// There is no Energy Walk folder under ../puzzlekit-dataset/assets/data, so the
// real-puzzle list is empty.
"use strict";

module.exports = {
	pid: "energywalk",
	dataset: null,
	urls: [
		"energywalk/3/3/1g1",
		"https://puzz.link/p?energywalk/3/3/1g1",
		"http://pzv.jp/p.html?type=player&energywalk/3/3/1g1",
		"energywalk_edit/3/3/1g1",
		"energywalk/f/3/3/1g1",
		"https://puzz.link/p?energywalk/f/3/3/1g1",
		"energywalk/xf/3/3/1g1",
		"energywalk/2/2/0.",
		"energywalk/f/2/2/0.i",
		"energywalk/2/2/c1",
		"energywalk/3/3/00123456789",
		"energywalk/4/4/0000f2f2",
		"energywalk/6/5/000000-1e",
		"energywalk/1/1/01",
		"energywalk/1/4/01",
		"energywalk/4/1/01",
		"energywalk/3/3/1g1n/",
		"energywalk/3/3/1g1nzz",
		"energywalk/3/3/1",
		"energywalk/2/2/0zz",
		"energywalk/2/2/0",
		"energywalk/2/2/H",
		"energywalk/2/2/0!"
	],
	fileios: [
		// 2x2 with a clue, the two ice answer arcs and inner border lines.
		"pzprv3/energywalk/2/2/2 A /. B /0 1 0 /0 0 0 /0 0 /1 0 /0 0 /",
		// 3x3 with the loop_full flag, a crossing arc, ice cells and inner lines.
		"pzprv3/energywalk/3/3/f/1 C . /. # # /. . . /0 0 0 0 /0 1 0 0 /0 0 0 0 /0 0 0 /0 1 0 /0 0 0 /0 0 0 /",
		// 1x1: the whole frame is answer border, so only cross marks are legal.
		"pzprv3/energywalk/1/1/A /-1 0 /0 /0 /",
		// 4x1 (one row) with the loop_full flag, a clue, an inner line and crosses.
		"pzprv3/energywalk/1/4/f/# C . 2 /0 1 0 -1 0 /0 0 0 0 /0 0 0 0 /"
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

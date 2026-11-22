// Fixture inputs for pzprjs pid "icewalk". See tools/gen_fixtures.js for the format.
// There is no Ice Walk folder under ../puzzlekit-dataset/assets/data; the three
// real-puzzle URLs below come from the puzz.link crawl log
// (.pzprst-samples.js icewalk).
"use strict";

module.exports = {
	pid: "icewalk",
	dataset: null,
	urls: [
		"icewalk/3/3/1g1",
		"https://puzz.link/p?icewalk/3/3/1g1",
		"http://pzv.jp/p.html?type=player&icewalk/3/3/1g1",
		"icewalk_edit/3/3/1g1",
		"icewalk/f/3/3/1g1",
		"https://puzz.link/p?icewalk/f/3/3/1g1",
		"icewalk/xf/3/3/1g1",
		"icewalk/2/2/0.",
		"icewalk/f/2/2/0.i",
		"icewalk/2/2/c1",
		"icewalk/3/3/00123456789",
		"icewalk/4/4/0000f2f2",
		"icewalk/4/4/+010",
		"icewalk/6/5/000000-1e",
		"icewalk/1/1/01",
		"icewalk/1/4/01",
		"icewalk/4/1/01",
		"icewalk/3/3/1g1n/",
		"icewalk/3/3/1g1nzz",
		"icewalk/3/3/1",
		"icewalk/2/2/0zz",
		"icewalk/2/2/0",
		"icewalk/2/2/H",
		"icewalk/2/2/0!",
		"https://puzz.link/p?icewalk/9/9/7toa6toq5hrm51ru0w2h34x3x42h2w",
		"https://puzz.link/p?icewalk/10/10/1g9i0000pjpj00009i1g9n7h7p7i7n3zl6naiap5h5n6",
		"https://puzz.link/p?icewalk/6/6/44q05i20n3m5h2m4n"
	],
	fileios: [
		// 2x2 with a clue, the two ice answer arcs and inner border lines.
		"pzprv3/icewalk/2/2/2 A /. B /0 1 0 /0 0 0 /0 0 /1 0 /0 0 /",
		// 3x3 with the loop_full flag, a crossing arc, ice cells and inner lines.
		"pzprv3/icewalk/3/3/f/1 C . /. # # /. . . /0 0 0 0 /0 1 0 0 /0 0 0 0 /0 0 0 /0 1 0 /0 0 0 /0 0 0 /",
		// 1x1: the whole frame is answer border.
		"pzprv3/icewalk/1/1/A /-1 0 /0 /0 /",
		// 4x1 (one row) with the loop_full flag, a clue, an inner line and crosses.
		"pzprv3/icewalk/1/4/f/# C . 2 /0 1 0 -1 0 /0 0 0 0 /0 0 0 0 /"
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

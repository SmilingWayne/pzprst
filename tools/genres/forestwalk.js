// Fixture inputs for pzprjs pid "forestwalk". See tools/gen_fixtures.js for the format.
// There is no Forest Walk folder under ../puzzlekit-dataset/assets/data; the
// three real-puzzle URLs below come from the puzz.link crawl log
// (.pzprst-samples.js forestwalk).
"use strict";

module.exports = {
	pid: "forestwalk",
	dataset: null,
	urls: [
		"forestwalk/3/3/1g1",
		"https://puzz.link/p?forestwalk/3/3/1g1",
		"http://pzv.jp/p.html?type=player&forestwalk/3/3/1g1",
		"forestwalk_edit/3/3/1g1",
		"forestwalk/f/3/3/1g1",
		"https://puzz.link/p?forestwalk/f/3/3/1g1",
		"forestwalk/xf/3/3/1g1",
		"forestwalk/2/2/0.",
		"forestwalk/f/2/2/0.i",
		"forestwalk/2/2/c1",
		"forestwalk/3/3/00123456789",
		"forestwalk/4/4/0000f2f2",
		"forestwalk/4/4/+010",
		"forestwalk/6/5/000000-1e",
		"forestwalk/1/1/01",
		"forestwalk/1/4/01",
		"forestwalk/4/1/01",
		"forestwalk/3/3/1g1n/",
		"forestwalk/3/3/1g1nzz",
		"forestwalk/3/3/1",
		"forestwalk/2/2/0zz",
		"forestwalk/2/2/0",
		"forestwalk/2/2/H",
		"forestwalk/2/2/0!",
		"https://puzz.link/p?forestwalk/5/5/0a2e0r1gam4g4",
		"https://puzz.link/p?forestwalk/6/6/05gpgq00g3h2o2l5o4h4g",
		"https://puzz.link/p?forestwalk/8/8/7001j9d5j001oq3g4s2p4i1o2j5l"
	],
	fileios: [
		// 2x2 with a clue, the two ice answer arcs and inner border lines.
		"pzprv3/forestwalk/2/2/2 A /. B /0 1 0 /0 0 0 /0 0 /1 0 /0 0 /",
		// 3x3 with the loop_full flag, a crossing arc, ice cells and inner lines.
		"pzprv3/forestwalk/3/3/f/1 C . /. # # /. . . /0 0 0 0 /0 1 0 0 /0 0 0 0 /0 0 0 /0 1 0 /0 0 0 /0 0 0 /",
		// 1x1: the whole frame is answer border.
		"pzprv3/forestwalk/1/1/A /-1 0 /0 /0 /",
		// 4x1 (one row) with the loop_full flag, a clue, an inner line and crosses.
		"pzprv3/forestwalk/1/4/f/# C . 2 /0 1 0 -1 0 /0 0 0 0 /0 0 0 0 /"
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

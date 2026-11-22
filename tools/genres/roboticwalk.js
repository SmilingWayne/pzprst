// Fixture inputs for pzprjs pid "roboticwalk". See tools/gen_fixtures.js for the format.
// There is no Robotic Walk folder under ../puzzlekit-dataset/assets/data and
// ../.pzprst-samples.js finds no native-derived link, so the real-puzzle list is
// empty.
"use strict";

module.exports = {
	pid: "roboticwalk",
	dataset: null,
	urls: [
		"roboticwalk/3/3/1g1",
		"https://puzz.link/p?roboticwalk/3/3/1g1",
		"http://pzv.jp/p.html?type=player&roboticwalk/3/3/1g1",
		"roboticwalk_edit/3/3/1g1",
		"roboticwalk/f/3/3/1g1",
		"https://puzz.link/p?roboticwalk/f/3/3/1g1",
		"roboticwalk/xf/3/3/1g1",
		"roboticwalk/2/2/0.",
		"roboticwalk/f/2/2/0.i",
		"roboticwalk/2/2/c1",
		"roboticwalk/3/3/00123456789",
		"roboticwalk/4/4/-10",
		"roboticwalk/6/5/000000-1e",
		"roboticwalk/1/1/01",
		"roboticwalk/1/4/01",
		"roboticwalk/4/1/01",
		"roboticwalk/3/3/1g1n/",
		"roboticwalk/3/3/1g1nzz",
		"roboticwalk/3/3/1",
		"roboticwalk/2/2/0zz",
		"roboticwalk/2/2/0",
		"roboticwalk/2/2/H",
		"roboticwalk/3/3",
		"roboticwalk/3/3/",
		"roboticwalk/4/4/f"
	],
	fileios: [
		// 2x2 with clues, the two ice answer arcs and inner border lines.
		"pzprv3/roboticwalk/2/2/2 A /. B /0 1 0 /0 0 0 /0 0 /1 0 /0 0 /",
		// 3x3 with the loop_full flag, a `?`, ice cells, all three arcs and lines.
		"pzprv3/roboticwalk/3/3/f/. # 4 /- . C /A . . /0 1 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 /1 0 0 /0 0 0 /0 0 0 /",
		// 1x1: the whole frame is answer border, so only arcs are legal.
		"pzprv3/roboticwalk/1/1/A /-1 0 /0 /0 /"
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

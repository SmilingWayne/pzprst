// Fixture inputs for pzprjs pid "waterwalk". See tools/gen_fixtures.js for the format.
// There is no Water Walk folder under ../puzzlekit-dataset/assets/data; the three
// real-puzzle URLs below come from the puzz.link crawl log
// (.pzprst-samples.js waterwalk).
"use strict";

module.exports = {
	pid: "waterwalk",
	dataset: null,
	urls: [
		"waterwalk/3/3/1g1",
		"https://puzz.link/p?waterwalk/3/3/1g1",
		"http://pzv.jp/p.html?type=player&waterwalk/3/3/1g1",
		"waterwalk_edit/3/3/1g1",
		"waterwalk/f/3/3/1g1",
		"https://puzz.link/p?waterwalk/f/3/3/1g1",
		"waterwalk/xf/3/3/1g1",
		"waterwalk/2/2/0.",
		"waterwalk/f/2/2/0.i",
		"waterwalk/2/2/c1",
		"waterwalk/3/3/00123456789",
		"waterwalk/4/4/0000f2f2",
		"waterwalk/4/4/+010",
		"waterwalk/6/5/000000-1e",
		"waterwalk/1/1/01",
		"waterwalk/1/4/01",
		"waterwalk/4/1/01",
		"waterwalk/3/3/1g1n/",
		"waterwalk/3/3/1g1nzz",
		"waterwalk/3/3/1",
		"waterwalk/2/2/0zz",
		"waterwalk/2/2/0",
		"waterwalk/2/2/H",
		"waterwalk/2/2/0!",
		"https://puzz.link/p?waterwalk/10/10/0c3cr0o61mdgc30r6o605i2j6h4r3h6j6h5k6l36l2k6h3j6h2r3h6j3i4",
		"https://puzz.link/p?waterwalk/9/9/7gs80pphoppg13gu03m4k2j2r1i3m3i1r2j3k5m6",
		"https://puzz.link/p?waterwalk/8/8/cpj016cp01j6ci35z2j5h7j3z72i"
	],
	fileios: [
		// 2x2 with a clue, the two ice answer arcs and inner border lines.
		"pzprv3/waterwalk/2/2/2 A /. B /0 1 0 /0 0 0 /0 0 /1 0 /0 0 /",
		// 3x3 with the loop_full flag, a crossing arc, ice cells and inner lines.
		"pzprv3/waterwalk/3/3/f/1 C . /. # # /. . . /0 0 0 0 /0 1 0 0 /0 0 0 0 /0 0 0 /0 1 0 /0 0 0 /0 0 0 /",
		// 1x1: the whole frame is answer border.
		"pzprv3/waterwalk/1/1/A /-1 0 /0 /0 /",
		// 4x1 (one row) with the loop_full flag, a clue, an inner line and crosses.
		"pzprv3/waterwalk/1/4/f/# C . 2 /0 1 0 -1 0 /0 0 0 0 /0 0 0 0 /"
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

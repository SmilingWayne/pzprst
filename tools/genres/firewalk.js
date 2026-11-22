// Fixture inputs for pzprjs pid "firewalk". See tools/gen_fixtures.js for the format.
// There is no Fire Walk folder under ../puzzlekit-dataset/assets/data, so the
// last three entries are native-derived links from ../.pzprst-samples.js.
"use strict";

module.exports = {
	pid: "firewalk",
	dataset: null,
	urls: [
		"firewalk/3/3/1g1",
		"https://puzz.link/p?firewalk/3/3/1g1",
		"http://pzv.jp/p.html?type=player&firewalk/3/3/1g1",
		"firewalk_edit/3/3/1g1",
		"firewalk/f/3/3/1g1",
		"https://puzz.link/p?firewalk/f/3/3/1g1",
		"firewalk/xf/3/3/1g1",
		"firewalk/2/2/0.",
		"firewalk/f/2/2/0.i",
		"firewalk/2/2/c1",
		"firewalk/3/3/00123456789",
		"firewalk/4/4/-10",
		"firewalk/6/5/000000-1e",
		"firewalk/1/1/01",
		"firewalk/1/4/01",
		"firewalk/4/1/01",
		"firewalk/3/3/1g1n/",
		"firewalk/3/3/1g1nzz",
		"firewalk/3/3/1",
		"firewalk/2/2/0zz",
		"firewalk/2/2/0",
		"firewalk/2/2/H",
		"firewalk/3/3",
		"firewalk/4/4/f",
		"https://puzz.link/p?firewalk/10/10/0c3cr0o61mdgc30r6o60q8l7u84m3h7l2h4m65u6l7q",
		"https://puzz.link/p?firewalk/8/8/0024000008g00l8g3s6j7l4g6iaq8q",
		"https://puzz.link/p?firewalk/11/11/1a00004fh7p3s9u8v20000580zj8gag6g3u8k3u2k6u5gbg3g3zj"
	],
	fileios: [
		// 2x2 with clues, the two fire answer arcs and the cross mark layer.
		"pzprv3/firewalk/2/2/2 A /. B /0 1 0 /0 0 0 /0 0 /1 0 /0 0 /1 0 0 /0 2 0 /0 0 1 /",
		// 3x3 with the loop_full flag, a `?`, fire cells, all three arcs, lines
		// and a full cross grid.
		"pzprv3/firewalk/3/3/f/. # 4 /- . C /A . . /0 1 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 /1 0 0 /0 0 0 /0 0 0 /1 0 0 0 /0 2 0 0 /0 0 1 0 /0 0 0 2 /",
		// 4x1 (one row) with the loop_full flag, the fire marks and cross marks.
		"pzprv3/firewalk/1/4/f/# C . 2 /0 1 0 -1 0 /0 0 0 0 /0 0 0 0 /0 1 0 0 0 /0 0 2 0 0 /"
	],
	randomize(p, r) {
		const bd = p.board;
		const maxnum = bd.cols * bd.rows;
		bd.cell.each((cell) => {
			cell.qnum = -1;
			cell.ques = 0;
			cell.qans = 0;
			if (r.rnd() < 0.3) {
				// A fire cell: `posthook.ques` clears the number, so the two
				// never coexist.
				cell.ques = 6;
				cell.qans = r.pick([0, 0, 1, 2, 3]);
			} else {
				const x = r.rnd();
				if (x < 0.12) {
					cell.qnum = -2;
				} else if (x < 0.55) {
					cell.qnum = 1 + Math.floor(r.rnd() * maxnum);
				}
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
		bd.cross.each((cross) => {
			cross.qsub = r.pick([0, 0, 0, 1, 2]);
		});
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

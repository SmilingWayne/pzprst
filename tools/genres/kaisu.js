// Fixture inputs for pzprjs pid "kaisu". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kaisu",
	dataset: null,
	urls: [
		// Canonical form and empty bodies (the marker positions stay implicit).
		"kaisu/4/4",
		"kaisu/4/4/",
		"kaisu/4/4/1141",
		// Full URLs (player and editor) and a pzv.jp player URL.
		"https://puzz.link/p?kaisu/3/3/11310000105",
		"https://puzz.link/p?kaisu_edit/3/3/11310000105",
		"http://pzv.jp/p.html?kaisu/2/2/1121g820",
		// Unknown pflags are ignored (kaisu defines no rule flag).
		"kaisu/f/3/3/11310000105",
		"kaisu/xf/3/3/11310000105",
		// Marker positions equal to the pzprjs default.
		"kaisu/1/5/1511",
		"kaisu/5/1/1151",
		// 1x1, a marked cell and a truncated border layer.
		"kaisu/1/1/",
		"kaisu/1/1/1111i",
		"kaisu/1/1/11110",
		"kaisu/3/3/1131",
		// Truncated body, trailing junk and 1/N / N/1 boards.
		"kaisu/3/3/11310000",
		"kaisu/3/3/11310000105zz",
		"kaisu/1/25",
		"kaisu/25/1/11211",
		// Base-36 skips (`g`, `i`) and a 5x5 body that ends early.
		"kaisu/3/3/1131000010g",
		"kaisu/5/5/1151000000i",
		"kaisu/6/6/1161"
	],
	fileios: [
		// 3x3: marker line, empty walls, 0/1/. cells and the border answer layer.
		"pzprv3.1/kaisu/3/3/1 1 5 1 /0 0 /0 0 /0 0 /0 0 0 /0 0 0 /0 1 0 /1 . 0 /0 0 1 /1u -1 /0 0 /0 1d /0 0 1r /1l 0 0 /",
		// 2x2 with walls drawn and a `-1` cross.
		"pzprv3.1/kaisu/2/2/1 1 3 1 /1 /0 /0 1 /1 0 /. 1 /0 /-1 /0 0 /",
		// 2x2 with an old (filever 0) border-line layer.
		"pzprv3/kaisu/2/2/1 1 3 1 /1 /0 /0 1 /1 0 /. 1 /0 /-1 /0 0 /",
		// 1x1: every border range is empty.
		"pzprv3.1/kaisu/1/1/1 1 1 1//1 //"
	],
	randomize(p, r) {
		const bd = p.board;
		const pos = (n) => 2 * (1 + Math.floor(r.rnd() * n)) - 1;
		bd.startpos.bx = pos(bd.cols);
		bd.startpos.by = pos(bd.rows);
		// A 1x1 board has nowhere else to put the goal. On larger boards the
		// two markers must differ: pzprjs resets an equal pair after every
		// board operation, which makes the URL orbit of such a board strictly
		// larger than the orbit of its transforms.
		let goal;
		do {
			goal = [pos(bd.cols), pos(bd.rows)];
		} while (bd.cols * bd.rows > 1 && goal[0] === bd.startpos.bx && goal[1] === bd.startpos.by);
		bd.goalpos.bx = goal[0];
		bd.goalpos.by = goal[1];
		bd.border.each((border) => {
			border.ques = r.rnd() < 0.2 ? 1 : 0;
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.line = 1;
			} else if (x < 0.3) {
				border.qsub = 2;
			} else if (x < 0.45) {
				border.qsub = r.pick([11, 12, 13, 14]);
			}
		});
		bd.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, 0, 1]);
		});
	}
};

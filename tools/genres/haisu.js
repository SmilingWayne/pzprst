// Fixture inputs for pzprjs pid "haisu". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "haisu",
	dataset: null,
	urls: [
		// Canonical form and empty bodies (the marker positions stay implicit).
		"haisu/4/4",
		"haisu/4/4/",
		"haisu/4/4/1141",
		// Full URLs (player and editor) and a pzv.jp player URL.
		"https://puzz.link/p?haisu/3/3/1131k0h0123456788",
		"https://puzz.link/p?haisu_edit/3/3/1131k0h0123456788",
		"http://pzv.jp/p.html?haisu/2/2/1121g81234",
		// Unknown pflags are ignored (haisu defines no rule flag).
		"haisu/f/3/3/1131k0h0123456788",
		"haisu/xf/3/3/1131k0h0123456788",
		// Marker positions equal to the pzprjs default.
		"haisu/1/5/1511",
		"haisu/5/1/1151",
		// 1x1, an explicit cell 0 and a truncated border layer.
		"haisu/1/1/",
		"haisu/1/1/1111",
		"haisu/1/1/11110",
		"haisu/3/3/1131k0",
		// Truncated body, trailing junk and 1/N / N/1 boards.
		"haisu/3/3/1131k0h0",
		"haisu/3/3/1131k0h0123456788zz",
		"haisu/1/25",
		"haisu/25/1/11211",
		// A clue that fills its room (`-` escape for 25) and a 6x6 truncation.
		"haisu/5/5/115100000000000000000-19",
		"haisu/6/6/1161"
	],
	fileios: [
		// 3x3: marker line, walls, numbers 1-8, borders with aux marks.
		"pzprv3.1/haisu/3/3/1 1 5 1 /1 0 /1 0 /0 0 /1 0 0 /0 1 0 /1 2 3 /4 5 6 /7 8 8 /1u -1 /0 0 /0 1d /0 0 1r /1l 0 0 /",
		// 2x2 with an old (filever 0) border-line layer.
		"pzprv3/haisu/2/2/1 1 3 1 /1 /0 /0 1 /1 2 /3 4 /0 /-1 /0 0 /",
		// 1x1: every border range is empty.
		"pzprv3.1/haisu/1/1/1 1 1 1//. //"
	],
	randomize(p, r) {
		const bd = p.board;
		const pos = (n) => 2 * (1 + Math.floor(r.rnd() * n)) - 1;
		bd.startpos.bx = pos(bd.cols);
		bd.startpos.by = pos(bd.rows);
		// A 1x1 board has nowhere else to put the goal. On larger boards the
		// two markers must differ: pzprjs resets an equal pair after every
		// board operation, which makes the URL orbit of such a board strictly
		// larger than the orbit of its transforms (dedupe-key invariant).
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
		// Numbers must fit their room (`Cell@haisu.maxnum` is the room size).
		bd.rebuildInfo();
		bd.cell.each((cell) => {
			cell.qnum = -1;
			if (r.rnd() < 0.5) {
				cell.qnum = 1 + Math.floor(r.rnd() * cell.room.clist.length);
			}
		});
	}
};

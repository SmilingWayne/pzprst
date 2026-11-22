// Fixture inputs for pzprjs pid "amibo". See tools/gen_fixtures.js for the format.
// "ruleofthree" shares pzprjs's variety/amibo.js but overrides Board.hasborder
// and Encode, so it is a separate pid and is not covered here.
"use strict";

module.exports = {
	pid: "amibo",
	dataset: null,
	urls: [
		// Hand-built 3x3 numbers and `?`.
		"amibo/3/3/232323232",
		"amibo/3/3/2.3.2.3..",
		"https://puzz.link/p?amibo/3/3/232323232",
		"https://puzz.link/p?amibo_edit/3/3/232323232",
		"http://pzv.jp/p.html?type=player&amibo/3/3/232323232",
		// Unknown flag.
		"amibo/x/3/3/232",
		// Base-16 fallback: 10 and 256 as escapes, `?`, then blanks.
		"amibo/3/3/-+00a+100.",
		"amibo/12/12/-+00b",
		// Base-36 run lengths, 1x1, 1/N and N/1 boards.
		"amibo/4/4/g",
		"amibo/1/1/2",
		"amibo/1/5/.....",
		"amibo/5/1/.....",
		// Truncated body, trailing junk and empty bodies.
		"amibo/3/3/2",
		"amibo/3/3/232zz",
		"amibo/3/3",
		"amibo/3/3/",
		// Real puzzle URLs (puzz.link).
		"https://puzz.link/p?amibo/6/6/e.g2c.b.f4f5a",
		"https://puzz.link/p?amibo/8/8/f3e3c6n5c4k6d4f2b2a",
		"https://puzz.link/p?amibo/7/7/i.b.b.b.b.b.d.c.c.c.g.b./"
	],
	fileios: [
		// 3x3 with numbers, `?`, `x`, every direction bar, a circle and lines.
		"pzprv3/amibo/3/3/2 l 3 /- 2 + /3 o# x /1 -1 /0 0 /0 0 /0 0 0 /0 0 0 /",
		// 2x2 with a circle, a diagonal bar, the other diagonal bar and a cross.
		"pzprv3/amibo/2/2/o2 l /- x /0 /0 /0 0 /",
		// 1x1: no inner borders, so the border layer is empty.
		"pzprv3/amibo/1/1/2 //"
	],
	randomize(p, r) {
		const maxbx = 2 * p.board.cols;
		const maxby = 2 * p.board.rows;
		p.board.cell.each((cell) => {
			const col = (cell.bx < maxbx >> 1 ? maxbx - cell.bx : cell.bx) >> 1;
			const row = (cell.by < maxby >> 1 ? maxby - cell.by : cell.by) >> 1;
			const maxnum = Math.max(col, row);
			const x = r.rnd();
			cell.qnum = -1;
			cell.ques = 0;
			cell.qans = 0;
			if (x < 0.35) {
				// empty
			} else if (x < 0.45) {
				cell.qnum = -2;
			} else if (x < 0.55) {
				cell.qans = r.pick([11, 12, 13]);
			} else if (x < 0.62) {
				cell.ques = 7;
			} else if (maxnum >= 2) {
				cell.qnum = 2 + Math.floor(r.rnd() * (maxnum - 1));
			}
			cell.qsub = r.rnd() < 0.2 ? 1 : 0;
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

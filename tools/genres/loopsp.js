// Fixture inputs for pzprjs pid "loopsp". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "loopsp",
	dataset: null,
	urls: [
		// Hand-built 3x3: clues, `?` and line parts 11/12.
		"loopsp/3/3/1.2..g.5",
		"https://puzz.link/p?loopsp/3/3/1.2..g.5",
		"https://puzz.link/p?loopsp_edit/3/3/1.2..g.5",
		"http://pzv.jp/p.html?type=player&loopsp/3/3/1.2..g.5",
		// Every line part g-m on 1x1 boards.
		"loopsp/1/1/g",
		"loopsp/1/1/m",
		// `?` alone.
		"loopsp/1/1/.",
		// Largest single-digit clue and the -/+ escapes.
		"loopsp/1/1/f",
		"loopsp/1/1/-0f",
		"loopsp/1/1/+100",
		// 3x3 mixing all four escape forms.
		"loopsp/3/3/1-0f+100",
		// Base-36 run lengths, 1/N and N/1 boards.
		"loopsp/1/5/z",
		"loopsp/5/1/z",
		"loopsp/4/4/z",
		"loopsp/2/2/zz",
		// Truncated body, trailing junk and empty bodies.
		"loopsp/2/2/1",
		"loopsp/3/3/12345678gzz",
		"loopsp/3/3",
		"loopsp/3/3/",
		"loopsp/4/4/.",
		// Real sample URLs.
		"https://puzz.link/p?loopsp/9/7/zslnilnilo1n2kn3gohnjinigzs",
		"https://puzz.link/p?loopsp/8/8/p1r1nlogp2o.o3jyk.o.o3pjojn1r.p",
		"https://puzz.link/p?loopsp/8/8/q1r2pin2omn2qmnmq1nmo3r2nmoi3omw"
	],
	fileios: [
		// 3x3 with clues, `?`, a line part and a loop with one cross.
		"pzprv3/loopsp/3/3/1 -2 2 /-2 -2 a /-2 5 . /1 0 /0 -1 /1 1 /0 1 0 /1 0 0 /",
		// 2x3 with a circle, `-2`, a line part and answer lines.
		"pzprv3/loopsp/3/2/1 -2 /2 . /. o /0 /-1 /0 /0 1 /1 0 /",
		// 1x1 with no inner borders, so the border layer is empty.
		"pzprv3/loopsp/1/1/-2 /"
	],
	randomize(p, r) {
		const maxnum = (p.board.cols * p.board.rows) / 4 | 0;
		p.board.cell.each((cell) => {
			cell.ques = 0;
			cell.qnum = -1;
			const x = r.rnd();
			if (x < 0.45) {
				if (r.rnd() < 0.25) {
					cell.qnum = -2;
				}
			} else if (x < 0.62) {
				cell.ques = r.pick([-2, 6, 11, 12, 13, 14, 15, 16, 17]);
			} else {
				cell.qnum = Math.floor(r.rnd() * (maxnum + 1));
			}
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

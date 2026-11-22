// Fixture inputs for pzprjs pid "pipelinkr". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "pipelinkr",
	dataset: "Pipelink",
	urls: [
		"https://puzz.link/p?pipelinkr/8/8/mciciboc0h0hc0lbs0hbh0cp0i",
		"https://puzz.link/p?pipelinkr/7/7/h0ncibm0ibqck0ck0k",
		"https://puzz.link/p?pipelinkr/6/6/igfj0i0h0h0q0i0jdei",
		"pipelinkr/8/8/mciciboc0h0hc0lbs0hbh0cp0i",
		"pipelinkr/3/3/0",
		"pipelinkr/3/3/9",
		"pipelinkr/3/3/6a",
		"pipelinkr/3/3/.",
		"pipelinkr/3/3/",
		"pipelinkr/2/2/0",
		"pipelinkr/3/3/0zz",
		"pipelinkr/3/3/00",
		"pipelinkr/1/1/0",
		"pipelinkr/1/4/0",
		"pipelinkr/4/1/0",
		"pipelinkr/2/3/00ab",
		"pipelinkr/2/2/9",
		"pipelinkr/2/2/z",
		"pipelinkr/3/3/azzz",
		"http://pzv.jp/p.html?type=player&pipelinkr/3/3/0"
	],
	fileios: [
		"pzprv3/pipelinkr/2/2/circle/o . /. . /0 /0 /0 0 /",
		"pzprv3/pipelinkr/2/2/circle/- o /. c /1 /-1 /0 1 /",
		"pzprv3/pipelinkr/1/1/circle/o //"
	],
	randomize(p, r) {
		const maxnum = (p.board.cols * p.board.rows) / 4 | 0;
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 0, 0, -2, 6, 6, 11, 12, 13, 14, 15, 16, 17]);
			cell.qnum = maxnum > 0 && r.rnd() < 0.2 ? 1 + Math.floor(r.rnd() * maxnum) : -1;
		});
		p.board.border.each((border) => {
			if (r.rnd() < 0.2) {
				border.line = 1;
			} else if (r.rnd() < 0.15) {
				border.qsub = 2;
			}
		});
	}
};

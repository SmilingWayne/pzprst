// Fixture inputs for pzprjs pid "mejilink". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "mejilink",
	dataset: "Mejilink",
	urls: [
		"mejilink/7/7/9l6nk6qdbkuh57kvs",
		"https://puzz.link/p?mejilink/8/8/2e9g6t79qfrg86slg3vpv7o",
		"https://puzz.link/p?mejilink/14/12/mat0kk2l8133ktpal8l9ilatalla1lapa8d9j7m203o48019009voc00001bvo",
		"http://pzv.jp/p.html?mejilink/7/7/9l6nk6qdbkuh57kvs",
		"mejilink_edit/4/4/0",
		"http://pzv.jp/p.html?type=player&mejilink/4/4/0",
		"mejilink/4/4/0",
		"mejilink/7/7/9l6",
		"mejilink/7/7/9l6nk6qdbkuh57kvs",
		"mejilink/7/7/9l6nk6qdbkuh57kvszz",
		"mejilink/4/4/v",
		"mejilink/3/3/123",
		"mejilink/1/1",
		"mejilink/1/1/0",
		"mejilink/1/4/0",
		"mejilink/4/1/0",
		"mejilink/4/4/..",
		"mejilink/9/9/-50",
		"mejilink/16/16/0000000000000000000000000000000000000"
	],
	fileios: [
		"pzprv3/mejilink/2/2/0 2 -1 /0 1 2 /1 -1 /0 0 /0 0 /",
		"pzprv3/mejilink/3/3/0 1 2 -1 /1 0 2 1 /-1 2 0 0 /1 2 0 /-1 0 1 /2 1 -1 /0 0 2 /",
		"pzprv3/mejilink/1/1/1 2 /0 /-1 /"
	],
	randomize(p, r) {
		p.board.border.each((border) => {
			border.ques = 0;
			border.line = 0;
			border.qsub = 0;
			const x = r.rnd();
			if (x < 0.4) {
				border.ques = 1;
			} else if (x < 0.6) {
				border.line = 1;
			} else if (x < 0.8) {
				border.qsub = 2;
			}
		});
	}
};

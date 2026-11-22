// Fixture inputs for pzprjs pid "numlin". See tools/gen_fixtures.js for the format.
// "arukone" shares pzprjs's variety/numlin.js but overrides Encode/FileIO, so it is
// a separate pid and is not covered here.
"use strict";

module.exports = {
	pid: "numlin",
	dataset: "NumberLink",
	urls: [
		"numlin/2/2/12",
		"https://puzz.link/p?numlin/10/10/j12k3l3h4l5k62x16l5j7s7l4q",
		"https://puzz.link/p?numlin/10/10/r12n3j3q4j56i71i87j8q4j5n62r",
		"https://puzz.link/p?numlin/12/21/1p-11h9lei4ncjbjcl2jdwaldh2p-11i7j9j5nft6n-10s3peh1lfu7n4h8g6jbg8j5jau3p-10",
		"numberlink/2/2/12",
		"http://pzv.jp/p.html?type=player&numlin/2/2/12",
		"numlin_edit/3/3/123",
		"numlin/4/4/1",
		"numlin/4/4/1zzzzzz",
		"numlin/4/4",
		"numlin/4/4/",
		"numlin/4/4//",
		"numlin/1/1/1",
		"numlin/2/1/1g",
		"numlin/1/2/g",
		"numlin/30/1/z1",
		"numlin/1/30/zp",
		"numlin/x/4/4/12",
		"numlin/2/2/--1",
		"numlin/5/5/1.2.3.",
		"numlin/4/4/+3e7"
	],
	fileios: [
		"pzprv3/numlin/2/2/1 2 /. . /1 /-1 /1 0 /",
		"pzprv3/numlin/3/3/1 . 2 /. 3 . /. . . /0 1 /-1 0 /0 1 /1 0 0 /0 1 0 /",
		"pzprv3/numlin/1/1/- //"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.5) {
				cell.qnum = -1;
			} else if (x < 0.62) {
				cell.qnum = -2;
			} else {
				cell.qnum = 1 + Math.floor(r.rnd() * 999);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.3) {
				border.line = 1;
			} else if (x < 0.45) {
				border.qsub = 2;
			}
		});
	}
};

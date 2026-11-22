// Fixture inputs for pzprjs pid "heavydots". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "heavydots",
	// No Heavy Dots folder in ../puzzlekit-dataset/assets/data, and
	// .pzprst-samples.js finds no native-derived URLs in the crawl log.
	dataset: null,
	urls: [
		"heavydots/4/4/1u000000000",
		"https://puzz.link/p?heavydots/4/4/1u000000000",
		"http://pzv.jp/p.html?type=player&heavydots/4/4/1u000000000",
		"heavydots/4/4/1u000000000/",
		"heavydots/4/4/1u000000000zz",
		"heavydots/4/4/1",
		"heavydots/4/4/",
		"heavydots/4/4/-10u000000000",
		"heavydots/3/3/123456789",
		"heavydots/3/3/.n000000",
		"heavydots/2/2/jabc",
		"heavydots/2/2/jq00",
		"heavydots/2/2/jzzz",
		"heavydots/1/1/1",
		"heavydots/1/1/",
		"heavydots/1/4/1234",
		"heavydots/4/1/1234",
		"heavydots/2/1/12",
		"heavydots/20/20/1"
	],
	fileios: [
		"pzprv3/heavydots/2/2/1 2 /- . /3 . 4 /. 3 . /. . 4 /1 /-1 /2 0 /",
		"pzprv3/heavydots/1/1/. /. /. /. /",
		"pzprv3/heavydots/3/2/1 - /3 . /. . /3 . 4 /. 3 . /. . 4 /3 . . /1 /-1 /0 /2 0 /0 0 /",
		"pzprv3/heavydots/2/2/. . /. . /. . . /. . . /. . . /0 /0 /0 0 /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.5) {
				cell.qnum = -1;
			} else if (x < 0.6) {
				cell.qnum = -2;
			} else {
				cell.qnum = 1 + Math.floor(r.rnd() * maxnum);
			}
		});
		// pzprjs Cross has minnum 3 / maxnum 4; -2 is not a legal dot value.
		p.board.cross.each((cross) => {
			cross.qnum = r.pick([-1, -1, -1, -1, -1, 3, 4]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.qans = 1;
			} else if (x < 0.4) {
				border.qsub = 1;
			}
		});
	}
};

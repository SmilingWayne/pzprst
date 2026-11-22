// Fixture inputs for pzprjs pid "circlesquare". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "circlesquare",
	dataset: null,
	urls: [
		"circlesquare/11/11/a6k6b3b3001600b000a006i003i0160023j6j6a3i",
		"https://puzz.link/p?circlesquare/20/20/39009c00339i2316000062363090010020990030300609019900030604600000030i1000030003211301000000900002300109000b060010016b3333b300000003ib00",
		"https://puzz.link/p?circlesquare/8/8/h1o0004300010609910g00",
		"http://pzv.jp/p.html?circlesquare/8/8/h1o0004300010609910g00",
		"circlesquare_edit/4/4/0000",
		"http://pzv.jp/p.html?type=player&circlesquare/4/4/0000",
		"circlesquare/4/4/0000",
		"circlesquare/2/2/0",
		"circlesquare/2/2/q",
		"circlesquare/3/3/qqq",
		"circlesquare/3/3/123",
		"circlesquare/3/3/000",
		"circlesquare/3/3/zzz",
		"circlesquare/3/3/ZZZ",
		"circlesquare/4/4/",
		"circlesquare/4/4/000000zz",
		"circlesquare/1/1",
		"circlesquare/1/1/1",
		"circlesquare/1/4/00",
		"circlesquare/4/1/00",
		"circlesquare/9/9/-50",
		"circlesquare/16/16/000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
	],
	fileios: [
		"pzprv3/circlesquare/8/8/1 2 2 . . 1 2 2 /. . . . . . . . /. . . 1 1 . 1 . /. . . . . . . . /. . . 1 . . . . /2 . . . . 1 . . /1 . . . . 1 . . /. 1 2 1 . . . . /. # + . . # . /# . . . + . . . /. # . . . . # . /. . + # . . . . /. . . . # . . . /. + . . . # . . /. . . # . . + . /. . . . . # . . /",
		"pzprv3/circlesquare/3/3/1 2 . /. . 1 /2 . 1 /. # + /# . . /+ # . /",
		"pzprv3/circlesquare/1/1/- /. /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, 1, 2, -2]);
			cell.qans = r.rnd() < 0.25 ? 1 : 0;
			cell.qsub = r.rnd() < 0.25 ? 1 : 0;
		});
	}
};

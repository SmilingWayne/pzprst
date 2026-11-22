// Fixture inputs for pzprjs pid "slither". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "slither",
	dataset: "Slitherlink",
	urls: [
		"slither/3/2/h12g3",
		"slither/3/2/h17d",
		"slitherlink/3/2/h17d",
		"https://puzz.link/p?slither/3/2/h17d",
		"https://puzz.link/p?slitherlink_edit/3/2/h17d",
		"http://pzv.jp/p.html?type=player&slither/3/2/h17d",
		"slither/3/2/h17d/",
		"slither/3/2/h17d//",
		"slither/3/2/h17dzzz",
		"slither/3/2/HZ",
		"slither/3/2/zzzzzzzzzz",
		"slither/3/2",
		"slither/3/2/",
		"slither/3/2/5",
		"slither/3/2/7",
		"slither/1/1/.",
		"slither/1/1/0",
		"slither/2/1/1./",
		"slither/f/3/2/h17d",
		"slither/xf/3/2/h17d",
		"slither/x/3/2/h17d",
		"slither/5/5/cbcbcddad",
		"slither/25/1/0123401234012340123401234",
		"slither/1/25/0123401234012340123401234",
		"slither/30/1/zzk",
		"slither/7/7/hdh76022adch11286671bh23c23",
		"slither/10/10/q8b1bi570b0d08ccdnbc83a3a368ai1b6do",
		"slither/7/7/c58d7bg7286a8ablc758",
		"slither/4/4/........",
		"slither/4/4/.g.g.g.g"
	],
	fileios: [
		"pzprv3/slither/1/2/1 - /0 1 1 /-1 0 /0 0 /",
		"pzprv3.1/slither/2/3/f/. . . /. . . /0 0 0 /0 0 0 /1 0 0 1 /1 1 0 0 /1 1 1 /0 1 1 /1 0 0 /",
		"pzprv3.1/slither/1/5/. . . . . /0 0 0 0 0 /0 1 0 0 0 0 /1 0 0 0 0 /0 1 0 0 0 /",
		"pzprv3/slither/5/5/2 . . 1 . /. 2 . . 1 /. . 2 . . /3 . . 3 . /. 0 . . 3 /1 -1 0 -1 -1 1 /1 1 -1 1 -1 1 /1 1 1 0 0 1 /1 1 1 1 1 1 /-1 -1 -1 0 1 1 /1 1 1 1 1 /-1 1 1 -1 -1 /0 -1 1 0 0 /0 0 0 1 0 /1 -1 1 -1 0 /-1 0 -1 0 1 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
		p.setConfig("slither_full", r.rnd() < 0.3);
	}
};

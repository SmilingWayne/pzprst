// Fixture inputs for pzprjs pid "twinarea". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "twinarea",
	dataset: null,
	urls: [
		"twinarea/5/5/duumh4uazksj34f",
		"twinarea/2/3/ca3k00",
		"https://puzz.link/p?twinarea/5/5/duumh4uazksj34f",
		"http://pzv.jp/p.html?type=player&twinarea/3/3/",
		"twinarea/3/3/",
		"twinarea/3/3/00",
		"twinarea/3/3/0000o00zz",
		"twinarea/1/1/g0",
		"twinarea/1/3/0i0",
		"twinarea/3/1/0i0",
		"twinarea/2/2/00+3e7i0",
		"twinarea/2/2/00-2ci0",
		"twinarea/2/2/006io",
		"twinarea/3/3/tgrgh5l00",
		"twinarea/4/4/i90i90co3j-2cgg00",
		"twinarea/6/5/"
	],
	fileios: [
		"pzprv3/twinarea/5/5/0 1 1 0 /1 1 1 1 /1 0 1 1 /1 1 0 1 /0 1 1 0 /1 0 0 0 1 /0 0 1 0 0 /1 1 1 1 0 /0 1 0 1 0 /- - - . . /- . . - - /. . . - - /. . - . . /. - - - - /. + 2 . 4 /2 . . 4 . /. . 5 . . /. 1 . 3 . /3 . . . 5 /",
		"pzprv3/twinarea/3/3/1 1 /1 0 /1 1 /1 1 0 /1 1 1 /. . 5 /. . . /. . . /. 1 . /2 . 3 /. 4 . /",
		"pzprv3/twinarea/2/2/0 /0 /0 0 /-6 - /. . /. . /. . /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 1, 2, 3, -2]);
			cell.ques = r.rnd() < 0.2 ? 6 : 0;
			cell.anum = r.pick([-1, -1, -1, 1, 2, 3]);
			cell.qsub = r.pick([0, 0, 0, 1, 2]);
			cell.qcmp = r.rnd() < 0.1 ? 1 : 0;
		});
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.25 ? 1 : 0;
		});
	}
};

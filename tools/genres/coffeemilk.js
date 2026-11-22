// Fixture inputs for pzprjs pid "coffeemilk". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "coffeemilk",
	dataset: null,
	urls: [
		"coffeemilk/4/4/p",
		"coffeemilk/4/4/12",
		"coffeemilk/4/4/1",
		"coffeemilk/4/4/.",
		"coffeemilk/4/4/z1",
		"coffeemilk/4/4/1zz",
		"coffeemilk/1/1/1",
		"coffeemilk/1/5/12.12",
		"coffeemilk/5/1/12.12",
		"coffeemilk/5/5/12.12",
		"coffeemilk/2/2/22",
		"coffeemilk/2/2/..",
		"coffeemilk/2/2/-0f",
		"coffeemilk/2/2/--ff",
		"coffeemilk/2/2/-=fff",
		"coffeemilk/2/2/-@000",
		"coffeemilk/2/2/-*0000",
		"coffeemilk/2/2/-$00000",
		"coffeemilk/2/2/A1",
		"coffeemilk/2/2/+1",
		"coffeemilk/2/2/-1g",
		"coffeemilk/2/2/--zz",
		"coffeemilk/f/2/2/12",
		"https://puzz.link/p?coffeemilk/4/4/12",
		"https://puzz.link/p?coffeemilk_play/3/3/12.",
		"http://pzv.jp/p.html?type=player&coffeemilk/3/3/12.",
		"https://pzprxs.vercel.app/p?coffeemilk/3/3/12."
	],
	fileios: [
		"pzprv3/coffeemilk/2/2/1 2 . - /0 /-1 /1 0 /",
		"pzprv3/coffeemilk/3/2/1 2 - /2 1 - /0 1 /-1 0 /1 -1 0 /",
		"pzprv3/coffeemilk/2/2/10 15 1 2 /0 /0 /0 0 /",
		"pzprv3/coffeemilk/2/2/256 4096 8192 12240 /0 /0 /0 0 /",
		"pzprv3/coffeemilk/2/3/1 - 2 /- 1 . /0 0 /0 /0 /0 /0 0 /0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 1, 2, -2]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

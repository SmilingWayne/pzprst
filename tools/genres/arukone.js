// Fixture inputs for pzprjs pid "arukone". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "arukone",
	dataset: null,
	urls: [
		// Real puzzles: pzprjs test/script/arukone.js and dataset samples.
		"arukone/4/4/12l3123j",
		"arukone/6/6/j-13gflfk-10g-10p-13i",
		"arukone/10/10/g11j22h3h45g6i3h45g6m7k889g7ahbi9hagcbncdefj-10hdefg-11-11g-10s",
		"arukone/10/10/k2zl5l6j7o1k2n6h34g7j1j4h5k3k",
		"https://puzz.link/p?arukone/10/10/k2zl5l6j7o1k2n6h34g7j1j4h5k3k",
		"https://puzz.link/p?arukone_edit/4/4/12l3123j",
		"http://pzv.jp/p.html?type=player&arukone/4/4/12l3123j",
		"arukone/e/4/4/12l3123j",
		"arukone/x/4/4/12l3123j",
		"arukone/4/4/12l3123j/",
		"arukone/4/4/12l3123jzz",
		"arukone/4/4/12l",
		"arukone/4/4/",
		"arukone/4/4",
		"arukone/1/1/.",
		"arukone/1/1/1",
		"arukone/52/1/-34",
		"arukone/1/52/-34",
		"arukone/2/2/....",
		"arukone/30/1/zzk"
	],
	fileios: [
		// Real 4x4 board from pzprjs test/script/arukone.js, with answer lines.
		"pzprv3/arukone/4/4/passallcell/A B . . /. . . . /C A B C /. . . . /0 1 0 /0 0 1 /0 0 0 /0 0 0 /0 0 1 0 /0 0 1 0 /0 0 0 0 /",
		// dontpassallcell, letters z (52) and ? in the top row.
		"pzprv3/arukone/2/2/dontpassallcell/z - /. . /0 /0 /0 0 /",
		// 1x1: one cell, then the empty vertical-border line.
		"pzprv3/arukone/1/1/passallcell/1 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, -2, 1, 2, 3, 26, 27, 52]);
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
		p.setConfig("dontpassallcell", r.rnd() < 0.3);
	}
};

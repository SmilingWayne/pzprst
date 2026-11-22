// Fixture inputs for pzprjs pid "hashikake". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "hashikake",
	dataset: "Hashi",
	urls: [
		// Real puzzles: pzprjs test/script/hashikake.js and dataset samples.
		"hashikake/5/5/4g2i3h23k3g1g3g4g3",
		"hashi/5/5/4g2i3h23k3g1g3g4g3",
		"bridges/5/5/4g2i3h23k3g1g3g4g3",
		"https://puzz.link/p?hashi/9/9/g2g2g4g2g2g3g1k1g2g4h25g4g2g1k2g3k1g4g2g43h3g4g2k3g2g3g2g4g3g2g",
		"https://puzz.link/p?hashi/9/9/g1g4h4g22g3g1h1h2g1j23g4g4g5i3g2g3g2i2g2g2g33j2g3h2h3g1g23g3h2g2g",
		"https://puzz.link/p?bridges/5/5/4g2i3h23k3g1g3g4g3",
		"http://pzv.jp/p.html?type=player&hashi/5/5/4g2i3h23k3g1g3g4g3",
		"hashi_edit/5/5/4g2i3h23k3g1g3g4g3",
		"hashi/x/5/5/4g2i3h23k3g1g3g4g3",
		"hashi/5/5/4g2i3h23k3g1g3g4g3/",
		"hashi/5/5/4g2i3h23k3g1g3g4g3zz",
		"hashi/5/5/4g2i3",
		"hashi/5/5/",
		"hashi/5/5",
		"hashi/1/1/.",
		"hashi/1/1/8",
		"hashi/1/1/9",
		"hashi/10/1/8",
		"hashi/1/10/8",
		"hashi/2/2/....",
		"hashi/30/1/zzk"
	],
	fileios: [
		// Real 5x5 board from pzprjs test/script/hashikake.js, with single,
		// double and crossed-out bridge positions.
		"pzprv3/hashikake/5/5/4 . 2 . . /. 3 . . 2 /3 . . . . /. 3 . 1 . /3 . 4 . 3 /2 0 1 0 /0 1 0 0 /1 0 0 2 /0 0 1 0 /0 2 0 0 /1 0 0 0 0 /0 1 2 0 0 /0 0 0 1 0 /0 1 0 0 0 /",
		// ? and the largest clue, double bridge and cross.
		"pzprv3/hashikake/2/2/1 - /. 8 /2 0 /0 1 /1 0 2 /",
		// 1x1: no inner borders, so the border layer is one empty line.
		"pzprv3/hashikake/1/1/8 //"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -2, 1, 2, 3, 4, 5, 6, 7, 8]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.line = 1;
			} else if (x < 0.3) {
				border.line = 2;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

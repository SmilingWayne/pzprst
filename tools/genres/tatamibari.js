// Fixture inputs for pzprjs pid "tatamibari". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tatamibari",
	dataset: "Tatamibari",
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"tatamibari/10/10/i2i2g12j3j2k3g2i32j1j3g2k2k3g2k2h1g13h2k2g1g2j1h2i3h2",
		"tatamibari/10/10/2g2h2o2h3g1k1k3h32h21g22l2g3j1g1h1m1i2g2h3k1g3h2g3g3i2g",
		"tatamibari/10/10/j1g22h12k2k1g13i3j1i13i3h3g1h1h3j3i2g1h2k3j2g2j1g3k23g2g",
		"https://puzz.link/p?tatamibari/10/10/i2i2g12j3j2k3g2i32j1j3g2k2k3g2k2h1g13h2k2g1g2j1h2i3h2",
		"https://puzz.link/p?tatamibari_edit/10/10/2g2h2o2h3g1k1k3h32h21g22l2g3j1g1h1m1i2g2h3k1g3h2g3g3i2g",
		"http://pzv.jp/p.html?type=player&tatamibari/10/10/j1g22h12k2k1g13i3j1i13i3h3g1h1h3j3i2g1h2k3j2g2j1g3k23g2g",
		// Every token class: three clue letters, `?`, empty runs and the
		// "unrecognized character skips two cells" branch.
		"tatamibari/3/3/123.2g1",
		"tatamibari/3/3/123zz",
		"tatamibari/2/2/.",
		"tatamibari/2/2/321",
		"tatamibari/3/3/2",
		"tatamibari/1/1/1",
		"tatamibari/1/1/.",
		"tatamibari/1/5/g",
		"tatamibari/5/1/g",
		"tatamibari/3/3/!2"
	],
	fileios: [
		// Every clue kind, `?`, and both border answer tokens with the cross.
		"pzprv3.3/tatamibari/3/3/a b c /- b . /a . . /1 0 /2 -1 /0 0 /1 0 2 /0 0 1 /",
		// 2x2 with `?` in the corner and a border-answer layer without cells.
		"pzprv3.3/tatamibari/2/2/- . /. . /1 /-1 /1 2 /",
		// 1x1 board: a single cell and no border layer.
		"pzprv3/tatamibari/1/1/c /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, -2, 1, 2, 3]);
		});
		p.board.border.each((border) => {
			border.qans = r.rnd() < 0.3 ? 1 : 0;
			border.qsub = r.rnd() < 0.2 ? 1 : 0;
		});
	}
};

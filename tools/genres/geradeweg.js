// Fixture inputs for pzprjs pid "geradeweg". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "geradeweg",
	dataset: "Geradeweg",
	urls: [
		// Real puzzles: pzprjs test/script/geradeweg.js and dataset samples.
		"geradeweg/5/5/l211i2h3i4k",
		"geradeweg/6/6/g3g3u3h3i3o",
		"geradeweg/10/10/r2j5i4l3k12m3h3l4h4m12k2l4i4j2r",
		"geradeweg/6/6/m2g2j1g1h2g2g2h1j2h1g",
		"https://puzz.link/p?geradeweg/5/5/l211i2h3i4k",
		"https://puzz.link/p?geradeweg_edit/5/5/l211i2h3i4k",
		"http://pzv.jp/p.html?type=player&geradeweg/5/5/l211i2h3i4k",
		"geradeweg/f/5/5/l211i2h3i4k",
		"geradeweg/x/5/5/l211i2h3i4k",
		"geradeweg/5/5/l211i2h3i4k/",
		"geradeweg/5/5/l211i2h3i4kzz",
		"geradeweg/5/5/l211",
		"geradeweg/5/5/",
		"geradeweg/5/5",
		"geradeweg/1/1/.",
		"geradeweg/1/1/1",
		"geradeweg/10/1/9",
		"geradeweg/1/10/9",
		"geradeweg/2/2/....",
		"geradeweg/30/1/zzk"
	],
	fileios: [
		// The real 5x5 puzzle, with loop_full and answer lines/crosses.
		"pzprv3/geradeweg/5/5/f/. . . . . /. 2 1 1 . /. . 2 . . /3 . . . 4 /. . . . . /1 0 0 0 /0 1 0 0 /0 0 1 -1 /0 0 0 0 /1 0 0 0 /1 0 0 0 0 /0 1 0 1 0 /0 0 0 0 0 /0 0 1 0 0 /",
		// Question mark, full border layer.
		"pzprv3/geradeweg/2/2/- . /. 3 /1 0 /0 1 /",
		// 1x1: no inner borders, so the border layer is one empty line.
		"pzprv3/geradeweg/1/1/. //"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols - 1, p.board.rows - 1);
		const clues = [-1, -1, -1, -1, -2];
		for (let n = 1; n <= Math.min(maxnum, 6); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
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
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

// Fixture inputs for pzprjs pid "canal". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "canal",
	dataset: "CanalView",
	urls: [
		// Real puzzles: pzprjs test/script/canal.js and dataset samples.
		"canal/6/6/3i3h2z4h5i2",
		"canal/4/4/12r31",
		"canal/7/7/1k1i9q2i5q7i1k./",
		"canal/17/17/r11q33h33m31h13m31h16q42q16u81z14u21q21u43z16u31q31q62h41m54h31m12h15q21r",
		"https://puzz.link/p?canal/6/6/3i3h2z4h5i2",
		"https://puzz.link/p?canal_edit/6/6/3i3h2z4h5i2",
		"http://pzv.jp/p.html?type=player&canal/6/6/3i3h2z4h5i2",
		"canal/x/6/6/3i3h2z4h5i2",
		"canal/6/6/3i3h2z4h5i2/",
		"canal/6/6/3i3h2z4h5i2zz",
		"canal/6/6/3i3",
		"canal/6/6/",
		"canal/6/6",
		"canal/1/1/0",
		"canal/1/1/.",
		"canal/10/1/9",
		"canal/1/10/9",
		"canal/17/17/-20",
		"canal/3/3/zz",
		"canal/30/1/zzk"
	],
	fileios: [
		// Real 6x6 board from pzprjs test/script/canal.js, with every answer mark.
		"pzprv3/canal/6/6/3 . . . 3 . /. 2 . . . . /. . . . . . /. . . . . . /. . . . 4 . /. 5 . . . 2 /# . . . . . /. + . . . . /. . c . . . /. . . # . . /. . . . # . /. . . . . c /",
		// Question mark and the three answer marks.
		"pzprv3/canal/2/2/1 - /. . /# c /+ . /",
		"pzprv3/canal/1/1/0 /# /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols + p.board.rows - 2;
		const clues = [-1, -1, -1, -1, -2];
		for (let n = 0; n <= Math.min(maxnum, 6); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			const x = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			cell.qcmp = 0;
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.3) {
				cell.qsub = 1;
			} else if (x < 0.4) {
				cell.qcmp = 1;
			}
		});
	}
};

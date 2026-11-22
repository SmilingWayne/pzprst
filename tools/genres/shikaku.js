// Fixture inputs for pzprjs pid "shikaku". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "shikaku",
	dataset: "Shikaku",
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"shikaku/10/10/g2j43h3h4k6g2k4l3h4g2g62h2n2h34g2g4h2l6k2g2k8h6h46j2g",
		"shikaku/10/10/4l4j3h6i2h3h4h6g4o6h6g2g4j2g4g4h3o3g4h4h4h4i2h2j6l4",
		"shikaku/10/10/2h3h6j6k2k23i3g4i4j34g4h4g2j4g2h2g24j4i2g3i42k6k3j4h3h3",
		"https://puzz.link/p?shikaku/10/10/g2j43h3h4k6g2k4l3h4g2g62h2n2h34g2g4h2l6k2g2k8h6h46j2g",
		"http://pzv.jp/p.html?type=player&shikaku/10/10/g2j43h3h4k6g2k4l3h4g2g62h2n2h34g2g4h2l6k2g2k8h6h46j2g",
		"shikaku/10/10/g2j43h3h4k6g2k4l3h4g2g62h2n2h34g2g4h2l6k2g2k8h6h46j2g/",
		"shikaku/4/4/1zz",
		"shikaku/10/10/g2j43h3h4k6g2k4l3h4g2g6",
		"shikaku/4/4/.",
		"shikaku/4/4/+3e7",
		"shikaku/2/2/-ff",
		"shikaku/1/1/0",
		"shikaku/1/5/1",
		"shikaku/5/1/-19",
		"shikaku/4/4",
		"shikaku/3/3/z1",
		"shikaku/4/4/1G",
		"shikaku/x/4/4/1",
		"shikaku/f/4/4/1"
	],
	fileios: [
		// 5x5 clues with answer borders and crosses.
		"pzprv3/shikaku/5/5/2 . 4 . 6 /4 . 2 . 6 /. 9 . . . /. . . . . /. . . . . /1 0 1 0 /0 -1 0 0 /1 0 1 0 /0 0 0 0 /1 0 1 0 /2 0 0 0 0 /0 0 -1 0 0 /0 0 0 0 0 /0 0 0 0 0 /",
		// Question mark, the largest clue, two-digit clues and every answer token.
		"pzprv3/shikaku/2/3/. - 999 /16 1 . /2 1 /-1 0 /1 2 -1 /",
		// Empty board: no clue and no answer data.
		"pzprv3/shikaku/3/3/. . . /. . . /. . . /0 0 /0 0 /0 0 /0 0 0 /0 0 0 /"
	],
	randomize(p, r) {
		const clues = [-1, -1, -1, -1, -1, 1, 2, 3, 15, 16, 255, 256, 999, -2];
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qans = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.qans = 1;
			} else if (x < 0.3) {
				border.qsub = 1;
			} else if (x < 0.4) {
				border.qans = 1;
				border.qsub = 1;
			}
		});
	}
};

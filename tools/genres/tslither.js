// Fixture inputs for pzprjs pid "tslither". See tools/gen_fixtures.js for the format.
// Touch Slitherlink shares variety/vslither.js with "vslither". There is no
// Touch Slitherlink folder under ../puzzlekit-dataset/assets/data, so the last
// three entries are native-derived links from ../.pzprst-samples.js.
"use strict";

module.exports = {
	pid: "tslither",
	dataset: null,
	urls: [
		"tslither/6/6/g7c6bh7b6ch79cj",
		"https://puzz.link/p?tslither/6/6/g7c6bh7b6ch79cj",
		"http://pzv.jp/p.html?type=player&tslither/6/6/g7c6bh7b6ch79cj",
		"tslither_edit/6/6/g7c6bh7b6ch79cj",
		"tslither/f/6/6/g7c6bh7b6ch79cj",
		"tslither/xf/6/6/g7c6bh7b6ch79cj",
		"tslither/4/4/....",
		"tslither/4/4/4444",
		"tslither/4/4/0123",
		"tslither/2/2/44",
		"tslither/1/1/0",
		"tslither/1/4/0",
		"tslither/4/1/0",
		"tslither/4/4/01",
		"tslither/4/4/01zz",
		"tslither/4/4/",
		"tslither/4/4",
		"tslither/4/4/A",
		"tslither/3/3/e",
		"tslither/6/6/g7c6bh7b6ch79cjzz",
		"https://puzz.link/p?tslither/6/11/2cgbg3d1bgdg2c3d2cgcg3d2cgbg2c2c2cgc",
		"https://puzz.link/p?tslither/9/9/gcibg86cg8687bk67888cgdgcbgbi6dp"
	],
	fileios: [
		// 2x2 with clues, `?`, backgrounds, frame and inner lines, and cross marks.
		"pzprv3/tslither/2/2/1 . /- 4 /0 1 /0 2 /1 0 0 /0 0 0 /0 -1 /1 0 /0 0 /1 0 2 /0 1 0 /2 0 1 /",
		// 3x3 with the slither_full flag, a `?`, every background value and marks.
		"pzprv3/tslither/3/3/f/0 4 . /- . 2 /. 4 1 /2 0 1 /0 2 0 /0 0 1 /0 0 1 0 /0 -1 0 0 /1 0 0 0 /0 0 1 /0 0 0 /1 0 0 /0 0 1 /1 0 0 0 /0 1 0 0 /0 0 0 1 /0 0 1 0 /"
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
		p.board.cross.each((cross) => {
			cross.qsub = r.pick([0, 0, 0, 1, 2]);
		});
		p.setConfig("slither_full", r.rnd() < 0.3);
	}
};

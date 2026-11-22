// Fixture inputs for pzprjs pid "vslither". See tools/gen_fixtures.js for the format.
// Vertex Slitherlink shares variety/vslither.js with "tslither". There is no
// Vertex Slitherlink folder under ../puzzlekit-dataset/assets/data, so the last
// three entries are native-derived links from ../.pzprst-samples.js.
"use strict";

module.exports = {
	pid: "vslither",
	dataset: null,
	urls: [
		"vslither/6/6/k26chcg268e891cgc",
		"https://puzz.link/p?vslither/6/6/k26chcg268e891cgc",
		"http://pzv.jp/p.html?type=player&vslither/6/6/k26chcg268e891cgc",
		"vslither_edit/6/6/k26chcg268e891cgc",
		"vslither/f/6/6/k26chcg268e891cgc",
		"vslither/xf/6/6/k26chcg268e891cgc",
		"vslither/4/4/....",
		"vslither/4/4/4444",
		"vslither/4/4/0123",
		"vslither/2/2/44",
		"vslither/1/1/0",
		"vslither/1/4/0",
		"vslither/4/1/0",
		"vslither/4/4/01",
		"vslither/4/4/01zz",
		"vslither/4/4/",
		"vslither/4/4",
		"vslither/4/4/A",
		"vslither/3/3/e",
		"vslither/6/6/k26chcg268e891cgczz",
		"https://puzz.link/p?vslither/6/6/338833ddg8d3dkd8d",
		"https://puzz.link/p?vslither/7/7/766ck776ek789ek976b"
	],
	fileios: [
		// 2x2 with clues, `?`, backgrounds, frame and inner lines, and cross marks.
		"pzprv3/vslither/2/2/1 . /- 4 /0 1 /0 2 /1 0 0 /0 0 0 /0 -1 /1 0 /0 0 /1 0 2 /0 1 0 /2 0 1 /",
		// 3x3 with the slither_full flag, a `?`, every background value and marks.
		"pzprv3/vslither/3/3/f/0 4 . /- . 2 /. 4 1 /2 0 1 /0 2 0 /0 0 1 /0 0 1 0 /0 -1 0 0 /1 0 0 0 /0 0 1 /0 0 0 /1 0 0 /0 0 1 /1 0 0 0 /0 1 0 0 /0 0 0 1 /0 0 1 0 /"
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

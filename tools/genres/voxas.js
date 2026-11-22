// Fixture inputs for pzprjs pid "voxas". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "voxas",
	dataset: null,
	urls: [
		"voxas/5/5/ja1a0bbi10ap00c",
		"voxas/8/8/nbgcdldjcldcgdudg2dzj3bgbm",
		"voxas/10/10/jdt0agcgahagdh2ak2ck3agagaqahan01d017ax01c00cz13c135am",
		"https://puzz.link/p?voxas/5/5/ja1a0bbi10ap00c",
		"http://pzv.jp/p.html?type=player&voxas/5/5/ja1a0bbi10ap00c",
		"voxas/5/5/ja1a",
		"voxas/5/5/ja1a0bbi10ap00czz",
		"voxas/3/3/0123",
		"voxas/2/2/3333",
		"voxas/1/1/",
		"voxas/2/2/",
		"voxas/3/3/z",
		"voxas/3/3/g",
		"voxas/10/10/z",
		"voxas/t/3/3/0123",
		"voxas/t/5/5/ja1a0bbi10ap00c",
		"voxas/t/1/1/"
	],
	fileios: [
		"pzprv3/voxas/3/3/t/1 2 /3 4 /-5 -0 /0 1 2 /-0 5 -0 /",
		"pzprv3/voxas/2/2/5 /-0 /4 -0 /",
		"pzprv3/voxas/1/1/"
	],
	randomize(p, r) {
		p.board.border.each((border) => {
			border.ques = r.pick([0, 0, 0, 0, 1, 2, 3, 4, 5]);
			border.qans = r.pick([0, 0, 0, 1]);
			border.qsub = r.pick([0, 0, 0, 1]);
		});
		p.setConfig("voxas_tatami", r.rnd() < 0.3);
	}
};

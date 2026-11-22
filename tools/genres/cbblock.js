// Fixture inputs for pzprjs pid "cbblock". See tools/gen_fixtures.js for the format.
// "dbchoco", "nikoji" and "mirrorbk" share pzprjs's variety/cbblock.js but
// override Encode/FileIO, so they are separate pids and are not covered here.
"use strict";

module.exports = {
	pid: "cbblock",
	dataset: null,
	urls: [
		"cbblock/8/8/isll14q8g92e07hp823sico",
		"https://puzz.link/p?cbblock/12/11/qerdb5m9dcbr661lrlqsrbvqtpt7flkn6jg2ped5d8nfnj3lg",
		"https://puzz.link/p?cbblock/8/8/kqh5p19qipqqlf82nlu82rg",
		"http://pzv.jp/p.html?cbblock/8/8/isll14q8g92e07hp823sico",
		"cbblock_edit/4/4/0",
		"http://pzv.jp/p.html?type=player&cbblock/4/4/0",
		"cbblock/4/4/0",
		"cbblock/4/4/v",
		"cbblock/4/4/vvvvv",
		"cbblock/2/2/u",
		"cbblock/3/3/123",
		"cbblock/4/4/..",
		"cbblock/4/4/z",
		"cbblock/8/8/isll14q8g92e07hp823sicozz",
		"cbblock/1/1",
		"cbblock/1/4/0",
		"cbblock/4/1/0",
		"cbblock/9/9/-50",
		"cbblock/16/16/0000000000000000000000000000000000000"
	],
	fileios: [
		"pzprv3/cbblock/2/2/3 /-1 /1 -2 /",
		"pzprv3/cbblock/3/3/0 3 /-1 1 /-2 2 /0 -1 3 /1 -2 0 /",
		"pzprv3/cbblock/1/1/"
	],
	randomize(p, r) {
		p.board.border.each((border) => {
			border.ques = 0;
			border.qans = 0;
			border.qsub = 0;
			const x = r.rnd();
			if (x < 0.14) {
				border.qans = 1;
			} else if (x < 0.28) {
				border.qans = 1;
				border.qsub = 1;
			} else if (x < 0.42) {
				border.qsub = 1;
			} else if (x < 0.56) {
				border.ques = 1;
				border.qsub = 1;
			} else if (x < 0.7) {
				border.ques = 1;
			}
		});
	}
};

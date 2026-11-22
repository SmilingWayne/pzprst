// Fixture inputs for pzprjs pid "koburin". See tools/gen_fixtures.js for the format.
// "yajilin", "yajilin-regions", "lixloop", "retsurin" and "heyajirimisaki"
// share pzprjs's variety/yajilin.js but override Encode/FileIO, so they are
// separate pids and are not covered here.
"use strict";

module.exports = {
	pid: "koburin",
	dataset: "Koburin",
	urls: [
		"koburin/10/10/lchczgbgcp3bqbu.l.m",
		"https://puzz.link/p?koburin/10/10/x6c5czjbhbjbkbgbzg",
		"https://puzz.link/p?koburin/10/10/cz6cqcgdzgczbgbi",
		"http://pzv.jp/p.html?koburin/10/10/lchczgbgcp3bqbu.l.m",
		"koburin_edit/4/4/0",
		"http://pzv.jp/p.html?type=player&koburin/4/4/0",
		"koburin/10/10/lchczgbgcp3bqbu.l.m",
		"koburin/o/10/10/lchczgbgcp3bqbu.l.m",
		"koburin/m/10/10/lchczgbgcp3bqbu.l.m",
		"koburin/b/10/10/lchczgbgcp3bqbu.l.m",
		"koburin/bmo/10/10/lchczgbgcp3bqbu.l.m",
		"koburin/omb/10/10/lchczgbgcp3bqbu.l.m",
		"koburin/4/4/lch",
		"koburin/4/4/lchczgbgcp3bqbu.l.m",
		"koburin/4/4/lchczgbgcp3bqbu.l.mzz",
		"koburin/4/4/4444444444444444",
		"koburin/4/4/....",
		"koburin/4/4/",
		"koburin/1/1/0",
		"koburin/1/4/0",
		"koburin/4/1/0",
		"koburin/9/9/012345678",
		"koburin/9/9/-50",
		"koburin/16/16/0000000000000000000000000000"
	],
	fileios: [
		"pzprv3/koburin/2/2/o/m/b/0 - /2 . /# + /. # /1 /0 /1 -1 /",
		"pzprv3/koburin/3/3/0 1 2 /3 4 0 /. 1 2 /# + . /. # + /+ . # /0 1 /-1 0 /1 0 /0 -1 1 /1 0 1 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -2, 0, 1, 2, 3, 4]);
			cell.qans = r.rnd() < 0.2 ? 1 : 0;
			cell.qsub = r.rnd() < 0.2 ? 1 : 0;
		});
		p.board.border.each((border) => {
			border.line = 0;
			border.qsub = 0;
			const x = r.rnd();
			if (x < 0.3) {
				border.line = 1;
			} else if (x < 0.45) {
				border.qsub = 2;
			}
		});
		p.setConfig("yajilin_out", r.rnd() < 0.3);
		p.setConfig("koburin_minesweeper", r.rnd() < 0.3);
		p.setConfig("disptype_yajilin", r.rnd() < 0.3 ? 2 : 1);
	}
};

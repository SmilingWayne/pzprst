// Fixture inputs for pzprjs pid "pipelink". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "pipelink",
	dataset: "Pipelink",
	urls: [
		"https://puzz.link/p?pipelink/10/10/sbhgjeiekdmgkeiciehflglbkdhbhehehdkendv",
		"https://puzz.link/p?pipelink/10/10/igrdhdkbmgjgidnfjbigkcldhciglgkgjehbhbjbq",
		"http://pzv.jp/p.html?pipelink/10/10/jgigrglcidighfjgkbieieqghgifqfjgjcifhfs",
		"pipelink/10/10/jgigrglcidighfjgkbieieqghgifqfjgjcifhfs",
		"pipelink/3/3/a",
		"pipelink/3/3/g",
		"pipelink/3/3/...",
		"pipelink/3/3/...m",
		"pipelink/2/2/.",
		"pipelink/3/3/azzz",
		"pipelink/3/3/",
		"pipelink/3/3/h",
		"pipelink/1/1/",
		"pipelink/1/1/z",
		"pipelink/1/4/",
		"pipelink/4/1/",
		"pipelink/2/3/abc",
		"pipelink/2/2/zz",
		"pipelink/2/2/.../",
		"http://pzv.jp/p.html?type=player&pipelink/3/3/a",
		"pipelink/2/2/...."
	],
	fileios: [
		"pzprv3/pipelink/2/2/pipe/a . /. . /1 /-1 /0 1 /",
		"pzprv3/pipelink/2/2/pipe/- . /. c /0 /0 /0 0 /",
		"pzprv3/pipelink/1/1/pipe/- //"
	],
	randomize(p, r) {
		const maxnum = (p.board.cols * p.board.rows) / 4 | 0;
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 0, 0, -2, 11, 12, 13, 14, 15, 16, 17]);
			cell.qnum = maxnum > 0 && r.rnd() < 0.2 ? 1 + Math.floor(r.rnd() * maxnum) : -1;
		});
		p.board.border.each((border) => {
			if (r.rnd() < 0.2) {
				border.line = 1;
			} else if (r.rnd() < 0.15) {
				border.qsub = 2;
			}
		});
	}
};

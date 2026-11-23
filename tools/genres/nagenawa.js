"use strict";
module.exports = {
	pid: "nagenawa", dataset: null,
	urls: ["https://puzz.link/p?nagenawa/8/8/aikl79aqislg03vh047v904g3421456504045500", "https://puzz.link/p?nagenawa/10/10/l9ail5ib4m9d2qllba0007fo07ho1vcf000016146145552262316", "nagenawa/4/4/", "nagenawa/1/1/", "nagenawa/1/5/", "nagenawa/5/1/"],
	fileios: ["pzprv3/nagenawa/2/2/2/0 0 /1 1 /1 . /2 . /1 /-1 /0 1 /0 1 /1 0 /"],
	randomize(p, r) {
		p.board.border.each(b => b.setQues(r.rnd() < 0.35 ? 1 : 0));
		p.board.roommgr.rebuild();
		p.board.roommgr.components.forEach(room => { room.top.qnum = r.pick([-1, -1, -2, 0, 1, 2]); });
		p.board.border.each(b => { const v = r.rnd(); if (v < 0.2) b.line = 1; else if (v < 0.35) b.qsub = 2; });
		p.board.cell.each(c => { if (r.rnd() < 0.25) c.qsub = 1; });
	}
};

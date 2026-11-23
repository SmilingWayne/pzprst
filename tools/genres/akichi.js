"use strict";
module.exports = {
	pid: "akichi", dataset: null,
	urls: ["https://puzz.link/p?akichi/5/6/248h2u0uu033333", "https://puzz.link/p?akichi/11/10/aaaaaaaaaaaaaa28282800336ec00001of3oe3ou3g44g343443344h", "akichi/4/4/", "akichi/1/1/", "akichi/1/5/", "akichi/5/1/", "akichi/x/2/2/001"],
	fileios: ["pzprv3/akichi/2/3/3/0 0 1 /2 2 1 /2 . 1 /. . . /# . + /. # . /", "pzprv3.1/akichi/2/2/x/0 /0 /0 0 /. 2 /. . /. . /. . /"],
	randomize(p, r) {
		p.setConfig("akichi_maximum", r.rnd() < 0.5);
		p.board.border.each(b => b.setQues(r.rnd() < 0.35 ? 1 : 0));
		p.board.roommgr.rebuild();
		p.board.roommgr.components.forEach(room => { room.top.qnum = r.pick([-1, -1, -2, 0, 1, 2]); });
		p.board.cell.each(c => { const v = r.rnd(); if (v < 0.2) c.qans = 1; else if (v < 0.35) c.qsub = 1; });
		p.board.border.each(b => { const v = r.rnd(); if (v < 0.15) b.line = 1; else if (v < 0.25) b.qsub = 2; });
	}
};

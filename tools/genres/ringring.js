"use strict";
module.exports = {
	pid: "ringring", dataset: null,
	urls: [
		"https://puzz.link/p?ringring/15/11/00533637i855a5.06e5300",
		"https://puzz.link/p?ringring/13/10/534c59g30jv2",
		"https://puzz.link/p?ringring/15/11/0052306234i85j26gi0d00c",
		"ringring/1/1/0", "ringring/1/1/", "ringring/9/9/.z0", "ringring/4/4/123z"
	],
	fileios: ["pzprv3/ringring/2/2/1 0 /0 1 /1 /-1 /0 1 /"],
	randomize(p, r) {
		p.board.cell.each(c => { if (r.rnd() < 0.3) c.ques = 1; });
		p.board.border.each(b => { const v = r.rnd(); if (v < 0.2) b.line = 1; else if (v < 0.4) b.qsub = 2; });
	}
};

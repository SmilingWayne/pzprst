"use strict";
module.exports = {
	pid: "usoone", dataset: null,
	urls: ["https://puzz.link/p?usoone/10/10/488gh12204080204080000003v0000vv0000hcgckcj3di8bhcgchcjdgc7cn52c8cdo6ag", "https://puzz.link/p?usoone/10/10/h4pl5mt2p8bkb29h94bl4i98me4nfq1ulic4ibgcn6b33dcbmdgd2adid788akco6b70ch", "usoone/4/4/", "usoone/1/1/", "usoone/1/5/", "usoone/5/1/"],
	fileios: ["pzprv3/usoone/2/3/3/0 0 1 /2 2 1 /2 . 1 /. 3 . /1 +o +x /o x + /"],
	randomize(p, r) {
		p.board.border.each(b => { b.setQues(r.rnd() < 0.35 ? 1 : 0); const v = r.rnd(); if (v < 0.15) b.line = 1; else if (v < 0.25) b.qsub = 2; });
		p.board.cell.each(c => {
			c.qnum = r.pick([-1, -1, -1, -2, 0, 1, 2, 3, 4]);
			if (r.rnd() < 0.2) c.qans = 1;
			else { if (r.rnd() < 0.3) c.qsub = 1; c.qcmp = r.pick([0, 0, 0, 1, 2]); }
		});
	}
};

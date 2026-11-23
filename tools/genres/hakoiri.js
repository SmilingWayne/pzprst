"use strict";
module.exports = {
    pid: "hakoiri",
    dataset: "Hakoiri",
    urls: ["https://puzz.link/p?hakoiri/6/6/lntn7724lhg0q3f2d2a1d", "https://puzz.link/p?hakoiri/8/8/5hdbd0h54kh001sauk86jto0a3i3c1a2c1o2a2j1c2a1a2e", "https://puzz.link/p?hakoiri/8/8/51a6peqtimc00o8fqsh8iomga3b1j1e2b12c3h2e1l2a3c3", "hakoiri/4/4/", "hakoiri/1/1/", "hakoiri/1/5/", "hakoiri/5/1/"],
    fileios: [],
    randomize(p, r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { c.qnum = r.pick([-1, -1, -1, -2, 0, 1, 2, 3]); });
        p.board.cell.each((c) => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });
        p.board.cell.each((c) => { c.anum = r.pick([-1,-1,-1,1,2]); if (c.anum === -1 && r.rnd() < 0.3) c.snum = [r.pick([-1, 1]), -1, r.pick([-1, 2]), -1]; });
    }
};

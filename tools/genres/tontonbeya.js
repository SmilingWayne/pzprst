"use strict";
module.exports = {
    pid: "tontonbeya",
    dataset: null,
    urls: ["https://puzz.link/p?tontonbeya/8/8/1acn6k4hkc10o8s1dubrghs0j3c3332c1t3c1232c1j", "https://puzz.link/p?tontonbeya/12/12/592k9ckiab52516gkl2aja9h4p000n7g63083tt040ens009otg010d3b2e2h1e3b1p3a2f3a2x1a2f3a1p3b1e2h3e3b1d", "https://puzz.link/p?tontonbeya/8/8/10822gk51080000fg0073s001g2g3f1k2k3g23f1", "tontonbeya/4/4/", "tontonbeya/1/1/", "tontonbeya/1/5/", "tontonbeya/5/1/"],
    fileios: [],
    randomize(p, r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { c.qnum = r.pick([-1, -1, -1, -2, 0, 1, 2, 3]); });
        p.board.cell.each((c) => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });
        p.board.cell.each((c) => { c.anum = r.pick([-1,-1,-1,1,2]); if (c.anum === -1 && r.rnd() < 0.3) c.snum = [r.pick([-1, 1]), -1, r.pick([-1, 2]), -1]; });
    }
};

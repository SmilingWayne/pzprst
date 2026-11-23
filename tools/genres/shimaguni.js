"use strict";
module.exports = {
    pid: "shimaguni",
    dataset: "Shimaguni",
    urls: ["https://puzz.link/p?shimaguni/10/10/884aha5h85jshipgi17vjqfmudt1buhlti1ui2h34g3m", "https://puzz.link/p?shimaguni/10/10/i4qqkjba4cogi8969cb4ko84br887gvufme04124g3h5k1", "https://puzz.link/p?shimaguni/10/10/k3b6ld9hnqj966cc1g3e0e34us9a7g300fev1i2h3i4i", "shimaguni/4/4/", "shimaguni/1/1/", "shimaguni/1/5/", "shimaguni/5/1/"],
    fileios: [],
    randomize(p, r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.roommgr.rebuild();
        p.board.roommgr.components.forEach((room) => { room.top.qnum = r.pick([-1, -1, -2, 0, 1, 2]); });
        p.board.cell.each((c) => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });
        p.board.cell.each((c) => { if (r.rnd() < 0.3) c.snum = [r.pick([-1,1]), -1, r.pick([-1,2]), -1]; });
    }
};

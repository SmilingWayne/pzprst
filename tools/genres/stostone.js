"use strict";
module.exports = {
    pid: "stostone",
    dataset: null,
    urls: ["https://puzz.link/p?stostone/9/10/8910002i98000gi200fg01u0g7o00v000h2g6", "https://puzz.link/p?stostone/10/10/10ecso80g10204080g2200220000vv00000099j", "https://puzz.link/p?stostone/10/10/1m7cuhlb56acldas4ou0u110gfc000t00f604g44g3h34334g", "stostone/4/4/", "stostone/1/1/", "stostone/1/5/", "stostone/5/1/"],
    fileios: [],
    randomize(p, r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.roommgr.rebuild();
        p.board.roommgr.components.forEach((room) => { room.top.qnum = r.pick([-1, -1, -2, 0, 1, 2]); });
        p.board.cell.each((c) => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });
    }
};

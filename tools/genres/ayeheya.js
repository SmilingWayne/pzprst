"use strict";
module.exports = {
    pid: "ayeheya",
    dataset: null,
    urls: ["https://puzz.link/p?ayeheya/10/10/14284g90i14488gh1200v0000s00vv00u000444h4i", "https://puzz.link/p?ayeheya/13/12/95298ia4ih4k95298k95298ia4ah2003003ge1o00oo00vvg01g000v000k2444224l", "https://puzz.link/p?ayeheya/16/13/930930920920920cgg4gg4gg4gg4gg489489489000c0vo7g0000fvvs00007o000001vvv0000000254l5h45g453g", "ayeheya/4/4/", "ayeheya/1/1/", "ayeheya/1/5/", "ayeheya/5/1/"],
    fileios: [],
    randomize(p, r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.roommgr.rebuild();
        p.board.roommgr.components.forEach((room) => { room.top.qnum = r.pick([-1, -1, -2, 0, 1, 2]); });
        p.board.cell.each((c) => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });
    }
};

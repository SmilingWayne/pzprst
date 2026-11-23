"use strict";
module.exports = { pid: "maxi", dataset: null, urls: ["https://puzz.link/p?maxi/6/6/i6r7j3cab3gs333333", "https://puzz.link/p?maxi/8/8/n5paajmunlg00sc60vss00vg22g1g31h1g1i1g", "https://puzz.link/p?maxi/6/6/444444000000a6", "maxi/4/4/", "maxi/1/1/", "maxi/1/5/", "maxi/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.roommgr.rebuild();
        p.board.roommgr.components.forEach((room) => { room.top.qnum = r.pick([-1,-1,-2,0,1,2]); });
        p.board.border.each((e) => { const n=r.rnd(); e.line=n<0.2?1:0; e.qsub=n>=0.2&&n<0.35?2:0; });
        p.board.cell.each((c) => { c.qsub = r.pick([0,0,0,1,2]); });
    }
};

"use strict";
module.exports = { pid: "detour", dataset: "Detour", urls: ["https://puzz.link/p?detour/8/8/03thrmtors007hvdn6erfou0g10100g111010001g110110", "https://puzz.link/p?detour/8/8/51baqm0ldaag0vg00vrv03g02310j112g1", "https://puzz.link/p?detour/6/6/4lntl4f15kgu3456g1g2", "detour/4/4/", "detour/1/1/", "detour/1/5/", "detour/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.roommgr.rebuild();
        p.board.roommgr.components.forEach((room) => { room.top.qnum = r.pick([-1,-1,-2,0,1,2]); });
        p.board.border.each((e) => { const n=r.rnd(); e.line=n<0.2?1:0; e.qsub=n>=0.2&&n<0.35?2:0; });
        p.board.cell.each((c) => { c.qsub = r.pick([0,0,0,1,2]); });
    }
};

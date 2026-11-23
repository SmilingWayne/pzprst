"use strict";
module.exports = { pid: "onsen", dataset: null, urls: ["https://puzz.link/p?onsen/7/7/2baspqmi0ed9p4sljgr4q.q3r", "https://puzz.link/p?onsen/7/13/8papcdnsvmoqdac8fi4duli518javc8jszx2l3l4zx", "https://puzz.link/p?onsen/10/10/6afsrclpn5bmqnv7kc0j1as41ae4d36hn0bm.n.zzzz.n./", "onsen/4/4/", "onsen/1/1/", "onsen/1/5/", "onsen/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { c.qnum = r.pick([-1,-1,-1,-2,0,1,2,3]); });
        p.board.border.each((e) => { const n=r.rnd(); e.line=n<0.2?1:0; e.qsub=n>=0.2&&n<0.35?2:0; });
    }
};

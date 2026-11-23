"use strict";
module.exports = { pid: "dotchi", dataset: null, urls: ["https://puzz.link/p?dotchi/10/10/5gb0m180g102gd0q1k00000fs0vv07u000003a33k0ij6313123k46j6103c23il6j0390", "https://puzz.link/p?dotchi/6/6/444444007s00933913591661", "https://puzz.link/p?dotchi/8/8/518a0g420g40003vg07v0000078f99c93k9091k99923mi", "dotchi/4/4/", "dotchi/1/1/", "dotchi/1/5/", "dotchi/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { c.qnum = r.pick([-1,-1,-1,1,2]); });
        p.board.border.each((e) => { const n=r.rnd(); e.line=n<0.2?1:0; e.qsub=n>=0.2&&n<0.35?2:0; });
        p.board.cell.each((c) => { c.qsub = r.pick([0,0,0,1,2]); });
    }
};

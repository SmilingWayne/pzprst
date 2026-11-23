"use strict";
module.exports = { pid: "nothing", dataset: null, urls: ["https://puzz.link/p?nothing/15/15/dqnvjfk5emkqmlequvfjvflbanbb9vpjue2od95l9ap7s2venevele9vkot38n75ajqtc5eno0f7csfvubud", "nothing/4/4/", "nothing/1/1/", "nothing/1/5/", "nothing/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.border.each((e) => { const n=r.rnd(); e.line=n<0.2?1:0; e.qsub=n>=0.2&&n<0.35?2:0; });
        p.board.cell.each((c) => { c.qsub = r.pick([0,0,0,1,2]); });
    }
};

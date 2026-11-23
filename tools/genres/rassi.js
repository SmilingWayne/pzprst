"use strict";
module.exports = { pid: "rassi", dataset: null, urls: ["https://puzz.link/p?rassi/10/10/ih5p7geib9v3ujq7i90490o23kr88030rg44", "https://puzz.link/p?rassi/5/4/89104r4", "https://puzz.link/p?rassi/8/8/047lo61ll500foo4o0373ge0000000000g000", "rassi/4/4/", "rassi/1/1/", "rassi/1/5/", "rassi/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { if (r.rnd() < 0.12) c.setQues(7); });
        p.board.border.each((e) => { const n=r.rnd(); e.line=n<0.2?1:0; e.qsub=n>=0.2&&n<0.35?2:0; });
        p.board.cell.each((c) => { c.qsub = r.pick([0,0,0,1,2]); });
    }
};

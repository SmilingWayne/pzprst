"use strict";
module.exports = { pid: "doubleback", dataset: "DoubleBack", urls: ["https://puzz.link/p?doubleback/10/10/a8mhs9ol38l7de9a340o374u4suff28cuv1o", "https://puzz.link/p?doubleback/10/10/mq90i5mh42ptlq0kbdi900vpjp1gufi940i9i90000i90000i90000i9", "https://puzz.link/p?doubleback/6/8/idtvvjc0b9jo1uavo", "doubleback/4/4/", "doubleback/1/1/", "doubleback/1/5/", "doubleback/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { if (r.rnd() < 0.12) c.setQues(7); });
        p.board.border.each((e) => { const n=r.rnd(); e.line=n<0.2?1:0; e.qsub=n>=0.2&&n<0.35?2:0; });
        p.board.cell.each((c) => { c.qsub = r.pick([0,0,0,1,2]); });
    }
};

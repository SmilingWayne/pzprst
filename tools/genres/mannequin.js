"use strict";
module.exports = {
    pid: "mannequin", dataset: null,
    urls: ["https://puzz.link/p?mannequin/10/10/abkt6ij7o7j4nvnu69g4fapb2nsefe4ja3huhag21g2k1h1j1", "mannequin/4/4/", "mannequin/1/1/", "mannequin/1/5/", "mannequin/5/1/"],
    fileios: [],
    randomize(p, r) { p.board.border.each(e => e.setQues(r.rnd() < 0.35 ? 1 : 0)); p.board.roommgr.rebuild(); p.board.roommgr.components.forEach(room => { room.top.qnum = r.pick([-1,-1,-2,0,1,2]); }); p.board.cell.each(c => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; }); p.board.cell.each(c => { if (r.rnd() < 0.2) c.snum = [r.pick([-1,1]),-1,r.pick([-1,2]),-1]; }); }
};

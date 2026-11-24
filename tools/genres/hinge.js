"use strict";
module.exports = {
    pid: "hinge", dataset: null,
    urls: ["https://puzz.link/p?hinge/12/12/4i292994kkaa552ih994ki4i290007vu003u00co0pg03u003vv000g2g23434g3a3g42321g1g", "https://puzz.link/p?hinge/8/8/9295af5a949003u169687s00g1g23241g5g", "https://puzz.link/p?hinge/11/11/4i4i82c6eeeeeec682949400fv0vc71g001gs6v0vu004g222a8g6424g20", "hinge/4/4/", "hinge/1/1/", "hinge/1/5/", "hinge/5/1/"],
    fileios: [],
    randomize(p, r) { p.board.border.each(e => e.setQues(r.rnd() < 0.35 ? 1 : 0)); p.board.roommgr.rebuild(); p.board.roommgr.components.forEach(room => { room.top.qnum = r.pick([-1,-1,-2,0,1,2]); }); p.board.cell.each(c => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });  }
};

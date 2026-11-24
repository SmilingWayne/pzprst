"use strict";
module.exports = {
    pid: "arrowflow", dataset: null,
    urls: ["https://puzz.link/p?arrowflow/7/7/003g00h2i3g300g1k2k3g002g4i1h00g200", "https://puzz.link/p?arrowflow/8/8/h3g06o5g0g8k3k00k2k2g0g3o80g8h", "arrowflow/4/4/", "arrowflow/1/1/", "arrowflow/1/5/", "arrowflow/5/1/"],
    fileios: [],
    randomize(p, r) {  p.board.cell.each(c => { c.qnum = r.pick([-1,-1,-1,-2,1,2,3,4,5,7]); }); p.board.cell.each(c => { if (c.qnum === -1) { c.anum = r.pick([-1,-1,1,2,3,4]); c.qsub = c.anum === -1 && r.rnd() < 0.2 ? 1 : 0; } });  }
};

"use strict";
module.exports = {
    pid: "tilecity", dataset: null,
    urls: ["https://puzz.link/p?tilecity/8/8/nvuvrfdvnvugdjd6tvrmbcr0d2k4zd3k1d", "https://puzz.link/p?tilecity/10/10/fjlvv5eftn7v7fvnsqveavlvuu5vrvmqju7r4h4zzzb4h4", "tilecity/4/4/", "tilecity/1/1/", "tilecity/1/5/", "tilecity/5/1/"],
    fileios: [],
    randomize(p, r) { p.board.border.each(e => e.setQues(r.rnd() < 0.35 ? 1 : 0)); p.board.cell.each(c => { c.qnum = r.pick([-1,-1,-1,-2,0,1,2,3]); }); p.board.cell.each(c => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });  }
};

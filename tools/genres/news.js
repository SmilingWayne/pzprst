"use strict";
module.exports = {
    pid: "news", dataset: null,
    urls: ["https://puzz.link/p?news/6/6/2iqqaae3o3u09..6.7.a.6.7.6.7.5", "news/4/4/", "news/1/1/", "news/1/5/", "news/5/1/", "news/n/3/3/0000g"],
    fileios: [],
    randomize(p, r) { p.board.border.each(e => e.setQues(r.rnd() < 0.35 ? 1 : 0)); p.board.cell.each(c => { c.qnum = r.pick([-1,-1,-1,-2,1,2,3,4]); }); p.board.cell.each(c => { if (c.qnum === -1) { c.anum = r.pick([-1,-1,1,2,3,4]); c.qsub = c.anum === -1 && r.rnd() < 0.2 ? 1 : 0; } }); p.setConfig("tren_new",r.rnd()<0.5); p.board.cell.each(c => { if (c.qnum === -1 && c.anum === -1 && r.rnd()<0.2) c.snum = [1,-1,2,-1]; }); }
};

"use strict";
module.exports = {
    pid: "toichika2", dataset: null,
    urls: ["https://puzz.link/p?toichika2/7/6/afg00v50l2nvt8l2g4g3h1", "toichika2/4/4/", "toichika2/1/1/", "toichika2/1/5/", "toichika2/5/1/"],
    fileios: [],
    randomize(p, r) { p.board.border.each(e => e.setQues(r.rnd() < 0.35 ? 1 : 0)); p.board.roommgr.rebuild(); p.board.roommgr.components.forEach(room => { room.top.qnum = r.pick([-1,-1,-2,0,1,2]); }); p.board.cell.each(c => { if (c.qnum === -1) { c.anum = r.pick([-1,-1,1,2,3,4]); c.qsub = c.anum === -1 && r.rnd() < 0.2 ? 1 : 0; } });  }
};

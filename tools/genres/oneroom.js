"use strict";
module.exports = {
    pid: "oneroom", dataset: null,
    urls: ["https://puzz.link/p?oneroom/10/10/58agl080g10204080gs000vv000000000000g2232cc", "https://puzz.link/p?oneroom/6/6/hd28mhkihh95g10g3g0g", "https://puzz.link/p?oneroom/6/6/hd28mhkihh95050g2g01", "oneroom/4/4/", "oneroom/1/1/", "oneroom/1/5/", "oneroom/5/1/"],
    fileios: [],
    randomize(p, r) { p.board.border.each(e => e.setQues(r.rnd() < 0.35 ? 1 : 0)); p.board.roommgr.rebuild(); p.board.roommgr.components.forEach(room => { room.top.qnum = r.pick([-1,-1,-2,0,1,2]); }); p.board.cell.each(c => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });  }
};

"use strict";
module.exports = {
    pid: "roma", dataset: null,
    urls: ["https://puzz.link/p?roma/5/7/96fpi8femdeue4a2a1a4a1c5c1a3a2a2a3e", "https://puzz.link/p?roma/6/6/6tja5pl53vlej3d3d4d4d5e", "https://puzz.link/p?roma/6/6/4lrqbifbtn2ud2a1a3c5j2c1a2a4d", "roma/4/4/", "roma/1/1/", "roma/1/5/", "roma/5/1/"],
    fileios: [],
    randomize(p, r) { p.board.border.each(e => e.setQues(r.rnd() < 0.35 ? 1 : 0)); p.board.cell.each(c => { c.qnum = r.pick([-1,-1,-1,-2,1,2,3,4,5,7]); }); p.board.cell.each(c => { if (c.qnum === -1) { c.anum = r.pick([-1,-1,1,2,3,4]); c.qsub = c.anum === -1 && r.rnd() < 0.2 ? 1 : 0; } });  }
};

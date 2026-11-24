"use strict";
module.exports = {
    pid: "martini", dataset: null,
    urls: ["https://puzz.link/p?martini/10/10/000000000000000000vvvvvvvvvvvvvvvvvv0m0l.j3l.q5g.l.t.i4n.i.p.l.g", "https://puzz.link/p?martini/10/10/47774uftc6nuf4sss4fs6e09n6c6cti0ec7u3h0i0g0i2j.i3l0m.i0v.i.m0l.i2j0i0g0i.h3", "https://puzz.link/p?martini/6/6/aakkaa0fo3vgl.g0g0.r0300n", "martini/4/4/", "martini/1/1/", "martini/1/5/", "martini/5/1/"],
    fileios: [],
    randomize(p, r) { p.board.border.each(e => e.setQues(r.rnd() < 0.35 ? 1 : 0)); p.board.cell.each(c => { c.qnum = r.pick([-1,-1,-1,-2,0,1,2,3]); }); p.board.cell.each(c => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });  }
};

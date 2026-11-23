"use strict";
module.exports = { pid: "lits", dataset: "LITS", urls: ["https://puzz.link/p?lits/18/10/gq22685b584h8e95j9qec6j59gkke75245fh1hcqea87tdun2bmcfg1ke7961t00ea0", "https://puzz.link/p?lits/18/10/30o0tjpe4ini79safgt2a3fb4n6j7100g0udl1i88k11dg7qauk4fgb2okk6t8c4bvo", "https://puzz.link/p?lits/18/10/844dnetacpnkll6jjubd9mnjkeij9am0s4don8albcjhadp74qs4c8pqmis66acvpu8", "lits/4/4/", "lits/1/1/", "lits/1/5/", "lits/5/1/", "lits/d/3/2/aabacc"], fileios: ["pzprv3/lits/2/3/3/0 0 1 /2 2 1 /# . + /. # . /"],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { const n=r.rnd(); c.qans=n<0.2?1:0; c.qsub=n>=0.2&&n<0.35?1:0; });
        p.board.border.each((e) => { const n = r.rnd(); e.line = n < 0.2 ? 1 : 0; e.qsub = n >= 0.2 && n < 0.35 ? 2 : 0; });
    }
};

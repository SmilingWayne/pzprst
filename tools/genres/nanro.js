"use strict";
module.exports = { pid: "nanro", dataset: "Nanro", urls: ["https://puzz.link/p?nanro/10/10/mjmf95pnpblaj9hok64s9v7tou42acn7aqtqg1g2t3h3m2x3h4t4h4g3x1l2h", "https://puzz.link/p?nanro/10/10/4j9d6dbehddamksd22ccn2km4qbr82c43ejgp4j3h4zh4m4m5j3r5l5w", "https://puzz.link/p?nanro/10/10/l5p7bdk393f6vnub144vbo47tvvcu0bg4ef32i2m3t3r2r2h2k3q3i4p3p", "nanro/4/4/", "nanro/1/1/", "nanro/1/5/", "nanro/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { c.qnum = r.pick([-1,-1,-1,-2,0,1,2,3]); });
    }
};

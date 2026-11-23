"use strict";
module.exports = {
    pid: "paintarea",
    dataset: "PaintArea",
    urls: ["https://puzz.link/p?paintarea/10/10/dndvuuvfnvuvbtrvvvfurrfvnrttuvvvvtrab2b1g2i2q3c2d1k3a1a2j2b1h1a1e1b", "https://puzz.link/p?paintarea/10/10/ftvrlvvuvttvvvvruvtr7vvrvuvvvvvupvnla2a3b1p1c2g2c1b2m1d1f2f1c1t", "https://puzz.link/p?paintarea/10/10/vvrvqmvnvnnvqvfvtrrrvvvrfmvfdvuntuuvc2g1f2u2a2a1d1f2c2a2d2c2b1d1h1g1c", "paintarea/4/4/", "paintarea/1/1/", "paintarea/1/5/", "paintarea/5/1/"],
    fileios: ["pzprv3/paintarea/2/3/3/0 0 1 /2 2 1 /2 . 1 /. . . /# . + /. # . /"],
    randomize(p, r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { c.qnum = r.pick([-1, -1, -1, -2, 0, 1, 2, 3]); });
        p.board.cell.each((c) => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });
    }
};

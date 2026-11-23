"use strict";
module.exports = {
    pid: "nurikabe",
    dataset: "Nurikabe",
    urls: ["https://puzz.link/p?nurikabe/10/10/j6ucj3i3h7p3h3p7u3g3zh2g", "https://puzz.link/p?nurikabe/10/10/h5k7t6l5p2h4p2g2q4g7j2v3j2g", "https://puzz.link/p?nurikabe/24/14/g6h4i5l4j5h4m6zq5k4j4j4n5l4r6k4k4k5z5g5q5k5z5r5i3zn5s5r3r4r3h5k2m5n7h4m5q", "nurikabe/4/4/", "nurikabe/1/1/", "nurikabe/1/5/", "nurikabe/5/1/"],
    fileios: [],
    randomize(p, r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { c.qnum = r.pick([-1, -1, -1, -2, 0, 1, 2, 3]); });
        p.board.cell.each((c) => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });
    }
};

"use strict";
module.exports = {
    pid: "aqre",
    dataset: "Aqre",
    urls: ["https://puzz.link/p?aqre/10/10/8po7rnr0000rtrs3j2rm1l0gl4004l10lgdr11g2300226132", "https://puzz.link/p?aqre/10/10/g90mqtn8c62tmbf0i1vu2c0bdktn5mq0a8fv1g11g1202h32h0g0", "https://puzz.link/p?aqre/18/18/aba2qqg6mi2nhodt6jfc57m8qt96l6a1828b1j6ucn7p5bspeseknpl0od86h00o00svvhe3e41s3g8r2gr3v9u0241vvvrufs3gf3soc0m1g21c3o3k3sn000s0g1g1g22g11g2g22g1g2212g1g1112233355g555355g3g3g355", "aqre/4/4/", "aqre/1/1/", "aqre/1/5/", "aqre/5/1/"],
    fileios: [],
    randomize(p, r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.roommgr.rebuild();
        p.board.roommgr.components.forEach((room) => { room.top.qnum = r.pick([-1, -1, -2, 0, 1, 2]); });
        p.board.cell.each((c) => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });
        p.setConfig("aqre_borders", r.rnd() < 0.5);
    }
};

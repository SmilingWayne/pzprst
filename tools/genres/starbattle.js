"use strict";
module.exports = {
    pid: "starbattle", dataset: "StarBattle",
    urls: ["https://puzz.link/p?starbattle/10/10/2/08296a9mskbeq48910v47i0cve8h1q26f21s", "https://puzz.link/p?starbattle/10/10/2/549hl3d7ikr7kgh78g0kog0c1gmm2gfd08ck", "https://puzz.link/p?starbattle/10/10/2/80kgpl6olqfgn5c2g81o47fi38f41q0o70g9", "starbattle/4/4/", "starbattle/1/1/", "starbattle/1/5/", "starbattle/5/1/"],
    fileios: [],
    randomize(p, r) { p.board.border.each(e => e.setQues(r.rnd() < 0.35 ? 1 : 0)); p.board.starCount.count = r.pick([1,2,3]); p.board.cell.each(c => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });  }
};

"use strict";
module.exports = {
    pid: "heyawake",
    dataset: "Heyawake",
    urls: ["https://puzz.link/p?heyawake/10/10/ocgp10m182qll8agtfo47vvo0fufu81u60puj2j33j2g2h32l", "https://puzz.link/p?heyawake/24/14/499a0h55854kmgkk9a2ih54aa4kg98ii154a84kh914i544i8kgi92j294kc94ihg00001vg0fs0vvg6000vg0000vo00e0fvv00001vvg03g03vo3g00fovvv00000023g23g23h5h3454j44h643g03g4g3j1222h3", "https://puzz.link/p?heyawake/24/14/4k850ih4k2a4ik98iaic29a9g96h694q1p5384kkd0ii2k2a2ag989a152584k8l0007s1v7g1s00vg7vu00003vv0fo01u0fo07o07vg1vg0vvs0u000e03vv3o001o224323322g2h22i2g2g2332j331334h34h2g3221g1", "heyawake/4/4/", "heyawake/1/1/", "heyawake/1/5/", "heyawake/5/1/"],
    fileios: ["pzprv3.1/heyawake/2/2/0 /0 /0 0 /. 2 /. . /. . /. . /", "pzprv3/heyawake/2/3/3/0 0 1 /2 2 1 /2 . 1 /. . . /# . + /. # . /"],
    randomize(p, r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.roommgr.rebuild();
        p.board.roommgr.components.forEach((room) => { room.top.qnum = r.pick([-1, -1, -2, 0, 1, 2]); });
        p.board.cell.each((c) => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });
    }
};

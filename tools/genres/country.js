"use strict";
module.exports = { pid: "country", dataset: null, urls: ["https://puzz.link/p?country/17/13/5hk2ubdeemlt591md29534ikibba5alii8k5ho8444071norg0l2hur13v7v270fe1scvik41ruvpuvhk8043g4g43i32g454h534445k3g3m", "https://puzz.link/p?country/17/13/88i729gm5sp1hcks7ai3l98qckmkai5aa2qkldaak83s0vd3k70imv0u47oso1ogbhup329pbs1u1u0001u1h34h2j44g5g35433i4g353j23g", "https://puzz.link/p?country/10/10/1cr9ejcl1i49bd2p56r86fq23he2tgevh40oi2334j4j33", "country/4/4/", "country/1/1/", "country/1/5/", "country/5/1/"], fileios: ["pzprv3.1/country/2/3/country_empty/3/0 0 1 /2 2 1 /2 . 1 /. . . /0 0 /0 0 /0 0 0 /0 0 0 /0 0 0 /"],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.roommgr.rebuild();
        p.board.roommgr.components.forEach((room) => { room.top.qnum = r.pick([-1,-1,-2,0,1,2]); });
        p.board.border.each((e) => { const n=r.rnd(); e.line=n<0.2?1:0; e.qsub=n>=0.2&&n<0.35?2:0; });
        p.setConfig("country_empty", r.rnd() < 0.5);
        p.board.cell.each((c) => { c.qsub = r.pick([0,0,0,1,2]); });
    }
};

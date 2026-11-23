"use strict";
module.exports = { pid: "moonsun", dataset: "MoonSun", urls: ["https://puzz.link/p?moonsun/10/10/4g90i152a4k98i142800003vs000vg0f00000632a66fi00i3f9i77000k6b20092f9a00", "https://puzz.link/p?moonsun/10/10/29m243a445c2p8ih448o60co3774s07o001s0j0932036006360212b300b06l0l019000", "https://puzz.link/p?moonsun/10/10/38a94lfh3364hh16302440r9ek100uq0481n56090j10906160k0131662020590i05209", "moonsun/4/4/", "moonsun/1/1/", "moonsun/1/5/", "moonsun/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { c.qnum = r.pick([-1,-1,-1,1,2]); });
        p.board.border.each((e) => { const n=r.rnd(); e.line=n<0.2?1:0; e.qsub=n>=0.2&&n<0.35?2:0; });
        p.board.cell.each((c) => { c.qsub = r.pick([0,0,0,1,2]); });
    }
};

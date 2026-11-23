"use strict";
module.exports = { pid: "norinori", dataset: "Norinori", urls: ["https://puzz.link/p?norinori/10/10/90i2c76esik8rapah800evmv37d4fsm9dmte", "https://puzz.link/p?norinori/10/10/d1ajnepkj156i6gm0s8o4e2kv071vvenf410", "https://puzz.link/p?norinori/10/10/g4r9d8k41sglntk90cfs2avf7ujvdmlbaofk", "norinori/4/4/", "norinori/1/1/", "norinori/1/5/", "norinori/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.cell.each((c) => { const n=r.rnd(); c.qans=n<0.2?1:0; c.qsub=n>=0.2&&n<0.35?1:0; });
        p.board.border.each((e) => { const n = r.rnd(); e.line = n < 0.2 ? 1 : 0; e.qsub = n >= 0.2 && n < 0.35 ? 2 : 0; });
    }
};

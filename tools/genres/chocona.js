"use strict";
module.exports = {
    pid: "chocona",
    dataset: "Chocona",
    urls: ["https://puzz.link/p?chocona/10/10/1c97cph5nkp5hkd944vamtb25tqie0avb2ss233g3542554333334g", "https://puzz.link/p?chocona/10/10/h25relfq96l9lid9s9fmmoa5faiehl86fs4i333423g434464333g33", "https://puzz.link/p?chocona/10/10/44skivupdbrbdl95gm7upkqs57f0a4ou7kv943233431332243g22343", "chocona/4/4/", "chocona/1/1/", "chocona/1/5/", "chocona/5/1/"],
    fileios: [],
    randomize(p, r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.roommgr.rebuild();
        p.board.roommgr.components.forEach((room) => { room.top.qnum = r.pick([-1, -1, -2, 0, 1, 2]); });
        p.board.cell.each((c) => { const n = r.rnd(); c.qans = n < 0.2 ? 1 : 0; c.qsub = n >= 0.2 && n < 0.35 ? 1 : 0; });
    }
};

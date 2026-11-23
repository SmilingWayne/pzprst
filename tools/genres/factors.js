"use strict";
module.exports = { pid: "factors", dataset: "Factors", urls: ["https://puzz.link/p?factors/9/9/qrf7rml5bev7mqo7nkqvrnvevqpff0c-15-2476-28-e1c-10-1886-3f-14-15+1e0-36-10-48c-23e-124-51-60a4-367-7878-12-36-10-23", "https://puzz.link/p?factors/9/9/bifreur6rrmviegecv7n6nvb7f7pjg+700-18-36fa-2a-1b-48-3f6-28-1e2c+240-1c-2d2-389-30-23e-18-286-30+2f4-5a-48e-14-36", "https://puzz.link/p?factors/9/9/mqrerfiqfmrmqr8dhfp5vltft4vkdg8-146-2af+480+237-18-23-18-69-124-1bc-30-10-28-15-12-28-8c+2374+288-10-2a-158-30f-12-28", "factors/4/4/", "factors/1/1/", "factors/1/5/", "factors/5/1/"], fileios: [],
    randomize(p,r) {
        p.board.border.each((e) => { e.setQues(r.rnd() < 0.35 ? 1 : 0); });
        p.board.roommgr.rebuild();
        p.board.roommgr.components.forEach((room) => { room.top.qnum = r.pick([-1,-1,-2,0,1,2]); });
        p.board.cell.each((c) => { c.anum = r.pick([-1,-1,-1,1,2]); if (c.anum === -1 && r.rnd() < 0.3) c.snum = [r.pick([-1, 1]), -1, r.pick([-1, 2]), -1]; });
    }
};

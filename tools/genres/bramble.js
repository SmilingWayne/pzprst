"use strict";
module.exports = {pid:"bramble",dataset:null,urls:["bramble/5/5/4cql3su032g222g", "bramble/5/5/4cql3su0g2g222g", "bramble/4/4/", "bramble/1/1/", "bramble/1/5/", "bramble/5/1/"],fileios:["pzprv3.1\nbramble\n5\n5\n0 0 1 0 \n0 0 1 1 \n0 0 1 1 \n0 1 0 1 \n0 1 0 1 \n0 0 0 1 1 \n1 1 1 0 0 \n1 1 1 1 0 \n0 0 0 0 0 \n3 . . 2 . \n. . . . 2 \n2 . . . . \n2 . . . . \n. . . . . \n# # + # # \n+ + # + + \n# # + + # \n+ + # # + \n# # + + # \n", "pzprv3.1\nbramble\n5\n5\n0 0 1 0 \n0 0 1 1 \n0 0 1 1 \n0 1 0 1 \n0 1 0 1 \n0 0 0 1 1 \n1 1 1 0 0 \n1 1 1 1 0 \n0 0 0 0 0 \n. . . 2 . \n. . . . 2 \n2 . . . . \n2 . . . . \n. . . . . \n. . + # # \n+ + # + + \n# # + + # \n+ + # # + \n# # + + # \n"],
 randomize(p,r) {
  p.board.border.each(e=>e.setQues(r.rnd()<0.35?1:0));
  p.board.roommgr.rebuild();
  p.board.roommgr.components.forEach(room=>{room.top.qnum=r.pick([-1,-1,-2,0,1,2]);});
  p.board.cell.each(c=>{const n=r.rnd();c.qans=n<0.2?1:0;c.qsub=n>=0.2&&n<0.35?1:0;});
 }
};

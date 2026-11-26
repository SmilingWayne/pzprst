"use strict";
module.exports = {pid:"nuritwin",dataset:null,urls:["nuritwin/6/5/9kcq4ba2iu13j", "nuritwin/6/5/9kcq4ba2iug3j", "nuritwin/4/4/", "nuritwin/1/1/", "nuritwin/1/5/", "nuritwin/5/1/"],fileios:["pzprv3.1\nnuritwin\n5\n6\n0 1 0 0 1 \n1 0 1 0 0 \n0 1 1 0 0 \n1 1 0 1 0 \n0 0 1 0 0 \n0 1 0 1 1 0 \n1 0 1 0 0 0 \n0 1 0 1 0 0 \n1 0 1 1 1 1 \n1 . . . . . \n. . . . . . \n3 . . . . . \n. . . . . . \n. . . . . . \n+ # # # # + \n# + + # + + \n# # # # + + \n# + + # # # \n# + # # + # \n", "pzprv3.1\nnuritwin\n5\n6\n0 1 0 0 1 \n1 0 1 0 0 \n0 1 1 0 0 \n1 1 0 1 0 \n0 0 1 0 0 \n0 1 0 1 1 0 \n1 0 1 0 0 0 \n0 1 0 1 0 0 \n1 0 1 1 1 1 \n. . . . . . \n. . . . . . \n3 . . . . . \n. . . . . . \n. . . . . . \n+ # # # # + \n# + + # + + \n# # # # + + \n# + + # # # \n# + # # + # \n"],
 randomize(p,r) {
  p.board.border.each(e=>e.setQues(r.rnd()<0.35?1:0));
  p.board.roommgr.rebuild();
  p.board.roommgr.components.forEach(room=>{room.top.qnum=r.pick([-1,-1,-2,0,1,2]);});
  p.board.cell.each(c=>{const n=r.rnd();c.qans=n<0.2?1:0;c.qsub=n>=0.2&&n<0.35?1:0;});
 }
};

"use strict";
module.exports = {pid:"tetroctb",dataset:null,urls:["tetroctb/7/5/3at208ekqoe04130g11", "tetroctb/7/5/3at208ekqoe0g130g11", "tetroctb/4/4/", "tetroctb/1/1/", "tetroctb/1/5/", "tetroctb/5/1/"],fileios:["pzprv3.1\ntetroctb\n5\n7\n0 0 0 1 1 0 \n1 0 1 0 1 1 \n1 0 1 0 0 0 \n1 0 0 0 0 0 \n0 0 1 0 0 0 \n0 1 1 1 0 1 0 \n1 0 0 1 1 0 1 \n0 1 1 0 0 0 0 \n1 1 1 0 0 0 0 \n4 . . . . 3 . \n. 0 . 1 . . . \n1 . . . . . . \n. . . . . . . \n1 . . . . . . \n# # + # + # # \n# + + # + # # \n# + + # # + + \n+ # # + + + + \n+ + # # + + + \n", "pzprv3.1\ntetroctb\n5\n7\n0 0 0 1 1 0 \n1 0 1 0 1 1 \n1 0 1 0 0 0 \n1 0 0 0 0 0 \n0 0 1 0 0 0 \n0 1 1 1 0 1 0 \n1 0 0 1 1 0 1 \n0 1 1 0 0 0 0 \n1 1 1 0 0 0 0 \n. . . . . 3 . \n. 0 . 1 . . . \n1 . . . . . . \n. . . . . . . \n1 . . . . . . \n. . + . + # # \n. + + . . . # \n. + + # # + . \n. # # + . + + \n. . . . + + + \n"],
 randomize(p,r) {
  p.board.border.each(e=>e.setQues(r.rnd()<0.35?1:0));
  p.board.roommgr.rebuild();
  p.board.roommgr.components.forEach(room=>{room.top.qnum=r.pick([-1,-1,-2,0,1,2]);});
  p.board.cell.each(c=>{const n=r.rnd();c.qans=n<0.2?1:0;c.qsub=n>=0.2&&n<0.35?1:0;});
 }
};

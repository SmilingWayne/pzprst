"use strict";
module.exports = {
 pid: "yajilin-regions", dataset: null,
 urls: ["https://puzz.link/p?yajilin-regions/10/10/0k5i567c4oi94khg8g1kugfk0b7uk03j1sd011222221311g", "https://puzz.link/p?yajilin-regions/10/10/ccmpnhf3s2q9ld942830284324tm206g5p0022g2g222g2g2", "https://puzz.link/p?yajilin-regions/10/10/l39eoqg2g92ogc66040s3gc250s13657mofg2222g2222", "yajilin-regions/4/4/", "yajilin-regions/1/1/", "yajilin-regions/1/5/", "yajilin-regions/5/1/"],
 fileios: [],
 randomize(p,r) {
  p.setConfig("yajilin_out", r.rnd() < 0.5);
  p.board.border.each(e=>e.setQues(r.rnd()<0.35?1:0));
  p.board.roommgr.rebuild();
  p.board.roommgr.components.forEach(room=>{room.top.qnum=r.pick([-1,-1,-2,0,1,2]);});
  p.board.cell.each(c=>{const n=r.rnd(); c.qans=n<0.2?1:0; c.qsub=n>=0.2&&n<0.35?1:0;});
  p.board.border.each(e=>{const n=r.rnd(); if(n<0.15)e.line=1;else if(n<0.25)e.qsub=2;});
 }
};

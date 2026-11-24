"use strict";
module.exports = {
 pid: "heyablock", dataset: null,
 urls: ["https://puzz.link/p?heyablock/8/8/aikl59aaikl06f637j1jpgpgi102222i22i0", "https://puzz.link/p?heyablock/8/8/04j5rd8a8l60e4610vo169q0342213612", "https://puzz.link/p?heyablock/10/10/11amldaqlh3265caggfu00f000vv00f600fu552233g352g541", "heyablock/4/4/", "heyablock/1/1/", "heyablock/1/5/", "heyablock/5/1/"],
 fileios: [],
 randomize(p,r) {
  
  p.board.border.each(e=>e.setQues(r.rnd()<0.35?1:0));
  p.board.roommgr.rebuild();
  p.board.roommgr.components.forEach(room=>{room.top.qnum=r.pick([-1,-1,-2,0,1,2]);});
  p.board.cell.each(c=>{const n=r.rnd(); c.qans=n<0.2?1:0; c.qsub=n>=0.2&&n<0.35?1:0;});
  
 }
};

"use strict";
module.exports = {
 pid: "cocktail", dataset: null,
 urls: ["https://puzz.link/p?cocktail/10/10/0o1g2l7cssqal5dcoa00vnc4rj3e8gv0cp323g3h2322h32g2", "https://puzz.link/p?cocktail/10/10/2ieku244dranmqip46sm29tg0eegggllag6o324g4h2g432043", "https://puzz.link/p?cocktail/10/10/kg9guadrbf54h4b840gft66drg8jks96sv30g2g232434g34h2g", "cocktail/4/4/", "cocktail/1/1/", "cocktail/1/5/", "cocktail/5/1/"],
 fileios: [],
 randomize(p,r) {
  
  p.board.border.each(e=>e.setQues(r.rnd()<0.35?1:0));
  p.board.roommgr.rebuild();
  p.board.roommgr.components.forEach(room=>{room.top.qnum=r.pick([-1,-1,-2,0,1,2]);});
  p.board.cell.each(c=>{const n=r.rnd(); c.qans=n<0.2?1:0; c.qsub=n>=0.2&&n<0.35?1:0;});
  
 }
};

"use strict";
module.exports = {
 pid: "juosan", dataset: null,
 urls: ["https://puzz.link/p?juosan/10/10/aklad5rb6vdujvfulgvsvve7hrdo17qv50uv21223g2h2324332g23g2g3g423221g221g2125", "https://puzz.link/p?juosan/10/10/rlnbentfqullbqjl7a8c7vvc531gmffsrv0c3g3212422h3g432g3g42g432222g3g2g224141", "https://puzz.link/p?juosan/10/10/1cip2daqmtcqhntforv0vvvu07voo7rvs0fc442413g2h1442223h3232i322g3224g", "juosan/4/4/", "juosan/1/1/", "juosan/1/5/", "juosan/5/1/"],
 fileios: [],
 randomize(p,r) {
  
  p.board.border.each(e=>e.setQues(r.rnd()<0.35?1:0));
  p.board.roommgr.rebuild();
  p.board.roommgr.components.forEach(room=>{room.top.qnum=r.pick([-1,-1,-2,0,1,2]);});
  p.board.cell.each(c=>{c.qans = r.pick([0,0,12,13]);});
  
 }
};

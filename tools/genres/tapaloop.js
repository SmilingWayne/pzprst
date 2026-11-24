"use strict";
module.exports = { pid:"tapaloop", dataset:null,
 urls: ["https://puzz.link/p?tapaloop/11/11/tadhakhaisajs+46jaksarj4sarsadhakhadt", "https://puzz.link/p?tapaloop/6/6/1ga0h2ta0+10n2j2", "https://puzz.link/p?tapaloop/10/10/2m3kasx+5qh+5qj+5jvasj+5khatx+5qk3m2"].concat(["tapaloop/1/1/0", "tapaloop/1/1/.", "tapaloop/4/4/a0a1az+10-10", "tapaloop/4/4/", "tapaloop/1/5/12345"]),
 fileios:["pzprv3/tapaloop/2/3/1,2 -,- 1,1,1,1 /. . . /0 0 /0 0 /0 0 0 /0 1 0 /1 0 1 /"],
 randomize(p,r) {
  const choices=[[],[],[],[-2],[0],[1],[8],[1,2],[-2,5],[1,1,2],[1,-2,-2],[1,1,1,1],[-2,1,1,-2]];
  p.board.cell.each(c=>{c.qnums=r.pick(choices).slice(); if(r.rnd()<0.2)c.qsub=1;});
  p.board.border.each(e=>{const v=r.rnd();if(v<0.15)e.line=1;else if(v<0.25)e.qsub=2;});
 }
};

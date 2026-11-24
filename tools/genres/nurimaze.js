"use strict";
module.exports = { pid:"nurimaze", dataset:null,
 urls: ["https://puzz.link/p?nurimaze/10/10/vvvevvvumvunvvjftvqtnudfqvnrrfvvnvvf7364o4k1a472j3j4g", "https://puzz.link/p?nurimaze/10/10/vvertvvfvvvutvtvttvvrvvv7n7vvnvvuvvn536452649453a3836183636453r364535374c4743745", "https://puzz.link/p?nurimaze/10/10/bvavfvtfmrvvnfvurvvteqnfrvnrvevfmmfu64h4648193h4g3g482h463b"].concat(["nurimaze/4/4/", "nurimaze/1/1/", "nurimaze/1/5/", "nurimaze/5/1/"]),
 fileios:["pzprv3/nurimaze/2/2/2/0 0 /1 1 /s o /t g /# + /. . /1 /-1 /0 1 /"],
 randomize(p,r) {
  p.board.border.each(e=>e.setQues(r.rnd()<0.35?1:0));
  const n=p.board.cell.length; const a=Math.floor(r.rnd()*n); let b=n===1?a:(a+1+Math.floor(r.rnd()*(n-1)))%n;
  p.board.startpos.set(p.board.cell[a]); p.board.goalpos.set(p.board.cell[b]);
  p.board.cell.each((c,i)=>{if(i!==a&&i!==b)c.ques=r.pick([0,0,0,41,42]); const v=r.rnd(); if(v<0.2)c.qans=1;else if(v<0.35)c.qsub=1;});
  p.board.border.each(e=>{const v=r.rnd();if(v<0.15)e.line=1;else if(v<0.25)e.qsub=2;});
 }
};

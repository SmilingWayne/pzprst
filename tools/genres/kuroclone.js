"use strict";
module.exports = { pid:"kuroclone", dataset:null,
 urls: ["https://puzz.link/p?kuroclone/10/10/a0a0ip58kaad9c8902500i01ckp64a5i62v0i34k44v34z14o34k14", "https://puzz.link/p?kuroclone/10/10/94lla4k449l58gi1011a1m80m80k90ic40o2u46p26o36zd36n", "https://puzz.link/p?kuroclone/10/10/2446eib2g2oeg4gggi02ei9b442o2gt41212p24z44g14f24w14c34m"].concat(["kuroclone/4/4/", "kuroclone/1/1/", "kuroclone/1/5/", "kuroclone/5/1/"]),
 fileios:["pzprv3/kuroclone/2/2/2/0 0 /1 1 /1,3 2,- /. 4,2 /# + /. . /"],
 randomize(p,r) {
  p.board.border.each(e=>e.setQues(r.rnd()<0.35?1:0));
  p.board.cell.each(c=>{if(r.rnd()<0.35){c.qnum=r.pick([-2,0,1,2,3,4]);c.qdir=r.pick([0,1,2,3,4]);}else {const n=r.rnd(); c.qans=n<0.2?1:0;c.qsub=n>=0.2&&n<0.35?1:0;}});
 }
};

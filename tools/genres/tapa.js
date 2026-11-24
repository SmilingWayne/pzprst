"use strict";
module.exports = { pid:"tapa", dataset:null,
 urls: ["https://puzz.link/p?tapa/10/10/qalibmhaliaezh3halibluagga9rafia9r", "https://puzz.link/p?tapa/10/10/ga9ibls2hafs2kaeraehafta7l2malq", "https://puzz.link/p?tapa/10/10/qalibqhagiaezja7h2ibqta9pa7i6h3r"].concat(["tapa/1/1/0", "tapa/1/1/.", "tapa/1/1/9", "tapa/4/4/a0a1azb0c0d0", "tapa/4/4/", "tapa/1/5/12345"]),
 fileios:["pzprv3/tapa/2/3/1,2 -,- 1,1,1,1 /# + . /0 0 /0 0 /0 0 0 /"],
 randomize(p,r) {
  const choices=[[],[],[],[-2],[0],[1],[8],[1,2],[-2,5],[1,1,2],[1,-2,-2],[1,1,1,1],[-2,1,1,-2]];
  p.board.cell.each(c=>{c.qnums=r.pick(choices).slice(); if(!c.qnums.length){const v=r.rnd(); if(v<0.25)c.qans=1;else if(v<0.4)c.qsub=1;}});
  p.board.border.each(e=>{const v=r.rnd();if(v<0.15)e.line=1;else if(v<0.25)e.qsub=2;});
 }
};

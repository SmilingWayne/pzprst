// Fixture inputs for pzprjs pid "simpleloop". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "simpleloop",
	dataset: "SimpleLoop",
	urls: [
		"simpleloop/4/4",
		"simpleloop/4/4/",
		"simpleloop/4/4//",
		"simpleloop/4/4/1",
		"simpleloop/4/4/1000",
		"https://puzz.link/p?simpleloop/4/4/1000",
		"https://puzz.link/p?simpleloop_edit/4/4/1000",
		"simpleloop/4/4/A",
		"simpleloop/4/4/000g",
		"simpleloop/4/4/0001",
		"simpleloop/4/4/0000",
		"simpleloop/4/4/0",
		"simpleloop/4/4/1000zzz",
		"simpleloop/3/2/1zz",
		"simpleloop/3/2/zzzz",
		"simpleloop/3/2/100",
		"simpleloop/1/1/",
		"simpleloop/1/1/g",
		"simpleloop/5/1/v",
		"simpleloop/25/1/00000",
		"simpleloop/1/25/00000",
		"simpleloop/2/2/v",
		"simpleloop/2/1/g",
		"simpleloop/1/2/g",
		"simpleloop/x/4/4/1000",
		"https://puzz.link/p?simpleloop/10/10/0o40441k00c00c005g43",
		"https://puzz.link/p?simpleloop/10/10/4h00008g0000g1000040",
		"https://puzz.link/p?simpleloop/10/10/08004450g40g01900004"
	],
	fileios: [
		"pzprv3/simpleloop/3/3/* * . /* * . /. . . /1 -1 /0 1 /0 0 /0 0 0 /-1 1 0 /",
		"pzprv3/simpleloop/4/4/. . . . /. . . . /. . . . /. . . * /1 0 0 /0 -1 0 /1 0 1 /0 0 0 /1 0 0 1 /0 -1 0 0 /1 0 0 0 /",
		"pzprv3/simpleloop/1/2/* . /1 /",
		"pzprv3/simpleloop/1/1/* //"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 0, 7]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

// Fixture inputs for pzprjs pid "midloop". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "midloop",
	dataset: "MidLoop",
	urls: [
		"midloop/4/4/2fezpf",
		"https://puzz.link/p?midloop/4/4/2fezpf",
		"http://pzv.jp/p.html?type=player&midloop/4/4/2fezpf",
		"midloop/f/4/4/2fezpf",
		"midloop/xf/4/4/2fezpf",
		"midloop/4/4/1",
		"midloop/4/4/2fezpfzz",
		"midloop/4/4",
		"midloop/4/4/",
		"midloop/2/2/i11",
		"midloop/2/2/je",
		"midloop/2/2/j",
		"midloop/2/2/0000",
		"midloop/1/1/e",
		"midloop/1/1/f",
		"midloop/1/1/i",
		"midloop/8/1/em",
		"midloop/1/8/em",
		"midloop/4/4/A",
		"midloop/4/4/:",
		"midloop/10/10",
		"https://puzz.link/p?midloop/10/10/v5fvdfyfrfz5fgfzfjfztfififtfkfmfzfzlf",
		"https://puzz.link/p?midloop/10/10/ifzrb9dfodfzlfifvfg5fkflbfz7fjfhfwfgfzzjft",
		"https://puzz.link/p?midloop/10/10/sfzgbffr7fjfwd3fg3fzifzfzjbfgfzgfzlfzzjf",
		"https://puzz.link/p?midloop/10/10/hfzzfifvffvfzjfhffzfbfsfztbfztfybf"
	],
	fileios: [
		"pzprv3/midloop/4/4/1.2..../.2.1.../...-.../......./....X../......./......2/1 0 0 /0 0 0 /0 0 0 /0 0 0 /0 0 0 0 /0 0 -1 0 /0 0 0 0 /",
		"pzprv3/midloop/3/3/f/.1.../.2.../..2../...X./...../0 0 /1 0 /0 0 /0 0 0 /0 0 0 /"
	],
	randomize(p, r) {
		for (let s = 0; s < p.board.dotsmax; s++) {
			const dot = p.board.dots[s];
			const x = r.rnd();
			if (x < 0.12) {
				dot.setDot(1);
			} else if (x < 0.24) {
				dot.setDot(2);
			} else if (x < 0.34) {
				dot.setDot(-2);
			} else if (x < 0.44) {
				dot.piece.ques = 7;
			} else {
				dot.setDot(0);
			}
		}
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.line = 1;
			} else if (x < 0.3) {
				border.qsub = 2;
			}
		});
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

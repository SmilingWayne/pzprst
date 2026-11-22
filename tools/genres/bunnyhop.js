// Fixture inputs for pzprjs pid "bunnyhop". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "bunnyhop",
	dataset: null,
	urls: [
		// Real puzzles, already canonical with pzprjs.
		"bunnyhop/8/8/ph2106e1g441g",
		"bunnyhop/12/12/j4i80oa9d0c7q1c106lt5ah0914a8",
		"https://puzz.link/p?bunnyhop/8/8/ph2106e1g441g",
		"http://pzv.jp/p.html?type=player&bunnyhop/12/12/j4i80oa9d0c7q1c106lt5ah0914a8",
		// The `bnuuyhop` alias.
		"bnuuyhop/8/8/ph2106e1g441g",
		"bunnyhop/8/8/ph2106e1g441g/",
		// Shaded cells: run lengths, partial last group, unknown characters.
		"bunnyhop/3/3/1",
		"bunnyhop/3/3/10",
		"bunnyhop/3/3/A",
		"bunnyhop/3/3/v",
		"bunnyhop/3/3/zz",
		// An unknown flag letter is ignored.
		"bunnyhop/x/3/3/1",
		// 1/1, 1/N and N/1 boards.
		"bunnyhop/1/1/1",
		"bunnyhop/1/1",
		"bunnyhop/2/1/v",
		"bunnyhop/1/2/v",
		// Missing body.
		"bunnyhop/3/3"
	],
	fileios: [
		// Question layer, border line layer and every peke/half mark.
		"pzprv3/bunnyhop/2/2/# . /. # /1 2 1 /2 1 2 /1 2 /2 1 /1 2 /3 4 /79 0 /",
		// Outer border lines, `-1` crosses and a 3x3 cell layer.
		"pzprv3/bunnyhop/3/3/. # . /. . . /# . . /1 0 2 0 /0 -1 0 1 /2 0 0 -1 /1 0 0 /0 2 0 /0 0 1 /-1 0 0 /3 4 0 /0 5 6 /7 0 15 /"
	],
	randomize(p, r) {
		const pekes = [0, 1, 2, 3, 4, 6, 8, 9, 12, 15];
		const halves = [0, 1, 2, 3, 4];
		p.board.cell.each((cell) => {
			cell.ques = r.rnd() < 0.3 ? 7 : 0;
			cell.qsub = r.pick(pekes) | (r.pick(halves) << 4);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.3) {
				border.line = r.pick([1, 2]);
			} else if (x < 0.45) {
				border.qsub = 2;
			}
		});
	}
};

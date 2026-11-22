// Fixture inputs for pzprjs pid "reflect". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "reflect",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"reflect/10/10/j16g5f20g20c20h33e23a20c5e44e46f43g5f14f20",
		"reflect/10/10/j10b20b14g24a40b20a14g30c10c30b15e33e35b34c20d30f30d30a5k",
		"reflect/10/10/l2010d5c3340b2510h3445c5p5c2310h3044b2414c5d3443l",
		"https://puzz.link/p?reflect/10/10/j16g5f20g20c20h33e23a20c5e44e46f43g5f14f20",
		"http://pzv.jp/p.html?type=player&reflect/10/10/j16g5f20g20c20h33e23a20c5e44e46f43g5f14f20",
		// Largest storable clue (255) and the zero/empty clue spelling.
		"reflect/4/4/6ffo",
		"reflect/4/4/10o",
		"reflect/2/2/10c",
		"reflect/1/1/5",
		"reflect/5/1/16523a",
		"reflect/1/5/16523a",
		"reflect/3/3/",
		"reflect/4/4/6ffozz",
		"reflect/6/6/j16g",
		"reflect/2/2/10c/",
		"reflect/z/4/4/10o"
	],
	fileios: [
		// Small board: every triangle direction, the 11 mark and one empty cell.
		"pzprv3/reflect/3/3/16 . 20 /35 + . /. 46 23 /1 0 /-1 0 /0 1 /0 1 0 /1 0 -1 /",
		// Answer borders (line 1 and cross -1) with a zero clue.
		"pzprv3/reflect/2/2/10 + /23 . /1 /-1 /0 1 /"
	],
	randomize(p, r) {
		const kinds = [0, 0, 0, 0, 2, 3, 4, 5, 11];
		const cluePool = [3, 4, 5, 9, 10, 15, 16];
		p.board.cell.each((cell) => {
			cell.ques = r.pick(kinds);
			cell.qnum = -1;
			if (cell.ques >= 2 && cell.ques <= 5) {
				const maxnum = cell.getmaxnum();
				const clues = cluePool.filter((n) => n <= maxnum);
				if (clues.length && r.rnd() < 0.5) {
					cell.qnum = r.pick(clues);
				}
			}
		});
		p.board.border.each((border) => {
			border.line = r.rnd() < 0.25 ? 1 : 0;
			border.qsub = r.rnd() < 0.2 ? 2 : 0;
		});
	}
};

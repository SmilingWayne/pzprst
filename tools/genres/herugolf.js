// Fixture inputs for pzprjs pid "herugolf". See tools/gen_fixtures.js for the format.
// There is no Herugolf folder under ../puzzlekit-dataset/assets/data, so the two
// real puzzles below come from pzprjs's own test data (test/script, test/variety).
"use strict";

module.exports = {
	pid: "herugolf",
	dataset: null,
	urls: [
		"herugolf/6/6/1cg00j803ihp2i2ihlhjhhj3l3k",
		"https://puzz.link/p?herugolf/6/6/1cg00j803ihp2i2ihlhjhhj3l3k",
		"herugolf_edit/6/6/1cg00j803ihp2i2ihlhjhhj3l3k",
		"http://pzv.jp/p.html?type=player&herugolf/6/6/1cg00j803ihp2i2ihlhjhhj3l3k",
		"herugolf/3/3/002p",
		"herugolf/3/2/h3.1i",
		"herugolf/2/2/-1a",
		"herugolf/3/3/00-10",
		"herugolf/4/4/00003",
		"herugolf/2/2/0i",
		"herugolf/1/1/0i",
		"herugolf/1/3/0i",
		"herugolf/3/1/0i",
		"herugolf/1/1/",
		"herugolf/1/1/0",
		"herugolf/2/1/zz",
		"herugolf/2/2/0iazz",
		"herugolf/5/5/00000zo",
		"herugolf/26/1/000000zoa",
		"herugolf/x/2/2/0i",
		"herugolf/2/2/0..",
		"herugolf/3/3/402mhj",
		"herugolf/2/2/h.",
		"herugolf/2/2/H.",
		"herugolf/3/3/00A.",
		"herugolf/6/6/",
		"herugolf/6/6/1"
	],
	fileios: [
		// The real 6x6 puzzle in its solved pzprjs test state (answer border lines only).
		"pzprv3/herugolf/6/6/3 . H . i . /i i . . i 2 /. 2 . H . . /. . H . . H /H i . 3 i i /. i 3 . . . /0 1 0 0 0 /0 0 0 1 1 /0 -1 0 0 0 /1 1 0 0 0 /1 1 1 0 0 /0 0 1 1 1 /1 1 0 0 0 0 /1 1 0 1 0 0 /1 0 0 0 0 0 /-1 0 0 0 0 1 /0 0 0 0 0 1 /",
		// Clue 2, water and hole with answer lines and a cross.
		"pzprv3/herugolf/3/3/2 . i /. . . /H . . /1 0 /0 1 /-1 0 /0 0 1 /1 0 0 /",
		// 2x2: all cells empty, answer lines only.
		"pzprv3/herugolf/2/2/. . /. . /1 /0 /0 1 /",
		// 1x1 has no inner borders, so only the cell line and one empty border line.
		"pzprv3/herugolf/1/1/i //"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const cx = cell.bx >> 1;
			const cy = cell.by >> 1;
			const max = Math.max(
				cx,
				cy,
				p.board.cols - 1 - cx,
				p.board.rows - 1 - cy
			);
			cell.qnum = -1;
			cell.ques = 0;
			const x = r.rnd();
			if (x < 0.15) {
				cell.ques = 31;
			} else if (x < 0.3) {
				cell.ques = 6;
			} else if (x < 0.8 && max >= 1) {
				cell.qnum = 1 + Math.floor(r.rnd() * max);
			}
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

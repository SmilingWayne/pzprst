// Fixture inputs for pzprjs pid "bhaibahan". See tools/gen_fixtures.js for the format.
// There is no Bhai Bahan folder under ../puzzlekit-dataset/assets/data and
// .pzprst-samples.js finds no native-derived links, so the two real puzzles come
// from pzprjs's own test data: test/script/bhaibahan.js (the 6x6 debug URL) and
// its 2x4 "loop_full" FileIO (canonicalised to bhaibahan/f/4/2/2.l).
"use strict";

module.exports = {
	pid: "bhaibahan",
	dataset: null,
	urls: [
		"bhaibahan/6/6/34o1k3.k.o28",
		"bhaibahan/f/4/2/2.l",
		"https://puzz.link/p?bhaibahan/6/6/34o1k3.k.o28",
		"http://pzv.jp/p.html?type=player&bhaibahan/6/6/34o1k3.k.o28",
		"bhaibahan_edit/6/6/34o1k3.k.o28",
		"bhaibahan/f/6/6/34o1k3.k.o28",
		"bhaibahan/xf/6/6/34o1k3.k.o28",
		"bhaibahan/6/6/34o1k3.k.o28/",
		"bhaibahan/6/6/34o1k3",
		"bhaibahan/6/6/34o1k3.k.o28zz",
		"bhaibahan/4/4/................",
		"bhaibahan/4/4/-10",
		"bhaibahan/1/1/1",
		"bhaibahan/1/1/.",
		"bhaibahan/1/5/12345",
		"bhaibahan/5/1/12345",
		"bhaibahan/3/3/z1",
		"bhaibahan/3/3/A1",
		"bhaibahan/6/6/",
		"bhaibahan/f/1/1"
	],
	fileios: [
		// The 6x6 debug puzzle, question layer only.
		"pzprv3/bhaibahan/6/6/3 4 . . . . /. . . . . 1 /. . . . . 3 /- . . . . . /- . . . . . /. . . . 2 8 /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /",
		// The same puzzle in pzprjs's solved test state: backgrounds `+`/`-` and lines.
		"pzprv3/bhaibahan/6/6/3 4 . . . . /. . . . . 1 /. . . . . 3 /- . . . . . /- . . . . . /. . . . 2 8 /- + + + + - /- - - + - + /. + - - - - /- - . - - . /+ - - . - - /- - - + + - /1 1 1 1 1 /1 0 1 1 0 /0 0 1 0 1 /1 0 0 1 0 /0 1 0 0 1 /1 0 1 1 1 /1 0 0 0 0 1 /0 1 1 0 1 1 /0 1 0 1 0 0 /1 0 0 0 1 0 /1 1 1 0 0 1 /",
		// The 2x4 "loop_full" test board with every config flag line.
		"pzprv3/bhaibahan/2/4/f/2 - . . /. . . . /. . . . /. . . . /1 1 0 /1 1 0 /1 0 1 0 /",
		// Answer layer with every qsub mark (`+ - = %`) and an answer shade (`1`).
		"pzprv3/bhaibahan/2/3/1 2 . /. . . /. + - /= % 1 /0 0 /0 0 /0 0 0 /",
		// 1x1 has no inner borders, so only the two cell layers.
		"pzprv3/bhaibahan/1/1/1 /. /"
	],
	randomize(p, r) {
		const max = p.board.cols * p.board.rows;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.15) {
				cell.qnum = -2;
			} else if (x < 0.45) {
				cell.qnum = 1 + Math.floor(r.rnd() * max);
			} else {
				cell.qnum = -1;
			}
			// FileIO stores qans and qsub in one token, so never set both.
			const y = r.rnd();
			if (y < 0.1) {
				cell.qans = 1;
			} else if (y < 0.35) {
				cell.qsub = r.pick([1, 2]);
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
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

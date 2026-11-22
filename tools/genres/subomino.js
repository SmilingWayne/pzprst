// Fixture inputs for pzprjs pid "subomino". See tools/gen_fixtures.js for the format.
// No Subomino folder exists under ../puzzlekit-dataset/assets/data and
// .pzprst-samples.js finds no native-derived links, so the real puzzles come from
// pzprjs's own test data (test/script/subomino.js).
"use strict";

module.exports = {
	pid: "subomino",
	dataset: null,
	urls: [
		"subomino/5/5/3j6j3i4o410000",
		"subomino/3/3/3n3i",
		"https://puzz.link/p?subomino/5/5/3j6j3i4o410000",
		"http://pzv.jp/p.html?type=player&subomino/5/5/3j6j3i4o410000",
		"subomino_edit/5/5/3j6j3i4o410000",
		// No rule flags exist; an unknown one is dropped by pzprjs.
		"subomino/x/5/5/3j6j3i4o410000",
		// Truncated body, trailing junk and a trailing slash.
		"subomino/5/5/3j6j3i4",
		"subomino/5/5/3j6j3i4o410000zz",
		"subomino/5/5/3j6j3i4o410000/",
		// `?` is a dot; shaded cells come from the binary empty layer.
		"subomino/5/5/.",
		"subomino/5/5/g",
		"subomino/5/5/g1h2i3",
		"subomino/4/4/A",
		// maxnum is cols*rows (25 on 5x5, hex -19); minnum is 2.
		"subomino/5/5/-19",
		"subomino/4/4/0000000000000000",
		// 1/1, 1/N and N/1 boards.
		"subomino/1/1/.",
		"subomino/1/1/0",
		"subomino/1/5/00000",
		"subomino/5/1/00000",
		"subomino/1/20/v",
		"subomino/20/1/v",
		// Empty bodies still canonicalise with run-length markers.
		"subomino/4/4",
		"subomino/4/4/",
		"subomino/x/4/4/0"
	],
	fileios: [
		// The 5x5 solved pzprjs test state (question and border layers).
		"pzprv3/subomino/5/5/3 . . . * /6 . . . . /3 . . . 4 /. . . . . /. . . . 4 /0 0 0 1 0 0 /0 0 1 0 1 0 /0 1 0 1 1 0 /0 1 1 0 1 0 /0 1 0 0 1 0 /0 0 0 0 0 /1 1 1 0 0 /1 0 1 0 0 /0 1 0 1 0 /0 0 1 1 0 /0 0 0 0 0 /",
		// A 3x3 board with shaded cells (`*`) and no border answers.
		"pzprv3/subomino/3/3/3 . . /* * * /. . * /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 /0 0 0 /0 0 0 /0 0 0 /",
		// Every question token (`?`, number, shaded) and border-answer buttons.
		"pzprv3/subomino/3/3/. 3 - /3 * 3 /. . 6 /0 1 0 2 /0 -1 0 0 /0 0 1 0 /0 0 0 /1 0 0 /0 0 0 /0 0 0 /",
		// 1x1: two vertical and two horizontal border tokens.
		"pzprv3/subomino/1/1/2 /0 0 /0 /0 /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		const clues = [-1, -1, -1, -1, -2];
		for (let n = 2; n <= Math.min(maxnum, 12); n++) {
			clues.push(n);
		}
		if (maxnum >= 2) {
			clues.push(maxnum);
		}
		p.board.cell.each((cell) => {
			// A `*` token wins over the number, so never set both.
			if (r.rnd() < 0.15) {
				cell.ques = 7;
			} else {
				cell.qnum = r.pick(clues);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.qans = 1;
			} else if (x < 0.4) {
				border.qsub = 1;
			}
		});
	}
};

// Fixture inputs for pzprjs pid "narrow". See tools/gen_fixtures.js for the format.
// No Narrowfence folder exists under ../puzzlekit-dataset/assets/data and
// .pzprst-samples.js finds no native-derived links, so the real puzzle comes from
// pzprjs's own test data (test/script/narrow.js and
// test/state/extended_variety_fixtures.js, the same 5x5 URL).
"use strict";

module.exports = {
	pid: "narrow",
	dataset: null,
	urls: [
		"narrow/5/5/g22p113i1k",
		"https://puzz.link/p?narrow/5/5/g22p113i1k",
		"http://pzv.jp/p.html?type=player&narrow/5/5/g22p113i1k",
		"narrow_edit/5/5/g22p113i1k",
		// No rule flags exist; an unknown one is dropped by pzprjs.
		"narrow/x/5/5/g22p113i1k",
		// Truncated body, trailing junk and a trailing slash.
		"narrow/5/5/g22p113",
		"narrow/5/5/g22p113i1kzz",
		"narrow/5/5/g22p113i1k/",
		// `?` is a dot; shaded cells come from the binary empty layer.
		"narrow/5/5/.",
		"narrow/5/5/g",
		"narrow/5/5/g1h2i3",
		"narrow/4/4/A",
		// The largest clue is 4; zero is below `minnum` but pzprjs reads it.
		"narrow/5/5/4444444444444444444444444",
		"narrow/4/4/0123",
		// 1/1, 1/N and N/1 boards.
		"narrow/1/1/.",
		"narrow/1/1/0",
		"narrow/1/5/00000",
		"narrow/5/1/00000",
		"narrow/1/20/v",
		"narrow/20/1/v",
		// Empty bodies still canonicalise with run-length markers.
		"narrow/4/4",
		"narrow/4/4/",
		"narrow/x/4/4/0"
	],
	fileios: [
		// The 5x5 solved pzprjs test state (question and border layers).
		"pzprv3/narrow/5/5/. 2 2 . . /. . . . . /. . . 1 1 /3 . . . 1 /. . . . . /0 0 1 1 0 0 /0 1 1 0 1 0 /0 1 0 0 1 0 /0 1 1 1 0 0 /0 0 0 0 1 0 /0 0 0 0 0 /0 1 0 1 0 /0 0 1 1 0 /1 0 1 1 1 /0 1 0 1 0 /0 0 0 0 0 /",
		// A 3x3 board with a shaded cell (`*`) and no border answers.
		"pzprv3/narrow/3/3/. 2 2 /. * . /. . . /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 /0 0 0 /0 0 0 /0 0 0 /",
		// Buttons `2`, `1`, `-1` on the border-answer layer.
		"pzprv3/narrow/3/3/. 2 2 /. * . /. . . /0 1 0 2 /0 -1 0 0 /0 0 1 0 /0 0 0 /1 0 0 /0 0 0 /0 0 0 /",
		// 1x1: two vertical and two horizontal border tokens.
		"pzprv3/narrow/1/1/1 /0 0 /0 /0 /"
	],
	randomize(p, r) {
		const clues = [-1, -1, -1, -1, 1, 2, 3, 4];
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

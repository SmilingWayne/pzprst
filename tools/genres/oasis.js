// Fixture inputs for pzprjs pid "oasis". See tools/gen_fixtures.js for the format.
// No Oasis folder exists under ../puzzlekit-dataset/assets/data and
// .pzprst-samples.js finds no native-derived links, so the real puzzles come from
// pzprjs's own test data: test/script/oasis.js and
// test/state/extended_variety_fixtures.js (both canonicalised to the same 6x6 URL).
"use strict";

module.exports = {
	pid: "oasis",
	dataset: null,
	urls: [
		"oasis/6/6/2.m.l3h2l.m.1",
		"https://puzz.link/p?oasis/6/6/2.m.l3h2l.m.1",
		"http://pzv.jp/p.html?type=player&oasis/6/6/2.m.l3h2l.m.1",
		"oasis_edit/6/6/2.m.l3h2l.m.1",
		// No rule flags exist; an unknown one is dropped by pzprjs.
		"oasis/x/6/6/2.m.l3h2l.m.1",
		// Truncated body, trailing junk and a trailing slash.
		"oasis/6/6/2.m.l3h2l",
		"oasis/6/6/2.m.l3h2l.m.1zz",
		"oasis/6/6/2.m.l3h2l.m.1/",
		// `?` is a dot; skipped cells use g-z run lengths.
		"oasis/3/3/.",
		"oasis/3/3/.........",
		"oasis/5/5/g1h2i3",
		"oasis/4/4/A",
		// maxnum is cols*rows-1: 24 on 5x5 (hex -18), 15 on 4x4 (hex f).
		"oasis/5/5/-18",
		"oasis/4/4/f",
		// Zero is below `minnum`, but pzprjs reads it.
		"oasis/4/4/0",
		// 1/1, 1/N and N/1 boards.
		"oasis/1/1/.",
		"oasis/1/1/0",
		"oasis/1/5/0",
		"oasis/5/1/0",
		"oasis/1/20/v",
		"oasis/20/1/v",
		// Empty bodies still canonicalise with a run-length marker.
		"oasis/4/4",
		"oasis/4/4/",
		"oasis/x/4/4/0"
	],
	fileios: [
		// The 6x6 solved pzprjs test state (question and answer layers).
		"pzprv3/oasis/6/6/2 - . . . . /. . . - . . /. . . . 3 . /. 2 . . . . /. . - . . . /. . . . - 1 /+ + 1 + + + /+ 1 + + 1 + /+ + 1 + + 1 /1 + + 1 + + /+ 1 + + + 1 /+ + + 1 + + /",
		// A 3x3 test state with `?`, a two-digit clue and the cmp/sub marks.
		"pzprv3/oasis/3/3/. 1 . /. . . /. . 2 /. - . /. 1 + /. . . /",
		// Every cell answer token, `?`, and both border layers.
		"pzprv3/oasis/2/3/1 2 . /. . - /1 + . /. - 1 /0 -1 0 0 /0 0 0 /",
		// 1x1 has no inner borders, so only the two cell layers are written.
		"pzprv3/oasis/1/1/0 /. /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows - 1;
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 1; n <= Math.min(maxnum, 12); n++) {
			clues.push(n);
		}
		if (maxnum >= 1) {
			clues.push(maxnum);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			// FileIO stores qans, qsub and qcmp as one token, so set at most one.
			const x = r.rnd();
			if (x < 0.15) {
				cell.qans = 1;
			} else if (x < 0.3) {
				cell.qsub = 1;
			} else if (x < 0.45) {
				cell.qcmp = 1;
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

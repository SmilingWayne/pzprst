// Fixture inputs for pzprjs pid "kurochute". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kurochute",
	dataset: "Kuroshuto",
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"kurochute/5/5/132k1i1i2k332",
		"kurochute/5/5/12k1h1i1h1k23",
		"kurochute/14/8/1i7l32g2h4g1j6h13i2j1i24l2k35k5l46i4j3i57h2j6g2h6g43l1i7",
		"https://puzz.link/p?kurochute/5/5/132k1i1i2k332",
		"http://pzv.jp/p.html?type=player&kurochute/5/5/132k1i1i2k332",
		"kurochute_edit/5/5/132k1i1i2k332",
		// No rule flags exist; an unknown one is dropped by pzprjs.
		"kurochute/x/5/5/132k1i1i2k332",
		// Truncated body, trailing junk and a trailing slash.
		"kurochute/5/5/132k1i1i2k",
		"kurochute/5/5/132k1i1i2k332zz",
		"kurochute/5/5/132k1i1i2k332/",
		// `?` is a dot; skipped cells use g-z run lengths.
		"kurochute/3/3/.",
		"kurochute/3/3/.........",
		"kurochute/5/5/g1h2i3",
		"kurochute/3/3/A",
		// The largest clue on 8x8/4x4 boards is max(cols,rows)-1 = 7/3.
		"kurochute/8/8/7",
		"kurochute/4/4/3333333333333333",
		// Zero is below `minnum`, but pzprjs reads it.
		"kurochute/4/4/0123",
		// 1/1, 1/N and N/1 boards.
		"kurochute/1/1/.",
		"kurochute/1/1/0",
		"kurochute/1/5/0",
		"kurochute/5/1/0",
		"kurochute/1/20/v",
		"kurochute/20/1/v",
		// Empty bodies still canonicalise with a run-length marker.
		"kurochute/4/4",
		"kurochute/4/4/",
		"kurochute/x/4/4/0"
	],
	fileios: [
		// The 5x5 solved pzprjs test state (question and answer layers).
		"pzprv3/kurochute/5/5/1 3 2 . . /. . . 1 . /. . 1 . . /. 2 . . . /. . 3 3 2 /- - - + 1 /1 + 1 . + /+ + . . 1 /. - + 1 . /1 . - - . /",
		// A 1x5 board with a cmp mark and the border "cross" mark (-1).
		"pzprv3/kurochute/1/5/1 . . . . /- . . . . /0 -1 -1 0 /",
		// Every cell answer token, `?`, and both border layers.
		"pzprv3/kurochute/2/3/1 2 . /. . - /1 + . /. - 1 /0 -1 0 0 /0 0 0 /",
		// 1x1 has no inner borders, so only the two cell layers are written.
		"pzprv3/kurochute/1/1/0 /. /"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows) - 1;
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 1; n <= maxnum; n++) {
			clues.push(n);
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

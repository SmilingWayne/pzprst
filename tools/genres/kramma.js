// Fixture inputs for pzprjs pid "kramma". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kramma",
	dataset: null,
	urls: [
		// Real puzzles, already canonical with pzprjs.
		"kramma/c/8/8/903j209i160130i966j009",
		"kramma/c/7/7/j6a3b3j6b3j3k6j69",
		"kramma/c/10/10/0fb6k199321190i0e09i21i312ik9160j0",
		"https://puzz.link/p?kramma/c/8/8/903j209i160130i966j009",
		"http://pzv.jp/p.html?kramma/c/5/5/9ock3ba9i",
		// pzprjs's own test URL.
		"kramma/c/5/5/9ock3ba9i",
		// The `c` flag: pzprjs always writes it. Without it the cross-mark layer
		// comes first; a full run of unmarked crosses is skipped and the board
		// keeps the pid (pzprjs re-reads a kramma URL with a dot as `kramman`,
		// so no dot can appear here).
		"kramma/c/3/3/000",
		"kramma/c/3/3/4000",
		"kramma/3/3/4000",
		"kramma/3/3/49ck",
		"kramma/x/3/3/4000",
		"kramma/cz/3/3/000",
		// Truncated bodies and trailing junk.
		"kramma/c/3/3/0",
		"kramma/c/4/4/0",
		"kramma/c/3/3/000zz",
		// The largest circle value 2.
		"kramma/c/2/2/qi",
		// 1x1, 1xN and Nx1 boards.
		"kramma/c/1/1/0",
		"kramma/1/1/00",
		"kramma/c/1/2/0",
		"kramma/c/2/1/0"
	],
	fileios: [
		// pzprjs's own answer-data fixture: circles, border lines and cross marks.
		"pzprv3/kramma/5/5/1 . . 2 2 /. 1 1 . 2 /. 2 . 1 . /1 . 2 1 . /1 1 . . 2 /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /1 0 1 -1 /1 0 1 -1 /1 -1 1 -1 /1 -1 1 -1 /1 0 1 -1 /-1 -1 -1 -1 -1 /1 1 1 1 1 /-1 -1 -1 -1 -1 /1 1 1 1 1 /",
		// Question marks on a circle and a cross. Cross lines must not contain a
		// `1` or pzprjs would re-read the file as `kramman`.
		"pzprv3/kramma/2/2/- 1 /. 2 /- . . /. . . /. . . /1 /0 /1 0 /",
		// A `?` cross and both border answer tokens.
		"pzprv3/kramma/3/3/1 . - /. 2 . /- . 1 /- . . . /. . . . /. . . . /. . . . /1 0 /2 -1 /0 0 /1 0 -1 /-1 2 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qnum = x < 0.6 ? -1 : x < 0.85 ? r.pick([1, 2]) : -2;
		});
		// A cross number of 1 would make pzprjs read the file back as `kramman`.
		p.board.cross.each((cross) => {
			cross.qnum = r.rnd() < 0.85 ? -1 : -2;
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qans = x < 0.25 ? 1 : 0;
			border.qsub = x >= 0.25 && x < 0.45 ? 1 : 0;
		});
	}
};

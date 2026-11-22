// Fixture inputs for pzprjs pid "cbanana". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "cbanana",
	dataset: null,
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"cbanana/8/8/j3o4g3r6p6g3k4j4k6j",
		"cbanana/11/11/r6m6k4g8w8g5g6g7u5g8g7g3w6g5k5m3r",
		"cbanana/6/6/33k3h3q3g3j3l",
		"https://puzz.link/p?cbanana/8/8/j3o4g3r6p6g3k4j4k6j",
		"http://pzv.jp/p.html?type=player&cbanana/8/8/j3o4g3r6p6g3k4j4k6j",
		"cbanana/8/8/j3o4g3r6p6g3k4j4k6j/",
		"cbanana/8/8/j3o4g3r6p6g3k4j4k6j//",
		"cbanana/8/8/j3o4g3r6p6g3k4j4k6jzz",
		"cbanana/8/8/j3o4g3r6p6g3k4j4k",
		// The largest clue is cols*rows; 16 and 36 are hex escapes ("-10", "-24").
		"cbanana/4/4/g1",
		"cbanana/4/4/-10",
		"cbanana/6/6/-24",
		"cbanana/2/2/-3",
		// `?` is a dot; two-digit clues use decimal FileIO but hex escapes in URLs.
		"cbanana/3/3/.",
		"cbanana/3/3/123456789",
		"cbanana/3/3/.........",
		// Skipped cells (g-z), incomplete escapes and unknown characters.
		"cbanana/5/5/g1h2i3",
		"cbanana/5/5/j",
		"cbanana/5/5/zzzzz",
		"cbanana/4/4/A",
		// 1/1, 1/N and N/1 boards.
		"cbanana/1/1/.",
		"cbanana/1/1/1",
		"cbanana/1/1/-1",
		"cbanana/1/6/1",
		"cbanana/6/1/1",
		"cbanana/1/20/v",
		"cbanana/20/1/v",
		// Empty bodies still canonicalise with a run-length marker.
		"cbanana/4/4",
		"cbanana/4/4/",
		"cbanana/x/4/4/1"
	],
	fileios: [
		// Question and answer layers.
		"pzprv3/cbanana/3/3/1 2 3 /. . . /. 4 . /. # . /# . # /. . .",
		// Question marks, two-digit clues and every answer token.
		"pzprv3/cbanana/2/3/- 2 16 /. 3 . /+ # . /. . # /+ . .",
		// Single-cell boards.
		"pzprv3/cbanana/1/1/1 /#",
		"pzprv3/cbanana/1/2/. 2 /. ."
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 1; n <= Math.min(maxnum, 12); n++) {
			clues.push(n);
		}
		if (maxnum >= 1) {
			clues.push(maxnum);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			const x = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.3) {
				cell.qsub = 1;
			}
		});
	}
};

// Fixture inputs for pzprjs pid "hasunomura". See tools/gen_fixtures.js for the format.
// No crawled real-puzzle URLs exist for this pid and there is no dataset folder,
// so the boards below are hand-written clue layouts that pzprjs canonicalises.
"use strict";

module.exports = {
	pid: "hasunomura",
	dataset: null,
	urls: [
		"hasunomura/6/6/33k3h3q3g3j3l",
		"hasunomura/5/5/0h0g0",
		"hasunomura/7/7/3h2g5",
		"https://puzz.link/p?hasunomura/6/6/33k3h3q3g3j3l",
		"http://pzv.jp/p.html?type=player&hasunomura/6/6/33k3h3q3g3j3l",
		"hasunomura/6/6/33k3h3q3g3j3l/",
		"hasunomura/6/6/33k3h3q3g3j3l//",
		"hasunomura/6/6/33k3h3q3g3j3lzz",
		"hasunomura/6/6/33k3h3q3g3j3",
		// The largest clue is cols*rows; 16 and 36 are hex escapes ("-10", "-24").
		"hasunomura/4/4/-10",
		"hasunomura/6/6/-24",
		"hasunomura/2/2/-3",
		// `minnum` is 0, so zero clues are legal ("0" in hex is the digit 0).
		"hasunomura/3/3/000000000",
		"hasunomura/3/3/023456789",
		// `?` is a dot.
		"hasunomura/3/3/.",
		"hasunomura/3/3/.........",
		// Skipped cells (g-z), short bodies and unknown characters.
		"hasunomura/5/5/g1h2i3",
		"hasunomura/5/5/j",
		"hasunomura/5/5/zzzzz",
		"hasunomura/4/4/A",
		"hasunomura/6/6/2",
		// 1/1, 1/N and N/1 boards.
		"hasunomura/1/1/.",
		"hasunomura/1/1/0",
		"hasunomura/1/6/0",
		"hasunomura/6/1/0",
		"hasunomura/1/20/v",
		"hasunomura/20/1/v",
		// Empty bodies still canonicalise with a run-length marker.
		"hasunomura/4/4",
		"hasunomura/4/4/",
		"hasunomura/x/4/4/0"
	],
	fileios: [
		// Question and answer layers, including the legal zero clue.
		"pzprv3/hasunomura/3/3/0 3 . /. 0 . /. . 9 /. # . /# . # /. . .",
		// Question marks, two-digit clues and every answer token.
		"pzprv3/hasunomura/2/3/- 0 16 /. 3 . /+ # . /. . # /+ . .",
		// Single-cell boards.
		"pzprv3/hasunomura/1/1/0 /#",
		"pzprv3/hasunomura/1/2/. 2 /. ."
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		const clues = [-1, -1, -1, -1, -2, 0, 0];
		for (let n = 0; n <= Math.min(maxnum, 12); n++) {
			clues.push(n);
		}
		clues.push(maxnum);
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

// Fixture inputs for pzprjs pid "cornerch". See tools/gen_fixtures.js for the format.
// No crawled real-puzzle URLs exist for this pid and there is no dataset folder,
// so the boards below are hand-written clue layouts that pzprjs canonicalises.
"use strict";

module.exports = {
	pid: "cornerch",
	dataset: null,
	urls: [
		"cornerch/6/6/33k3h3q3g3j3l",
		"cornerch/8/8/j3o4g3r6p6g3k4j4k6j",
		"cornerch/7/7/2h3g4",
		"https://puzz.link/p?cornerch/6/6/33k3h3q3g3j3l",
		"http://pzv.jp/p.html?type=player&cornerch/6/6/33k3h3q3g3j3l",
		"cornerch/6/6/33k3h3q3g3j3l/",
		"cornerch/6/6/33k3h3q3g3j3l//",
		"cornerch/6/6/33k3h3q3g3j3lzz",
		"cornerch/6/6/33k3h3q3g3j3",
		// The largest clue is cols*rows; 16 and 36 are hex escapes ("-10", "-24").
		"cornerch/4/4/-10",
		"cornerch/6/6/-24",
		"cornerch/2/2/-3",
		// `?` is a dot; clues below minnum (2) are still decoded by pzprjs.
		"cornerch/3/3/.",
		"cornerch/3/3/234234234",
		"cornerch/3/3/.........",
		"cornerch/3/3/111",
		// Skipped cells (g-z), short bodies and unknown characters.
		"cornerch/5/5/g1h2i3",
		"cornerch/5/5/j",
		"cornerch/5/5/zzzzz",
		"cornerch/4/4/A",
		"cornerch/6/6/2",
		// 1/1, 1/N and N/1 boards.
		"cornerch/1/1/.",
		"cornerch/1/1/2",
		"cornerch/1/6/2",
		"cornerch/6/1/2",
		"cornerch/1/20/v",
		"cornerch/20/1/v",
		// Empty bodies still canonicalise with a run-length marker.
		"cornerch/4/4",
		"cornerch/4/4/",
		"cornerch/x/4/4/2"
	],
	fileios: [
		// Question and answer layers.
		"pzprv3/cornerch/3/3/2 3 . /. 4 . /. . 5 /. # . /# . # /. . .",
		// Question marks and two-digit clues with every answer token.
		"pzprv3/cornerch/2/3/- 2 6 /. 3 . /+ # . /. . # /+ . .",
		// Single-cell boards.
		"pzprv3/cornerch/1/1/2 /#",
		"pzprv3/cornerch/1/2/. 2 /. ."
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 2; n <= Math.min(maxnum, 12); n++) {
			clues.push(n);
		}
		if (maxnum >= 2) {
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

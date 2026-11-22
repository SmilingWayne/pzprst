// Fixture inputs for pzprjs pid "island". See tools/gen_fixtures.js for the format.
// No crawled real-puzzle URLs exist for this pid and there is no dataset folder,
// so the boards below are hand-written clue layouts that pzprjs canonicalises.
"use strict";

module.exports = {
	pid: "island",
	dataset: null,
	urls: [
		"island/6/6/33k2h3q3g2j2l",
		"island/5/5/0h0g0",
		"island/7/7/3h2g5",
		"https://puzz.link/p?island/6/6/33k2h3q3g2j2l",
		"http://pzv.jp/p.html?type=player&island/6/6/33k2h3q3g2j2l",
		"island/6/6/33k2h3q3g2j2l/",
		"island/6/6/33k2h3q3g2j2l//",
		"island/6/6/33k2h3q3g2j2lzz",
		"island/6/6/33k2h3q3g2j2",
		// The largest clue is cell count - 1; 15 and 35 are hex "f" and "-23".
		"island/4/4/f",
		"island/6/6/-23",
		"island/2/2/-3",
		// `minnum` is 0, so zero clues are legal ("0" in hex is the digit 0).
		"island/3/3/000000000",
		"island/3/3/023456789",
		// `?` is a dot.
		"island/3/3/.",
		"island/3/3/.........",
		// Skipped cells (g-z), short bodies and unknown characters.
		"island/5/5/g1h2i3",
		"island/5/5/j",
		"island/5/5/zzzzz",
		"island/4/4/A",
		"island/6/6/2",
		// 1/1, 1/N and N/1 boards (1x1 has maxnum 0).
		"island/1/1/.",
		"island/1/1/0",
		"island/1/6/0",
		"island/6/1/0",
		"island/1/20/v",
		"island/20/1/v",
		// Empty bodies still canonicalise with a run-length marker.
		"island/4/4",
		"island/4/4/",
		"island/x/4/4/0"
	],
	fileios: [
		// Question and answer layers, including the legal zero clue.
		"pzprv3/island/3/3/0 3 . /. 0 . /. . 8 /. # . /# . # /. . .",
		// Question marks, two-digit clues and every answer token.
		"pzprv3/island/2/3/- 0 5 /. 3 . /+ # . /. . # /+ . .",
		// Single-cell boards.
		"pzprv3/island/1/1/0 /#",
		"pzprv3/island/1/2/. 1 /. ."
	],
	randomize(p, r) {
		const maxnum = Math.min(p.board.cols * p.board.rows - 1, 999);
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

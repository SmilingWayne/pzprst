// Fixture inputs for pzprjs pid "dominion". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "dominion",
	dataset: null,
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"dominion/13/17/k3m4q2i1k2y2k3zu1zh2zp3l4zm6m65v5n",
		"dominion/11/11/-18o-19j5g-1azn5g-1ag-12g1zzwei5i-17h",
		"dominion/6/6/m4h2g4g1l1m3g3j",
		"https://puzz.link/p?dominion/13/17/k3m4q2i1k2y2k3zu1zh2zp3l4zm6m65v5n",
		"http://pzv.jp/p.html?type=player&dominion/6/6/m4h2g4g1l1m3g3j",
		"dominion/6/6/m4h2g4g1l1m3g3j/",
		"dominion/6/6/m4h2g4g1l1m3g3j//",
		"dominion/6/6/m4h2g4g1l1m3g3jzz",
		"dominion/6/6/m4h2g4g1l1m3g3",
		// The largest clue is 52 ("-34" in hex) and the encoder tops at 52.
		"dominion/4/4/-34",
		"dominion/4/4/-35",
		"dominion/4/4/+100",
		"dominion/2/2/-3",
		// `?` is a dot and clues may be two digits in FileIO only.
		"dominion/3/3/.",
		"dominion/3/3/123456789",
		"dominion/3/3/.........",
		// Skipped cells (g-z), short bodies and unknown characters.
		"dominion/5/5/g1h2i3",
		"dominion/5/5/j",
		"dominion/5/5/zzzzz",
		"dominion/4/4/A",
		"dominion/6/6/4",
		// 1/1, 1/N and N/1 boards.
		"dominion/1/1/.",
		"dominion/1/1/4",
		"dominion/1/6/4",
		"dominion/6/1/4",
		"dominion/1/20/v",
		"dominion/20/1/v",
		// Empty bodies still canonicalise with a run-length marker.
		"dominion/4/4",
		"dominion/4/4/",
		"dominion/x/4/4/4"
	],
	fileios: [
		// Question and answer layers.
		"pzprv3/dominion/3/3/12 3 24 /. 4 . /. . 52 /. # . /# . # /. . .",
		// Question marks, two-digit clues and every answer token.
		"pzprv3/dominion/2/3/- 12 24 /. 3 . /+ # . /. . # /+ . .",
		// Single-cell boards.
		"pzprv3/dominion/1/1/52 /#",
		"pzprv3/dominion/1/2/. 2 /. ."
	],
	randomize(p, r) {
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 1; n <= 12; n++) {
			clues.push(n);
		}
		clues.push(24, 52);
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

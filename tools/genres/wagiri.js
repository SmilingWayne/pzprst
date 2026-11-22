// Fixture inputs for pzprjs pid "wagiri". See tools/gen_fixtures.js for the format.
// No Wagiri folder in ../puzzlekit-dataset/assets/data; the three puzz.link URLs
// are from the crawler log. The `d` flag is gokigen-only, so for wagiri it is an
// unknown flag (dropped) and the body is decoded as the normal `4Cross` layer.
"use strict";

module.exports = {
	pid: "wagiri",
	dataset: null,
	urls: [
		// Empty bodies, small and thin boards.
		"wagiri/7/7/",
		"wagiri/1/1/",
		"wagiri/2/2/",
		"wagiri/1/4/",
		"wagiri/4/1/",
		// A `?` cross, cross digits and base-36 skips, then the cell clue layer.
		"wagiri/2/2/.",
		"wagiri/2/2/5",
		"wagiri/2/2/a",
		"wagiri/1/1/....1",
		"wagiri/3/3/0123456789abcdef",
		// A truncated cross layer, trailing junk and an unknown flag.
		"wagiri/3/3/0g1",
		"wagiri/3/3/zz",
		"wagiri/d/2/2/123401234",
		// An editor URL keeps the same body.
		"wagiri_edit/3/3/0g1",
		// Canonical forms of the FileIO boards below.
		"wagiri/4/4/.01234.01234.01234.01234.12.b122.a1a21a2",
		"wagiri/2/2/1234.123e.12a",
		"wagiri/1/1/123e1",
		// Real puzz.link boards.
		"https://puzz.link/p?wagiri/7/7/ocg7b6166egckdgbsb6bn1f2f1h2b2h",
		"https://puzz.link/p?wagiri/7/7/pc2763bj8cgbmdn76bhbb1h2d1m2a2b1h1d",
		"https://puzz.link/p?wagiri/7/7/hbmcmc21ddvcr1g2d1h1a2e2i2h"
	],
	fileios: [
		// Cross clues (`?` and 0–4), cell clues (1/2/`?`/empty) and both slash
		// answers. (Wagiri has no config flags, so no flag line appears.)
		"pzprv3/wagiri/4/4/- 0 1 2 3 /4 - 0 1 2 /3 4 - 0 1 /2 3 4 - 0 /1 2 3 4 - /1 2 - . /. 1 2 2 /- . 1 . /2 1 . 2 /1 . 2 . /. 2 . 1 /1 . 2 . /. 1 . 2 /",
		// 2x2 with a corner cross and one slash.
		"pzprv3/wagiri/2/2/1 2 3 /4 - 1 /2 3 4 /- 1 /2 . /2 1 /. 2 /",
		// 1x1: a 2x2 cross grid, a cell clue and a slash.
		"pzprv3/wagiri/1/1/1 2 /3 4 /1 /1 /"
	],
	randomize(p, r) {
		const bd = p.board;
		// `Cross.maxnum` is 4 and `minnum` is 0; -2 is `?`, -1 empty.
		bd.cross.each((cross) => {
			cross.qnum = r.pick([-1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
		});
		bd.cell.each((cell) => {
			// The editor enters 1 (輪), 2 (切) and `?` (-2).
			cell.qnum = r.pick([-1, -1, -1, -1, 1, 2, -2]);
			const x = r.rnd();
			cell.qans = x < 0.35 ? 31 : x < 0.7 ? 32 : 0;
		});
	}
};

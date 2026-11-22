// Fixture inputs for pzprjs pid "bosnianroad". See tools/gen_fixtures.js for the format.
// No native puzz.link URLs for this pid exist in the dataset crawl, so the long URLs
// below were produced by pinned pzprjs itself (getURL of a filled board).
"use strict";

module.exports = {
	pid: "bosnianroad",
	dataset: null,
	urls: [
		// pzprjs-native canonical forms.
		"bosnianroad/5/5/a13a58a30a81a05a13a58a30a",
		"bosnianroad/8/6/a444a444a444a444a444a444a444a444a444a444a444a444",
		"bosnianroad/10/10/2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a5a2a3a",
		"https://puzz.link/p?bosnianroad/5/5/a13a58a30a81a05a13a58a30a",
		"http://pzv.jp/p.html?type=player&bosnianroad/5/5/a13a58a30a81a05a13a58a30a",
		"https://puzz.link/p?bosnianroad_edit/5/5/a13a58a30a81a05a13a58a30a",
		// One cell of every kind: clue 0, largest clue 8, out-of-range 9, ? and
		// empty.
		"bosnianroad/1/1/0",
		"bosnianroad/1/1/8",
		"bosnianroad/1/1/9",
		"bosnianroad/1/1/.",
		// Characters outside the number10 alphabet each consume one cell.
		"bosnianroad/3/3/--1",
		"bosnianroad/3/3/A",
		// Run lengths, truncated bodies, trailing junk, 1/N and N/1 boards.
		"bosnianroad/1/8/z1",
		"bosnianroad/8/1/z1",
		"bosnianroad/3/3/z",
		"bosnianroad/4/4/123",
		"bosnianroad/4/4/123zz",
		"bosnianroad/4/4",
		"bosnianroad/4/4/",
		"bosnianroad/2/2/zz"
	],
	fileios: [
		// Clue layer and shading layer with both answer tokens.
		"pzprv3/bosnianroad/3/3/1 2 3 /. . . /. 0 . /# . # /. . + /. . . /",
		// 2x2 board with the largest clue and an answer in every cell.
		"pzprv3/bosnianroad/2/2/8 0 /1 5 /# + /. # /",
		// Empty clue layer and a single answer mark.
		"pzprv3/bosnianroad/1/1/. /+ /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.45) {
				cell.qnum = -1;
			} else if (x < 0.58) {
				cell.qnum = -2;
			} else {
				cell.qnum = Math.floor(r.rnd() * 9);
			}
			const z = r.rnd();
			cell.qans = z < 0.25 ? 1 : 0;
			cell.qsub = z >= 0.25 && z < 0.4 ? 1 : 0;
		});
	}
};

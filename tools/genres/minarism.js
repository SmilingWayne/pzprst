// Fixture inputs for pzprjs pid "minarism". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "minarism",
	dataset: null,
	urls: [
		// Real puzz.link boards.
		"https://puzz.link/p?minarism/7/7/jhihmhi5ihhi5lho5l5lhi4i5i1i4vhi5i1jhphl",
		"https://puzz.link/p?minarism/7/7/ggi4ighh6o6g6i1hnhigggk4ihk1kgqhigqhh5ihogi3i",
		"minarism/4/4/gjhlhjgigpgi",
		"minarism_edit/4/4/gjhlhjgigpgi",
		"http://pzv.jp/p.html?type=player&minarism/4/4/gjhlhjgigpgi",
		// Unknown flag and trailing junk.
		"minarism/x/4/4/gjhlhjgigpgi",
		"minarism/4/4/gjhlhjgigpgi/",
		"minarism/4/4/gjhlhjgigpgi/zz",
		// `?` (`.`) and both hex escapes.
		"minarism/2/2/.",
		"minarism/2/2/-0f",
		"minarism/2/2/-ffk",
		// Inequality marks on a vertical and on a horizontal border.
		"minarism/2/1/g",
		"minarism/1/2/g",
		// 1/1, 1/N, N/1, blank runs and a truncated body.
		"minarism/1/1",
		"minarism/1/1/",
		"minarism/1/4/zn",
		"minarism/4/1/k",
		"minarism/5/5/zzl",
		"minarism/4/4/v",
		// Unknown character and empty body.
		"minarism/2/2/A",
		"minarism/4/4",
		"minarism/4/4/"
	],
	fileios: [
		// 2x2: a/b inequality marks, `?`, and answer numbers/marks.
		"pzprv3/minarism/2/2/a /b /0 . /1 c /+ 2 /",
		// 3x2: question marks plus every answer mark (`.`/c/`+`/`-`/`=`/`%`).
		"pzprv3/minarism/2/3/a 0 /0 0 /0 0 0 /. - . /2 3 c /",
		// 1x1: a single cell layer.
		"pzprv3/minarism/1/1/. /"
	],
	randomize(p, r) {
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qdir = 0;
			if (x < 0.12) {
				border.qdir = r.pick([0, 1, 2, 3, 4, 4]);
				if (border.qdir !== 0 && r.rnd() < 0.7) {
					border.qnum = -1;
				}
			} else if (x < 0.24) {
				border.qnum = -2;
			} else if (x < 0.8) {
				border.qnum = r.pick([-1, -1, 1, 2, 3, 5, 9, 15, 16, 100, 255, 300]);
			}
			if (r.rnd() < 0.2) {
				border.qnum = -1;
				border.qdir = 0;
			}
		});
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.anum = -1;
			cell.qcmp = 0;
			cell.qsub = 0;
			if (x < 0.2) {
				cell.anum = r.pick([0, 1, 2, 5]);
			} else if (x < 0.3) {
				cell.qcmp = 1;
			} else if (x < 0.5) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
	}
};

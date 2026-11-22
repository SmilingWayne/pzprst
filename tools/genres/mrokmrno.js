// Fixture inputs for pzprjs pid "mrokmrno". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "mrokmrno",
	dataset: null,
	urls: [
		"mrokmrno/4/4/123456.i",
		"mrokmrno/2/2/a.b",
		"mrokmrno/1/1/6",
		"mrokmrno/3/3/09.a3d",
		"mrokmrno/3/3/123456789",
		"https://puzz.link/p?mrokmrno/4/4/123456.i",
		"http://pzv.jp/p.html?type=player&mrokmrno/4/4/123456.i",
		"mrokmrno/z/4/4/123456.i",
		"mrokmrno/1/5/12345",
		"mrokmrno/5/1/12345",
		"mrokmrno/3/3/12",
		"mrokmrno/3/3/123456789zz",
		"mrokmrno/3/3/",
		"mrokmrno/2/2/AB12",
		"mrokmrno/2/2/..",
		"mrokmrno/2/2/0.",
		"mrokmrno/4/4/aaaa"
	],
	fileios: [
		"pzprv3/mrokmrno/4/4/1 2 3 4 /5 6 - . /. . . . /. . . . /. . . . /. . . 2 /-3 6 c + /- = % . /",
		"pzprv3/mrokmrno/2/2/. - /. . /. . /-3 5 /",
		"pzprv3/mrokmrno/1/1/6 /. /",
		"pzprv3/mrokmrno/3/3/0 9 - /. 3 . /. . . /3 . . /. . . /. . c /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -2, 1, 2, 3, 4, 5, 6]);
			const x = r.rnd();
			if (x < 0.1) {
				cell.anum = -3;
			} else if (x < 0.2) {
				cell.anum = r.pick([1, 2, 3, 4, 5, 6]);
			} else if (x < 0.3) {
				cell.qcmp = 1;
			} else if (x < 0.4) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
	}
};

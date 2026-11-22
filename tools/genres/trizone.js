// Fixture inputs for pzprjs pid "trizone". See tools/gen_fixtures.js for the format.
// No Trizone folder exists under ../puzzlekit-dataset/assets/data and
// .pzprst-samples.js finds no native-derived links, so the real puzzles come from
// pzprjs's own test data (test/script/trizone.js).
"use strict";

module.exports = {
	pid: "trizone",
	dataset: null,
	urls: [
		"trizone/6/6/22n2g2h4j1h2g1n00",
		"https://puzz.link/p?trizone/6/6/22n2g2h4j1h2g1n00",
		"http://pzv.jp/p.html?type=player&trizone/6/6/22n2g2h4j1h2g1n00",
		"trizone_edit/6/6/22n2g2h4j1h2g1n00",
		// The `g` (ghost) flag and an unknown flag.
		"trizone/g/6/6/22n2g2h4j1h2g1n00",
		"trizone/gx/6/6/22n2g2h4j1h2g1n00",
		"trizone/x/6/6/22n2g2h4j1h2g1n00",
		// Truncated body, trailing junk and a trailing slash.
		"trizone/6/6/22n2g2h4j",
		"trizone/6/6/22n2g2h4j1h2g1n00zz",
		"trizone/6/6/22n2g2h4j1h2g1n00/",
		// `?` is a dot; shaded cells come from the binary empty layer.
		"trizone/5/5/.",
		"trizone/5/5/g",
		"trizone/5/5/g1h2i3",
		"trizone/4/4/A",
		// maxnum is constant 6; zero is a legal clue (minnum 0).
		"trizone/6/6/666666666666666666666666666666666666",
		"trizone/4/4/0000000000000000",
		// 1/1, 1/N and N/1 boards.
		"trizone/1/1/.",
		"trizone/1/1/0",
		"trizone/1/5/00000",
		"trizone/5/1/00000",
		"trizone/1/20/v",
		"trizone/20/1/v",
		// Empty bodies still canonicalise with run-length markers.
		"trizone/4/4",
		"trizone/4/4/",
		"trizone/x/4/4/0"
	],
	fileios: [
		// The 6x6 solved pzprjs test state (question, border and cell layers).
		"pzprv3/trizone/6/6/2 2 . . . . /. . . . 2 . /2 . . 4 . . /. . 1 . . 2 /. 1 . . . . /. . . . 0 0 /1 0 0 1 0 /0 1 0 1 0 /0 0 0 1 0 /0 1 0 1 0 /0 1 0 1 0 /0 1 0 0 1 /0 1 0 1 0 0 /1 1 0 0 0 1 /0 0 1 0 0 0 /1 1 0 0 1 1 /0 0 1 1 1 0 /+ + + + # + /+ + # + + + /+ # + + # + /+ + + # + + /# + + + + + /+ + + + + + /",
		// The same 6x6 puzzle with the `g` config flag line.
		"pzprv3/trizone/6/6/g/2 2 . . . . /. . . . 2 . /2 . . 4 . . /. . 1 . . 2 /. 1 . . . . /. . . . 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /",
		// A 3x3 test state with a shaded cell, answer borders and answer cells.
		"pzprv3/trizone/3/3/6 . . /. . . /. . . /0 1 /0 0 /0 0 /0 0 0 /0 0 0 /+ + + /. # . /# . . /",
		// 1x1 has one empty inner-vertical layer and no horizontal row.
		"pzprv3/trizone/1/1/0 //. /"
	],
	randomize(p, r) {
		const ghost = r.rnd() < 0.3;
		const clues = [-1, -1, -1, -1, -2, 0, 1, 2, 3, 4, 5, 6];
		p.board.cell.each((cell) => {
			// A `#` token wins over the number, so never set both.
			if (r.rnd() < 0.15) {
				cell.ques = 7;
			} else {
				cell.qnum = r.pick(clues);
			}
			const x = r.rnd();
			if (x < 0.15) {
				cell.qans = 1;
			} else if (x < 0.3) {
				cell.qsub = 1;
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.qans = 1;
			} else if (x < 0.4) {
				border.qsub = 1;
			}
		});
		p.setConfig("trizone_ghost", ghost);
	}
};

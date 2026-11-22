// Fixture inputs for pzprjs pid "shwolf". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "shwolf",
	dataset: null,
	urls: [
		// Real puzzles, already canonical with pzprjs.
		"shwolf/10/10/312021122n24214220430331000616ab02912i3a699080k2111040",
		"shwolf/10/10/162304135b1013220416400106206900f239j3i1020930206906010",
		"shwolf/10/10/415c32640001151234349110c3166j0l209i090b04200303066309",
		"https://puzz.link/p?shwolf/10/10/312021122n24214220430331000616ab02912i3a699080k2111040",
		// pzprjs's own test URL.
		"shwolf/5/5/0282bocb6ajf9",
		// Cross marks: a run of all unmarked crosses, then one dot.
		"shwolf/3/3/4000",
		"shwolf/3/3/03000",
		// An unknown flag letter is ignored.
		"shwolf/x/3/3/4000",
		// Truncated bodies and trailing junk.
		"shwolf/3/3/4",
		"shwolf/3/3/4000zz",
		// The largest goat/wolf value 2.
		"shwolf/2/2/1qi",
		// 1x1, 1xN and Nx1 boards.
		"shwolf/1/1/00",
		"shwolf/1/2/400",
		"shwolf/2/1/00"
	],
	fileios: [
		// pzprjs's own answer-data fixture: cross dots, border lines and marks.
		"pzprv3/shwolf/5/5/1 . 2 2 2 /. 1 1 . 1 /. 2 . 2 . /1 . 1 2 . /1 1 2 . 1 /. . . . . . /. 1 . . 1 . /. . . . . . /. . . . . . /. 1 . . 1 . /. . . . . . /0 1 1 0 /1 1 1 0 /1 1 1 -1 /1 1 1 -1 /-1 1 1 -1 /0 1 1 1 1 /1 1 1 1 1 /-1 0 -1 -1 -1 /-1 1 1 1 1 /",
		// Question marks on a goat/wolf cell and a cross.
		"pzprv3/shwolf/2/2/2 . /. 1 /. 1 . /- . . /. . . /0 /1 /0 2 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qnum = x < 0.6 ? -1 : x < 0.85 ? r.pick([1, 2]) : -2;
		});
		p.board.cross.each((cross) => {
			const x = r.rnd();
			cross.qnum = x < 0.7 ? -1 : x < 0.8 ? -2 : r.pick([1, 2]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.qans = x < 0.25 ? 1 : 0;
			border.qsub = x >= 0.25 && x < 0.45 ? 1 : 0;
		});
	}
};

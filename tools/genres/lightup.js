// Fixture inputs for pzprjs pid "lightup". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "lightup",
	dataset: "Akari",
	urls: [
		// canonical pid / urlid / kanpen alias
		"lightup/3/3/10ej",
		"akari/3/3/10ej",
		"bijutsukan/3/3/10ej",
		"https://puzz.link/p?akari/3/3/10ej",
		// clue values 0-4, including the largest
		"lightup/4/4/444ep",
		"lightup/4/4/444eg",
		// question mark; the encoder appends "/" after a non-alphanumeric tail
		"lightup/1/1/.",
		"lightup/1/1/0",
		// 1/N and N/1 boards
		"lightup/1/5/01234",
		"lightup/5/1/01234",
		// truncated, trailing junk, run-length past the last cell
		"lightup/3/3/104",
		"lightup/3/3/10ejzz",
		"lightup/2/1/q",
		"lightup/10/10/0000",
		// empty body, unknown pflag
		"lightup/3/3/",
		"lightup/x/3/3/10ej",
		// real puzzles from ../puzzlekit-dataset/assets/data/Akari (janko problems
		// 71, 72 and 4), converted to pzpr FileIO and back through pzprjs.
		"akari/6/6/......0....3............1.......1.../",
		"akari/6/6/................2..3..1............./",
		"akari/18/10/.............1..2...........1.........4....3..............1.1......2...0.........1................1.........1...10.......2..............1....2......1...1....2....1...0............b"
	],
	fileios: [
		// question and answer data (lamp "#", sub mark "+", completion mark "-")
		"pzprv3/lightup/2/2/1 . /# + /- . /. . /",
		// every clue value incl. "?", plus both answer layers
		"pzprv3/lightup/3/3/0 1 2 /3 4 - /# + . /- . . /. . - /. . . /",
		// completion mark only
		"pzprv3/lightup/1/1/. /- /",
		// an all-empty completion layer is tolerated and dropped on re-encode
		"pzprv3/lightup/1/2/1 . /. . /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
			if (r.rnd() < 0.15) {
				cell.qans = 1;
			}
			if (r.rnd() < 0.08) {
				cell.qsub = 1;
			}
			if (r.rnd() < 0.1) {
				cell.qcmp = 1;
			}
		});
	}
};

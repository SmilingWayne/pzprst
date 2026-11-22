// Fixture inputs for pzprjs pid "lring". See tools/gen_fixtures.js for the format.
// lring shares variety/balance.js with balance but overrides Encode/FileIO, so
// it is a separate pid with its own inputs.
"use strict";

module.exports = {
	pid: "lring",
	dataset: null,
	urls: [
		// Hand-built 3x3: every direction and the `?`/empty cells.
		"lring/3/3/123401234",
		"lring/3/3/12340123e",
		"https://puzz.link/p?lring/3/3/123401234",
		"https://puzz.link/p?lring_edit/3/3/123401234",
		"http://pzv.jp/p.html?type=player&lring/3/3/123401234",
		// Rule flag alone, with an unknown flag, and on a 1x1 board.
		"lring/f/3/3/123401234",
		"lring/xf/3/3/123401234",
		"lring/f/1/1/4",
		// Largest allowed direction (4) on a 1x1 board and `?`.
		"lring/1/1/4",
		"lring/1/1/.",
		// `0` is decodable even though the UI starts at 1.
		"lring/1/1/0",
		// Base-36 run lengths, 1/N and N/1 boards.
		"lring/1/5/g",
		"lring/5/1/g",
		"lring/4/4/v",
		"lring/2/2/12",
		// Three-cell and two-cell compact forms of the 4Cell codec.
		"lring/4/1/1234",
		"lring/3/1/555",
		"lring/2/1/aa",
		// Truncated body, trailing junk, skipped characters and empty bodies.
		"lring/3/3/1234",
		"lring/3/3/123401234zz",
		"lring/3/3",
		"lring/3/3/"
	],
	fileios: [
		// 3x3 with directions, `?`, the config flag and answer lines.
		"pzprv3/lring/3/3/f/1 2 - /4 0 1 /2 3 4 /1 0 /0 -1 /1 1 /0 1 0 /1 0 0 /",
		// 2x2 with a direction 4 and every border token.
		"pzprv3/lring/2/2/4 - /0 3 /1 /0 /0 1 /",
		// 1x1 minimal file.
		"pzprv3/lring/1/1/4 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qnum = -1;
			cell.anum = -1;
			if (x < 0.4) {
				// empty
			} else if (x < 0.5) {
				cell.qnum = -2;
			} else {
				cell.qnum = Math.floor(r.rnd() * 5);
			}
			if (cell.qnum === -1 && r.rnd() < 0.1) {
				cell.anum = Math.floor(r.rnd() * 5);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

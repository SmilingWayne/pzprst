// Fixture inputs for pzprjs pid "compass". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "compass",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"compass/6/6/l....l333.l..33l.3..l3...g",
		"compass/9/9/p.96.k..8.zh5..1i...2h..37z0224i.4..o",
		"compass/7/7/n2111g3...g2..1o..2.g..11g..22o....g.3..g.120n",
		"https://puzz.link/p?compass/6/6/l....l333.l..33l.3..l3...g",
		// No pflags exist; an unknown flag is ignored like pzprjs.
		"compass/f/2/2/_",
		// Truncated body, and a non-number that ends the direction run.
		"compass/3/3/0",
		"compass/3/3/0_",
		// Any-height clue: the two-hex-digit form `-10` is 16, the largest
		// allowed value on a 4x4 board (`Cell.maxnum` = cols*rows).
		"compass/4/4/-10...o",
		// Four directions plus an absent direction (`.`).
		"compass/3/3/0123....g",
		// Blank run flushed at 20 cells, trailing junk.
		"compass/6/6/z",
		"compass/6/6/l....l333.l..33l.3..l3...gzz",
		// 1x1, 1xN and Nx1 boards; a shaded 1x1 cell.
		"compass/1/1/....",
		"compass/1/1/_",
		"compass/1/4/....___",
		"compass/4/1/....___",
		// Shaded cell followed by a blank run.
		"compass/2/2/_i"
	],
	fileios: [
		// Shaded, plain and compass cells, border answers of every kind.
		"pzprv3/compass/6/6/# . 1,2,3,4 . . . /-1,-1,-1,-1 . . . . . /. -1,3,3,3 . . . . /. . 3,-1,3,-1 . # . /. . . -1,3,-1,-1 . . /. . . . 0,-1,-1,9 . /0 1 0 0 0 /-1 2 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 1 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /2 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /",
		// Compass cells with an absent direction and a full one.
		"pzprv3/compass/3/3/. 0,1,2,3 # /. . . /. . -1,-1,-1,-1 /0 0 /1 0 /-1 0 /0 0 0 /2 0 0 /",
		// 1x1 shaded board.
		"pzprv3/compass/1/1/# /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 0, 7, 51, 51]);
			for (let dir = 1; dir <= 4; dir++) {
				cell.setQnumDir(dir, cell.ques === 51 ? r.pick([-1, -1, -1, 0, 1, 2, maxnum]) : -1);
			}
		});
		p.board.border.each((border) => {
			border.qans = r.pick([0, 0, 0, 0, 0, 1]);
			border.qsub = r.pick([0, 0, 0, 0, 0, 1]);
		});
	}
};

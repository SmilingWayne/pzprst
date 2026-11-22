// Fixture inputs for pzprjs pid "oyakodori". See tools/gen_fixtures.js for the format.
// kaero.js also serves kaero/armyants, which override Encode/FileIO and are separate
// pids. There is no Oyakodori folder under ../puzzlekit-dataset/assets/data.
"use strict";

module.exports = {
	pid: "oyakodori",
	dataset: null,
	urls: [
		// Real 4x4 puzzle from pzprjs test/script/oyakodori.js; canonicalises to itself.
		"oyakodori/4/4/2i0d080890096060",
		"https://puzz.link/p?oyakodori/4/4/2i0d080890096060",
		"http://pzv.jp/p.html?type=player&oyakodori/4/4/2i0d080890096060",
		"oyakodori_edit/4/4/2i0d080890096060",
		// Real puzz.link puzzles (native-derived, re-canonicalise unchanged).
		"https://puzz.link/p?oyakodori/5/5/25h20q40029203i10c01hg",
		"https://puzz.link/p?oyakodori/6/6/998888008000009060100k0301g10g60",
		// 1x1, 1xN and Nx1 boards.
		"oyakodori/1/1/1",
		"oyakodori/1/3/0",
		"oyakodori/3/1/0",
		// Truncated body, trailing junk, empty body.
		"oyakodori/4/4/2i0d",
		"oyakodori/4/4/2i0d080890096060zz",
		"oyakodori/3/3/",
		"oyakodori/6/6",
		// Circle layer (three base-3 values per character) after the border layer.
		"oyakodori/2/2/00d10",
		"oyakodori/2/2/00q10",
		// An invalid circle character decodes as an empty triple.
		"oyakodori/2/2/00z10",
		// Circles everywhere plus the shaded-nest layer.
		"oyakodori/3/3/0000qq0"
	],
	fileios: [
		// The real 4x4 puzzle, answered: nest walls and some path lines plus a cross.
		"pzprv3/oyakodori/4/4/. . 9 9 /4 4 3 . /. . . . /9 9 . 3 /0 0 0 /1 0 1 /0 0 1 /0 0 0 /0 1 1 0 /1 0 0 0 /0 0 0 1 /1 0 0 /0 1 0 /0 0 1 /0 0 0 /1 1 0 0 /0 0 1 0 /-1 0 0 0 /",
		// 2x2: shaded `?` (8), shaded circle 2 (12), circle 1 (3) and empty.
		"pzprv3/oyakodori/2/2/8 12 /3 . /1 /0 /0 1 /1 /0 /1 0 /",
		// 1x1 has no inner borders; two empty border layers only.
		"pzprv3/oyakodori/1/1/. ///"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.ques = r.rnd() < 0.25 ? 6 : 0;
			cell.qnum = r.pick([-2, -1, -1, -1, 0, 1, 2]);
		});
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.3 ? 1 : 0;
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

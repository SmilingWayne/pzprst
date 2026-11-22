// Fixture inputs for pzprjs pid "gokigen". See tools/gen_fixtures.js for the format.
// The `d` flag selects the old `decodecross_old` URL format; `encodePzpr` always
// writes the current `encode4Cross` format. The dataset folder GokigenNaname has
// problem texts but no `puzzlink_url` records.
"use strict";

module.exports = {
	pid: "gokigen",
	dataset: "GokigenNaname",
	urls: [
		"gokigen/4/4",
		"gokigen/1/1",
		"gokigen/1/4",
		"gokigen/4/1",
		"gokigen/2/2/123401234",
		"gokigen/2/2/.",
		"gokigen/2/2/5",
		"gokigen/2/2/a",
		"gokigen/2/2/01234",
		"gokigen/2/2/0!",
		"gokigen/2/2/1",
		"gokigen/3/3/0g1",
		"gokigen/3/3/z0",
		"gokigen/3/3/zz",
		"gokigen/3/3/0n/",
		"gokigen/1/1/4",
		"gokigen_edit/3/3/0g1",
		// The `d` flag selects the old `decodecross_old` format (digits only,
		// one cross per character; every other character is skipped).
		"gokigen/d/2/2/123401234",
		"gokigen/d/2/2/55a.01234",
		"gokigen/d/2/2/1",
		"gokigen/d/2/2/123401234zz",
		"gokigen/d/1/1/4",
		// A flag without a body is ignored, as in pzprjs.
		"gokigen/d/2/2",
		"gokigen/c/2/2/123401234",
		"gokigen/x/2/2/123401234",
		// Real puzz.link puzzles.
		"https://puzz.link/p?gokigen/7/7/gb6663cg6c28612bbgb8bg32cg77751ci",
		"https://puzz.link/p?gokigen/7/7/o22c263d1cd11cgcjc33chbhai1a",
		"https://puzz.link/p?gokigen/7/7/kbg718b4cdj77b3bc2d26262clb"
	],
	fileios: [
		// 2x2 with cross clues and both slash answers.
		"pzprv3/gokigen/2/2/1 2 3 /4 0 1 /2 3 4 /1 2 /. 1 /",
		// 3x3 with `?` on crosses and mixed cell answers.
		"pzprv3/gokigen/3/3/- . 0 1 /2 3 4 . /- . . . /. . . . /1 . 2 /. . 1 /2 . . /",
		// 1x1: a `?`/4 cross pair and a `/` answer.
		"pzprv3/gokigen/1/1/- 4 /0 1 /2 /",
		// One-row board (4x1): cross grid 5x2 and a single cell row.
		"pzprv3/gokigen/1/4/0 1 2 3 4 /- . 0 1 2 /1 2 . 1 /"
	],
	randomize(p, r) {
		const bd = p.board;
		bd.cell.each((cell) => {
			const x = r.rnd();
			cell.qans = x < 0.35 ? 31 : x < 0.7 ? 32 : 0;
		});
		// `Cross.maxnum` is 4 and `minnum` is 0; -2 is `?`, -1 empty.
		bd.cross.each((cross) => {
			cross.qnum = r.pick([-1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
		});
	}
};

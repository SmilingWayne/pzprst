// Fixture inputs for pzprjs pid "creek". See tools/gen_fixtures.js for the format.
// `../puzzlekit-dataset/assets/data/Creek` has problem texts but no
// `puzzlink_url` records, and `.pzprst-samples.js creek` finds no crawled URL,
// so the large boards below were canonicalised by pzprjs from the dataset
// cross layers (9x9 and two 10x10 puzzles).
"use strict";

module.exports = {
	pid: "creek",
	dataset: "Creek",
	urls: [
		"creek/4/4",
		"creek/1/1",
		"creek/1/4",
		"creek/4/1",
		"creek/2/2/123401234",
		"creek/2/2/.",
		"creek/2/2/5",
		"creek/2/2/a",
		"creek/2/2/01234",
		"creek/2/2/0!",
		"creek/3/3/0g1",
		"creek/3/3/0h1",
		"creek/3/3/0i1",
		"creek/6/5/z0",
		"creek/6/5/zz",
		"creek/3/3/0zz",
		"creek/3/3/0n/",
		"creek_edit/3/3/0g1",
		"creek/c/2/2/123401234",
		"creek/x/2/2/123401234",
		// The `d` flag selects the old `decodecross_old` format (digits only,
		// one cross per character; new-format markers are ignored).
		"creek/d/2/2/123401234",
		"creek/d/2/2/55a.01234",
		"creek/d/2/2/1",
		"creek/d/2/2/123401234zz",
		// A flag without a body is ignored, as in pzprjs.
		"creek/d/2/2",
		// Dataset puzzles.
		"https://puzz.link/p?creek/9/9/mb62b6ca376bgeg2cc76bje7b27ci61bc33b8ci6bg",
		"https://puzz.link/p?creek/10/10/l6cgagdgbgco5736cdcdpae827br32cj3biej7aga",
		"https://puzz.link/p?creek/10/10/r2c3cbidg3bgccgdh8322doc282cgdhcg7dgddbg8dg7cp"
	],
	fileios: [
		// 2x2: every cross clue mode plus shaded and background-marked cells.
		"pzprv3/creek/2/2/1 2 3 /4 0 1 /2 3 4 /. # /. + /",
		// 3x3 with `?` on a cross and mixed cell answers.
		"pzprv3/creek/3/3/0 1 2 3 /4 - . 0 /1 2 3 4 /. 0 1 - /# . + /# . . /. + # /",
		// 1x1: a `?` cross and a shaded cell on the frame cross grid.
		"pzprv3/creek/1/1/- 0 /1 . /# /",
		// One-row board (4x1): cross grid 2x5 and a single cell row.
		"pzprv3/creek/1/4/0 1 2 3 4 /- . 0 1 2 /# . + . /"
	],
	randomize(p, r) {
		const bd = p.board;
		bd.cell.each((cell) => {
			cell.qnum = -1;
			cell.ques = 0;
			cell.qans = 0;
			cell.qsub = 0;
			const x = r.rnd();
			if (x < 0.35) {
				cell.qans = 1;
			} else if (x < 0.55) {
				cell.qsub = 1;
			}
		});
		// `Cross.maxnum` is 4 and `minnum` is 0; -2 is `?`, -1 empty.
		bd.cross.each((cross) => {
			cross.qnum = r.pick([-1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
		});
	}
};

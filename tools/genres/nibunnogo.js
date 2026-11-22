// Fixture inputs for pzprjs pid "nibunnogo". See tools/gen_fixtures.js for the format.
// `nibunnogo` shares variety/lightshadow.js but overrides `Encode`/`FileIO`:
// the URL is `decode4Cross` only and FileIO stores cross numbers plus the
// three-state cell answer. There is no dataset folder and `.pzprst-samples.js
// nibunnogo` finds no crawled puzz.link URL, so every input below is hand-made.
"use strict";

module.exports = {
	pid: "nibunnogo",
	dataset: null,
	urls: [
		"nibunnogo/4/4",
		"nibunnogo/1/1",
		"nibunnogo/1/4",
		"nibunnogo/4/1",
		"nibunnogo/2/2/123401234",
		"nibunnogo/2/2/.",
		"nibunnogo/2/2/5",
		"nibunnogo/2/2/a",
		"nibunnogo/2/2/01234",
		"nibunnogo/2/2/0!",
		"nibunnogo/2/2/1",
		"nibunnogo/3/3/0g1",
		"nibunnogo/3/3/z0",
		"nibunnogo/3/3/zz",
		"nibunnogo/3/3/0n/",
		"nibunnogo/1/1/4",
		"nibunnogo_edit/3/3/0g1",
		// Unknown flags are ignored, as in pzprjs; a flag without a body too.
		"nibunnogo/c/2/2/123401234",
		"nibunnogo/x/2/2/123401234",
		"nibunnogo/c/2/2"
	],
	fileios: [
		// 2x2 with cross clues and every answer state (empty / shaded / dot).
		"pzprv3/nibunnogo/2/2/1 2 3 /4 0 1 /2 3 4 /0 1 /2 0 /",
		// 3x3 with `?` on crosses and mixed answers.
		"pzprv3/nibunnogo/3/3/- . 0 1 /2 3 4 . /- . . . /. . . . /0 1 2 /2 0 1 /1 2 0 /",
		// 1x1: a `?`/4 cross pair and a dot answer.
		"pzprv3/nibunnogo/1/1/- 4 /0 1 /2 /",
		// One-row board (4x1): cross grid 5x2 and a single answer row.
		"pzprv3/nibunnogo/1/4/0 1 2 3 4 /- . 0 1 2 /1 2 0 1 /"
	],
	randomize(p, r) {
		const bd = p.board;
		bd.cell.each((cell) => {
			cell.qans = r.pick([0, 0, 0, 1, 1, 2]);
		});
		// `Cross@nibunnogo.maxnum` is 4 and `minnum` is 0; -2 is `?`, -1 empty.
		bd.cross.each((cross) => {
			cross.qnum = r.pick([-1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, -2]);
		});
	}
};

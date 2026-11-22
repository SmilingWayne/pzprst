// Fixture inputs for pzprjs pid "hamlepaths". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "hamlepaths",
	dataset: null,
	urls: [
		"hamlepaths/4/4/1u",
		"https://puzz.link/p?hamlepaths/4/4/1u",
		"http://pzv.jp/p.html?type=player&hamlepaths/4/4/1u",
		"hamlepaths/z/4/4/1u",
		"hamlepaths/4/4/1",
		"hamlepaths/4/4/1uzz",
		"hamlepaths/4/4/.u",
		"hamlepaths/7/7/6zzn",
		"hamlepaths/20/20/-13",
		"hamlepaths/1/1/g",
		"hamlepaths/1/5/1j",
		"hamlepaths/5/1/1j",
		"hamlepaths/4/4/"
	],
	fileios: [
		"pzprv3/hamlepaths/4/4/1 . . . /. . . . /. . . . /. . . . /0 0 0 17 /32 0 0 0 /0 33 0 0 /0 0 0 48 /1 0 0 /0 -1 0 /0 0 0 /0 0 0 /0 0 0 0 /0 0 0 0 /-1 0 0 0 /",
		"pzprv3/hamlepaths/3/2/1 . /. . /. . /0 64 /0 0 /16 0 /1 /0 /0 /-1 0 /0 0 /",
		"pzprv3/hamlepaths/1/5/1 . . . 3 /0 0 0 0 32 /1 0 -1 0 /"
	],
	randomize(p, r) {
		const cols = p.board.cols;
		const rows = p.board.rows;
		p.board.cell.each((cell) => {
			const c = cell.bx >> 1;
			const d = cell.by >> 1;
			const maxnum = Math.max(c, cols - 1 - c, d, rows - 1 - d);
			cell.qnum = r.pick([-1, -1, -1, -1, -2, 0, Math.min(1, maxnum), maxnum]);
			cell.qsub = r.pick([0, 0, 0, 0, 1, 2, 15]);
			cell.qcmp = r.pick([0, 0, 0, 1]);
			cell.anum = r.pick([-1, -1, -1, 0]);
		});
		p.board.border.each((border) => {
			const y = r.rnd();
			border.line = y < 0.3 ? 1 : 0;
			border.qsub = y >= 0.3 && y < 0.45 ? 2 : 0;
		});
	}
};

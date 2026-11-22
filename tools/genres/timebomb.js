// Fixture inputs for pzprjs pid "timebomb". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "timebomb",
	dataset: null,
	urls: [
		"timebomb/4/4/fu",
		"https://puzz.link/p?timebomb/4/4/fu",
		"http://pzv.jp/p.html?type=player&timebomb/4/4/fu",
		"timebomb/z/4/4/1u",
		"timebomb/4/4/",
		"timebomb/4/4/1",
		"timebomb/4/4/1uzz",
		"timebomb/4/4/.u",
		"timebomb/40/40/+3e7",
		"timebomb/40/40/-ff",
		"timebomb/1/1/g",
		"timebomb/1/5/1j",
		"timebomb/5/1/1j"
	],
	fileios: [
		"pzprv3/timebomb/4/4/15 . . . /. . . . /. . . . /. . . . /0 0 0 17 /32 0 0 0 /0 33 0 0 /0 0 0 48 /1 0 0 /0 -1 0 /0 0 0 /0 0 0 /0 0 0 0 /0 0 0 0 /-1 0 0 0 /",
		"pzprv3/timebomb/3/2/1 . /. . /. . /0 64 /0 0 /16 0 /1 /0 /0 /-1 0 /0 0 /",
		"pzprv3/timebomb/1/5/1 . . . 3 /0 0 0 0 32 /1 0 -1 0 /"
	],
	randomize(p, r) {
		const maxnum = Math.min(p.board.cell.length - 1, 999);
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -2, 0, 1, maxnum]);
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

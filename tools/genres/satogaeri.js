// Fixture inputs for pzprjs pid "satogaeri". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "satogaeri",
	dataset: null,
	urls: [
		"satogaeri/4/4/1g0000v",
		"sato/4/4/000000v",
		"https://puzz.link/p?sato/4/4/000000v",
		"http://pzv.jp/p.html?type=player&satogaeri/4/4/1g0000v",
		"satogaeri/4/4/",
		"satogaeri/4/4/1",
		"satogaeri/4/4/1g0000vzz",
		"satogaeri/4/4/000000.u",
		"satogaeri/7/7/0000000000000000006zzn",
		"satogaeri/1/1/g",
		"satogaeri/1/5/1j",
		"satogaeri/5/1/1j",
		"satogaeri/z/4/4/1g0000v"
	],
	fileios: [
		"pzprv3/satogaeri/4/4/. . . . /. . . . /. . . . /. . . . /0 0 0 17 /32 0 0 0 /0 33 0 0 /0 0 0 48 /0 0 1 /0 1 1 /0 0 0 /0 0 0 /0 0 0 0 /0 0 1 0 /0 0 0 0 /1 0 0 /0 -1 0 /0 0 0 /0 0 0 /0 0 0 0 /0 0 0 0 /-1 0 0 0 /",
		"pzprv3/satogaeri/3/2/0 0 /0 0 /0 1 /0 64 /0 0 /16 0 /1 /0 /0 /0 0 /0 0 /1 /0 /0 /-1 0 /0 0 /"
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
			const x = r.rnd();
			border.ques = x < 0.25 ? 1 : 0;
			const y = r.rnd();
			border.line = y < 0.25 ? 1 : 0;
			border.qsub = y >= 0.25 && y < 0.4 ? 2 : 0;
		});
	}
};

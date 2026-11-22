// Fixture inputs for pzprjs pid "heyabon". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "heyabon",
	dataset: null,
	urls: [
		"https://puzz.link/p?heyabon/10/10/24488gl5aakl12244800001su0000f7g0000n0k.0i.i3h.h0i.i1.k2p.k2.i3i.h3h.i3i.1k.n",
		"https://puzz.link/p?heyabon/5/5/dtkmhlaa1i2g.g.i.i.g.g1i2",
		"https://puzz.link/p?heyabon/8/8/518a2gk518a00001u07o00005l3g1i3g.q..j.g..o4h.l1l5",
		"http://pzv.jp/p.html?type=player&heyabon/4/4/1g0000v",
		"heyabon/4/4/1g0000v",
		"heyabon/4/4/",
		"heyabon/4/4/1",
		"heyabon/4/4/1g0000vzz",
		"heyabon/4/4/000000.u",
		"heyabon/7/7/0000000000000000006zzn",
		"heyabon/1/1/g",
		"heyabon/1/5/1j",
		"heyabon/5/1/1j",
		"heyabon/z/4/4/1g0000v"
	],
	fileios: [
		"pzprv3/heyabon/4/4/. . . . /. . . . /. . . . /. . . . /0 0 0 17 /32 0 0 0 /0 33 0 0 /0 0 0 48 /0 0 1 /0 1 1 /0 0 0 /0 0 0 /0 0 0 0 /0 0 1 0 /0 0 0 0 /1 0 0 /0 -1 0 /0 0 0 /0 0 0 /0 0 0 0 /0 0 0 0 /-1 0 0 0 /",
		"pzprv3/heyabon/3/2/0 0 /0 0 /0 1 /0 64 /0 0 /16 0 /1 /0 /0 /0 0 /0 0 /1 /0 /0 /-1 0 /0 0 /"
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

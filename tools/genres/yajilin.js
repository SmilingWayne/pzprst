// Fixture inputs for pzprjs pid "yajilin". See tools/gen_fixtures.js for the format.
// yajilin shares variety/yajilin.js with lixloop. Its `b` URL flag (display
// type) is not stored in FileIO, so it is not a FileIO fixture input here: the
// fixture transforms rebuild the board from FileIO and pzprjs itself drops it.
"use strict";

module.exports = {
	pid: "yajilin",
	dataset: "Yajilin",
	urls: [
		// Hand-built 3x3: every direction, numbers 0-8, `?` and `+`.
		"yajilin/3/3/101112131415161718",
		"https://puzz.link/p?yajilin/3/3/101112131415161718",
		"https://puzz.link/p?yajilin_edit/3/3/101112131415161718",
		"http://pzv.jp/p.html?type=player&yajilin/3/3/101112131415161718",
		// The `yajirin` alias.
		"https://puzz.link/p?yajirin/3/3/101112131415161718",
		// The `o` config flag and an unknown flag. The `b` display flag is URL
		// only (FileIO drops it), so a `b` input would fail the FileIO-derived
		// transform fixtures; it is not a fixture input (same as lixloop).
		"yajilin/o/3/3/101112131415161718",
		"yajilin/x/3/3/101112131415161718",
		// `?`, `+` and the largest URL number (4095).
		"yajilin/3/3/1.+",
		"yajilin/1/1/-1fff",
		// Two- and three-character numbers.
		"yajilin/2/2/6107ff-12004.",
		"yajilin/1/1/510",
		// 1x1, 1/N and N/1 boards.
		"yajilin/1/1/00",
		"yajilin/1/5/g",
		"yajilin/5/1/g",
		// Base-36 run lengths.
		"yajilin/4/4/v",
		// Truncated body, trailing junk and empty bodies.
		"yajilin/3/3/1011",
		"yajilin/3/3/101112131415161718zz",
		"yajilin/3/3",
		"yajilin/3/3/",
		// Real puzzle URLs (puzz.link).
		"https://puzz.link/p?yajilin/17/17/g42e23h32j24zd32s33b12b23b13b22j23b13b22j23b13b22e42d23b13b22j23b13b22d32b12b23b13b22j23b13b42zd33l14z12b13h32g",
		"https://puzz.link/p?yajilin/17/17/zg1140b2324s42z10d41a40i10d31k10d42p21j32zj20b21a20g11a11h1511141115c40v12a20e31zd15a16c",
		"https://puzz.link/p?yajilin/19/13/n32l11g31za30d22a32f11a12e22y12o33p22c11f14b32f21zd33a33u32a32b14u"
	],
	fileios: [
		// 3x3 with clues, `?`, `+`, the `o` config line, answer marks and lines.
		"pzprv3/yajilin/3/3/o/1,0 1,1 1,2 /1,3 1,4 1,5 /1,6 1,7 1,8 /# + . /. # . /. . . /1 0 /0 1 /0 0 /1 0 0 /0 0 0 /",
		// 2x2 with every number range, `?`, answer marks and a cross border.
		"pzprv3/yajilin/2/2/1,16 2,255 /1,512 4,- /# . /. + /1 0 /0 -1 /",
		// 1x1: no inner borders, so the border layer is empty.
		"pzprv3/yajilin/1/1/0,0 /./"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows) >> 1;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qdir = 0;
			if (x < 0.4) {
				cell.qnum = -1;
			} else if (x < 0.5) {
				cell.qnum = -2;
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			} else if (x < 0.55) {
				cell.qnum = -3;
			} else {
				cell.qnum = Math.floor(r.rnd() * (maxnum + 1));
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			}
			const a = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (a < 0.2) {
				cell.qans = 1;
			} else if (a < 0.35) {
				cell.qsub = 1;
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
		p.setConfig("yajilin_out", r.rnd() < 0.3);
	}
};

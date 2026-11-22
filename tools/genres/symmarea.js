// Fixture inputs for pzprjs pid "symmarea". See tools/gen_fixtures.js for the format.
// The other pids sharing variety/fillomino.js override Encode/FileIO and are not
// covered here.
"use strict";

module.exports = {
	pid: "symmarea",
	dataset: null,
	urls: [
		// Real puzzles, verified as already-canonical with tools/pzprjs.js.
		"symmarea/10/10/va5j2h3k5h3l1h5g3l8g5h9l6h2k3h8j57v",
		"symmarea/10/10/3141592653zp9x2zp5827312384",
		"symmarea/10/10/5o2g3mag5g8lah7g7j5g7h2g1hag5h6g6j2g6halbg1g2m5g1o3",
		"https://puzz.link/p?symmarea/10/10/va5j2h3k5h3l1h5g3l8g5h9l6h2k3h8j57v",
		"https://puzz.link/p?symmarea_edit/10/10/3141592653zp9x2zp5827312384",
		"http://pzv.jp/p.html?type=player&symmarea/10/10/5o2g3mag5g8lah7g7j5g7h2g1hag5h6g6j2g6halbg1g2m5g1o3",
		// Border walls are stored in a second URL layer; an all-empty layer is
		// dropped by encodeBorderIfPresent.
		"symmarea/4/4/12232334344g4ii00o00",
		// The only pflag is `t` (fillomino_tri).
		"symmarea/t/3/3/12g34i94000",
		"symmarea/t/4/4/12232334344g4ii00o00",
		// maxnum is cols*rows: 16 on a 4x4 board.
		"symmarea/4/4/16",
		"symmarea/4/4",
		"symmarea/4/4/",
		"symmarea/4/4/.",
		"symmarea/x/4/4/1",
		"symmarea/1/1/1",
		"symmarea/1/1/.",
		"symmarea/1/5/1",
		"symmarea/5/1/1",
		"symmarea/2/2/-10",
		"symmarea/4/4/zz",
		"symmarea/3/3/A",
		"symmarea/2/2/+100",
		"symmarea/2/2/=001",
		"symmarea/2/2/@001",
		"symmarea/2/2/*0001",
		"symmarea/2/2/$00001",
		"symmarea/30/1/zzk",
		"symmarea/1/30/zp"
	],
	fileios: [
		// Question numbers, walls, and every answer token: anum, qsub, qcmp,
		// border qans and the -X wall/cross combination.
		"pzprv3/symmarea/4/4/1 2 2 3 /2 3 3 4 /3 4 4 . /4 . . . /2 . . . /. - . . /. . c . /. . . . /X 0 0 /X 0 0 /0 0 0 /0 0 0 /X -X 0 0 /0 0 0 0 /0 0 0 0 /",
		// Config flag line plus answer data.
		"pzprv3/symmarea/3/3/t/1 2 . /3 4 . /. . 9 /+ . . /. . . /. . . /0 0 /-X 0 /0 0 /0 0 0 /0 0 0 /"
	],
	randomize(p, r) {
		p.setConfig("fillomino_tri", r.rnd() < 0.3);
		p.board.border.each((border) => {
			border.ques = 0;
			border.qans = 0;
			border.qsub = 0;
			const x = r.rnd();
			if (x < 0.2) {
				border.ques = 1;
				border.qsub = r.rnd() < 0.3 ? 1 : 0;
			} else if (x < 0.35) {
				border.qans = 1;
				border.qsub = r.rnd() < 0.5 ? 1 : 0;
			} else if (x < 0.45) {
				border.qsub = 1;
			}
		});
		p.board.cell.each((cell) => {
			const maxnum = cell.getmaxnum();
			if (r.rnd() < 0.55) {
				cell.qnum = -1;
			} else {
				cell.qnum = r.pick(Array.from({ length: maxnum }, (_, i) => i + 1));
			}
			const a = r.rnd();
			if (a < 0.12) {
				cell.anum = 1 + Math.floor(r.rnd() * maxnum);
			} else if (a < 0.18) {
				cell.qcmp = 1;
			} else if (a < 0.3) {
				cell.qsub = 1 + Math.floor(r.rnd() * 4);
			}
		});
	}
};

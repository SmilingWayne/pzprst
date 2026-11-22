// Fixture inputs for pzprjs pid "numcity". See tools/gen_fixtures.js for the format.
// The other pids sharing variety/fillomino.js override Encode/FileIO and are not
// covered here.
"use strict";

module.exports = {
	pid: "numcity",
	dataset: null,
	urls: [
		// A 5x5 board split into two rooms by a vertical wall, numbers valid for
		// each room's size (maxnum 4 and 5).
		"numcity/5/5/12233213433243443545445558h240000",
		"https://puzz.link/p?numcity/5/5/12233213433243443545445558h240000",
		"https://puzz.link/p?numcity_edit/5/5/12233213433243443545445558h240000",
		"http://pzv.jp/p.html?type=player&numcity/5/5/12233213433243443545445558h240000",
		// The only pflag is `t` (fillomino_tri).
		"numcity/t/5/5/12233213433243443545445558h240000",
		"numcity/t/4/4/12232334344g4i140000",
		// Border walls are stored in a second URL layer; an all-empty layer is
		// dropped by encodeBorderIfPresent.
		"numcity/4/4/12232334344g4i140000",
		"numcity/3/3/12223333g",
		"numcity/4/4/5",
		"numcity/4/4",
		"numcity/4/4/",
		"numcity/x/4/4/1",
		"numcity/1/1/1",
		"numcity/1/1/.",
		"numcity/1/5/12233",
		"numcity/5/1/12233",
		// 2 border characters where 8 are expected.
		"numcity/5/5/12233213433243443545445558h",
		"numcity/3/3/12223333g0000zz",
		"numcity/2/2/-19",
		"numcity/2/2/+100",
		"numcity/2/2/7",
		"numcity/4/4/zz",
		"numcity/3/3/A",
		"numcity/2/2/=001",
		"numcity/2/2/@001",
		"numcity/2/2/*0001",
		"numcity/2/2/$00001",
		"numcity/30/1/zzk",
		"numcity/1/30/zp"
	],
	fileios: [
		// Question numbers, walls, and every answer token: anum, qsub, qcmp,
		// border qans and the -X wall/cross combination.
		"pzprv3/numcity/5/5/1 2 2 3 3 /2 1 3 4 3 /3 2 4 3 4 /4 3 5 4 5 /4 4 5 5 5 /3 - c . . /. . . . . /. . . . . /. . . . . /. . . . . /0 -X 0 0 /0 X 0 0 /0 X 0 0 /0 X 0 0 /0 X 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /",
		"pzprv3/numcity/3/3/1 2 2 /2 3 3 /3 3 . /. . . /2 - . /. . c /X 0 /X 0 /0 0 /0 -X 0 /0 0 0 /",
		// Config flag line plus answer data.
		"pzprv3/numcity/3/2/t/1 2 /. 3 /. . /3 c /. . /. . /0 /X /0 /0 0 /-1 0 /",
		"pzprv3/numcity/1/2/1 2 /= . /-X /"
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
		// The room split decides each cell's maxnum.
		p.board.roommgr.rebuild();
		p.board.cell.each((cell) => {
			const maxnum = cell.getmaxnum();
			if (r.rnd() < 0.55) {
				cell.qnum = -1;
			} else {
				cell.qnum = r.pick(Array.from({ length: maxnum }, (_, i) => i + 1));
			}
			const a = r.rnd();
			if (a < 0.12) {
				cell.anum = 1 + Math.floor(r.rnd() * 4);
			} else if (a < 0.18) {
				cell.qcmp = 1;
			} else if (a < 0.3) {
				cell.qsub = 1 + Math.floor(r.rnd() * 4);
			}
		});
	}
};

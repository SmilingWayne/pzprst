// Fixture inputs for pzprjs pid "kurodoko". See tools/gen_fixtures.js for the format.
// The other pids sharing variety/kurodoko.js (cave, teri, cityspace, nurimisaki) store
// different border layers and are not covered here.
"use strict";

module.exports = {
	pid: "kurodoko",
	dataset: "Kuromasu",
	urls: [
		// Real puzzles, verified as already-canonical with tools/pzprjs.js.
		"kurodoko/9/9/h2i64k2l8k6o5h4g4h3o.k4l.k8.i.h",
		"kurodoko/9/9/g.h4i.h2h5o5h5r5r2h5och4h3i3h9g",
		"kurodoko/9/9/k2g.nag8i6t45i8at2i.g5n4g4k",
		"https://puzz.link/p?kurodoko/9/9/h2i64k2l8k6o5h4g4h3o.k4l.k8.i.h",
		"https://puzz.link/p?kurodoko_edit/9/9/g.h4i.h2h5o5h5r5r2h5och4h3i3h9g",
		"http://pzv.jp/p.html?type=player&kurodoko/9/9/k2g.nag8i6t45i8at2i.g5n4g4k",
		// A smaller realistic board and the largest clue on a 4x4 board.
		"kurodoko/5/5/3h4h2h5k6h2h4h3",
		"kurodoko/4/4/7",
		// Truncated and trailing-junk versions of a real puzzle.
		"kurodoko/9/9/h2i64k2l8k6o5h4g4h3o.k4l.k8.i.hzz",
		"kurodoko/9/9/h2i64k2l8k6o5h4g4h3o.k4l.k8.i",
		"kurodoko/4/4",
		"kurodoko/4/4/",
		"kurodoko/4/4/.",
		"kurodoko/x/4/4/1",
		"kurodoko/1/1/2",
		"kurodoko/1/1/.",
		"kurodoko/1/5/2",
		"kurodoko/5/1/2",
		"kurodoko/2/2/-19",
		"kurodoko/2/2/+100",
		"kurodoko/3/3/A",
		"kurodoko/3/3/zz",
		"kurodoko/2/2/=001",
		"kurodoko/2/2/@001",
		"kurodoko/2/2/*0001",
		"kurodoko/2/2/$00001",
		"kurodoko/30/1/zzk",
		"kurodoko/1/30/zp"
	],
	fileios: [
		// Question numbers, both cell answer marks (# shaded, + background) and
		// both border marks (1 line, -1 cross).
		"pzprv3/kurodoko/5/5/3 . . 4 . /. 2 . . 5 /. . . . . /6 . . 2 . /. 4 . . 3 /# . . . . /. + . . . /. . # . . /. . . . . /. . . . . /1 0 0 0 /-1 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 1 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /",
		"pzprv3/kurodoko/3/2/2 . /3 . /4 . /# + /. . /. . /-1 /0 /0 /0 0 /0 0 /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols + p.board.rows - 1;
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 2; n <= Math.min(maxnum, 8); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			const a = r.rnd();
			if (a < 0.15) {
				cell.qans = 1;
			} else if (a < 0.3) {
				cell.qsub = 1;
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.15) {
				border.line = 1;
			} else if (x < 0.25) {
				border.qsub = 2;
			}
		});
	}
};

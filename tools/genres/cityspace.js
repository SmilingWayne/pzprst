// Fixture inputs for pzprjs pid "cityspace". See tools/gen_fixtures.js for the format.
// The other pids sharing variety/kurodoko.js (kurodoko, cave, teri, nurimisaki) store
// border marks and are not covered here.
"use strict";

module.exports = {
	pid: "cityspace",
	dataset: null,
	urls: [
		// Realistic boards (no native-derived puzz.link sample exists for cityspace).
		"cityspace/5/5/3h4h2h5k6h2h4h3",
		"cityspace/9/9/34h5g6g7h4h5h82i6h4h5g3h7j6h4h37h5h8i4h3h28h6g3h5p",
		"https://puzz.link/p?cityspace/5/5/3h4h2h5k6h2h4h3",
		"https://puzz.link/p?cityspace_edit/5/5/3h4h2h5k6h2h4h3",
		"http://pzv.jp/p.html?type=player&cityspace/5/5/3h4h2h5k6h2h4h3",
		"cityspace/5/5/3h4h2h5k6h2h4h3zz",
		"cityspace/5/5/3h4h2h5k6h2h4",
		// maxnum is cols+rows-1 and minnum is 2.
		"cityspace/4/4/7",
		"cityspace/5/5/9",
		"cityspace/4/4",
		"cityspace/4/4/",
		"cityspace/4/4/.",
		"cityspace/x/4/4/1",
		"cityspace/1/1/2",
		"cityspace/1/1/.",
		"cityspace/1/5/2",
		"cityspace/5/1/2",
		"cityspace/2/2/-19",
		"cityspace/2/2/+100",
		"cityspace/3/3/A",
		"cityspace/3/3/zz",
		"cityspace/2/2/=001",
		"cityspace/2/2/@001",
		"cityspace/2/2/*0001",
		"cityspace/2/2/$00001",
		"cityspace/30/1/zzk",
		"cityspace/1/30/zp"
	],
	fileios: [
		// Question numbers and both answer marks (# shaded, + background).
		"pzprv3/cityspace/5/5/3 . . 4 . /. 2 . . 5 /. . . . . /6 . . 2 . /. 4 . . 3 /# . . . . /. . . . . /. . # . . /. . . . . /. . . . . /",
		"pzprv3/cityspace/3/2/2 . /3 . /4 . /# + /# . /. . /",
		"pzprv3/cityspace/1/1/2 /# /"
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
	}
};

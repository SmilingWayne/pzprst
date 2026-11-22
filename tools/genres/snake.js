// Fixture inputs for pzprjs pid "snake". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "snake",
	dataset: "Snake",
	urls: [
		"snake/4/4/000000n",
		"snake/3/3/dd0l",
		"snake/3/3/qq0l",
		"snake/6/5/000000000012345g0000g",
		"snake/3/3/00012j",
		"snake/3/3/000.01i",
		"snake/3/3/000-0f",
		"snake/4/4/000",
		"snake/4/4/000000nzz",
		"snake/3/3/",
		"snake/3/3/000Z",
		"snake/1/1/01g",
		"snake/1/4/0000g00",
		"snake/4/1/001j",
		"https://puzz.link/p?snake/5/5/220101300i2j2g",
		"https://puzz.link/p?snake/9/9/j3a000001000a31000931000i3ao7g5k6",
		"https://puzz.link/p?snake/7/7/00000900i009000006g2g1g6k4g",
		"http://pzv.jp/p.html?type=player&snake/3/3/000l"
	],
	fileios: [
		"pzprv3/snake/3/4/. 2 1 3 . /. 0 4 8 0 /. 0 0 0 0 /. 0 0 0 0 /",
		"pzprv3\nsnake\n2\n3\n. 0 2 1 \n3 2 0 4 \n0 8 1 0 ",
		// `qcmp` (`c` in FileIO) is answer data; pzprjs `turnflipGroup` moves it
		// by object id while `adjustExCellTopLeft_2` rewrites only `qnum`, so on
		// boards larger than 1x1 it is not equivariant under the transform group.
		// Keep the property in fixtures on 1x1, where every reversal is identity.
		"pzprv3\nsnake\n1\n1\n. c1 \n2 0 "
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 1, 2]);
			const x = r.rnd();
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.35) {
				cell.qsub = 1;
			}
		});
		p.board.excell.each((excell) => {
			// `ExCell.maxnum`: the top row (by = -1) allows rows, the left
			// column allows cols.
			const max = excell.by === -1 ? p.board.rows : p.board.cols;
			excell.qnum = r.pick([-1, -1, -1, 0, 1, Math.floor(max / 2), max]);
		});
	}
};

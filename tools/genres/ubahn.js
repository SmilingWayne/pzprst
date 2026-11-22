// Fixture inputs for pzprjs pid "ubahn". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "ubahn",
	dataset: null,
	urls: [
		// Full-size boards built with pzprjs from random clues and shading.
		"ubahn/6/6/g3k61g20j4g650l242g6g3g2h5g40i3500040120g",
		"ubahn/5/5/g2g3g0g1h31k23g0154052h3l121100200",
		"ubahn/3/3/7g41g652i36789345g1i10",
		"https://puzz.link/p?ubahn/5/5/g2g3g0g1h31k23g0154052h3l121100200",
		// No pflags exist; an unknown flag is ignored like pzprjs.
		"ubahn/x/2/2/n",
		// Shading layer (`g` is cell 0), `?` (`.`), hex escape and its maximum.
		"ubahn/2/2/vg",
		"ubahn/2/2/.",
		"ubahn/2/2/-ff",
		// Truncated body, trailing junk, junk that is not base 32.
		"ubahn/2/2/1234",
		"ubahn/2/2/1234zz",
		"ubahn/2/2/vz",
		// Empty bodies, including the 20-empty-excell run flush.
		"ubahn/2/2/",
		"ubahn/6/6/",
		// 1x1, 1xN and Nx1 boards.
		"ubahn/1/1/",
		"ubahn/1/1/0",
		"ubahn/1/4/",
		"ubahn/4/1/"
	],
	fileios: [
		// 3x3: excell clues on both bands, `?` (-2), a shaded cell and border lines.
		"pzprv3/ubahn/3/3/. . . . -2 2 3 /. . . . 4 5 . /. . . . . 6 . /. . . . 7 . . /9 8 7 6 . . . /. 5 4 3 . # . /. . . 1 . . . /1 0 /0 1 /0 0 /0 0 1 /1 0 0 /",
		// 1x4: a full 4-row top band, a full 4-column left band, shading and lines.
		"pzprv3\nubahn\n4\n1\n. . . . 4 \n. . . . 3 \n. . . . 2 \n. . . . 1 \n16 15 14 13 . \n12 11 10 9 # \n8 7 6 5 . \n4 3 2 1 . \n\n\n\n\n0 \n0 \n0 ",
		// 6x6: the full puzzle with answer shading, a cross mark (-1) and lines.
		"pzprv3\nubahn\n6\n6\n. . . . . 6 0 . 5 . \n. . . . . . 2 . 6 . \n. . . . 3 . . . . . \n. . . . . . 1 . 4 0 \n2 . . . . . . . . . \n6 . 2 4 . . . . . . \n2 . 3 . # . . . . . \n. 5 . . . . . . . . \n. . 0 4 # . . . # . \n0 5 3 . . . . . . # \n1 0 0 0 0 \n0 0 0 0 0 \n0 1 0 0 0 \n0 0 0 -1 0 \n0 0 0 0 0 \n0 0 0 0 1 \n1 0 0 0 0 0 \n0 0 1 0 0 0 \n0 0 0 0 0 1 \n0 0 0 0 0 0 \n0 0 0 0 0 0 ",
		// 1x1 boards: the `c` (`qcmp`) prefix is only transform-equivariant here,
		// where every band reversal is the identity.
		"pzprv3\nubahn\n1\n1\n. . . . . \n. . . . . \n. . . . . \n. . . . c0 \n. . . . . \n",
		"pzprv3\nubahn\n1\n1\n. . . . . \n. . . . . \n. . . . . \n. . . . 2 \n. . . . # \n"
	],
	randomize(p, r) {
		p.board.excell.each((excell) => {
			// `ExCell.maxnum`: the top band allows rows, the left band cols.
			const max = excell.by < 0 ? p.board.rows : p.board.cols;
			excell.qnum = r.pick([-1, -1, -1, -1, -2, 0, 1, Math.floor(max / 2), max]);
			// `qcmp` is answer data and pzprjs moves it by object id while
			// `adjustExCellTopLeft_2` rewrites only `qnum`, so it is not
			// equivariant under composed transforms; keep it off random boards.
			excell.qcmp = 0;
		});
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 0, 7]);
		});
		p.board.border.each((border) => {
			const line = r.pick([0, 0, 0, 0, 1]);
			border.line = line;
			// `encodeBorderLine` writes the line and only falls back to the
			// cross mark when the line is empty.
			border.qsub = line === 0 && r.rnd() < 0.2 ? 2 : 0;
		});
	}
};

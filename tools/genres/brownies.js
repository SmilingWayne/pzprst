// Fixture inputs for pzprjs pid "brownies". See tools/gen_fixtures.js for the format.
// "yosenabe" and "yajisoko" share pzprjs's variety/yosenabe.js but override
// Encode/FileIO, so they are separate pids and are not covered here.
"use strict";

module.exports = {
	pid: "brownies",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"brownies/9/9/3h939h2h9i9i93i09g9g9g9i92h929h29i9g9g9g91i29i9i9h2h939h3",
		"brownies/8/8/.9g19h.0g9g19h91h90g9h1i9h9i2h9g19h29h91g9g2.h90g9./",
		"brownies/10/10/0.h9i9g9h139g149g9h9h9i229g31g9l9g9g22g9g9l9g22g943i9h9h9g914g931h9g9i9h.1",
		"https://puzz.link/p?brownies/9/9/3h939h2h9i9i93i09g9g9g9i92h929h29i9g9g9g91i29i9i9h2h939h3",
		"http://pzv.jp/p.html?type=player&brownies/8/8/.9g19h.0g9g19h91h90g9h1i9h9i2h9g19h29h91g9g2.h90g9./",
		"https://puzz.link/p?brownies_edit/8/8/.9g19h.0g9g19h91h90g9h1i9h9i2h9g19h29h91g9g2.h90g9./",
		// One cell of every kind: dark cell 9, number 0, largest number 8, ? and
		// empty.
		"brownies/1/1/9",
		"brownies/1/1/0",
		"brownies/1/1/8",
		"brownies/1/1/.",
		"brownies/1/1/--1",
		// Run lengths, truncated bodies, trailing junk, 1/N and N/1 boards.
		"brownies/1/8/z1",
		"brownies/8/1/z1",
		"brownies/4/4/z",
		"brownies/3/3/9",
		"brownies/3/3/123",
		"brownies/3/3/12",
		"brownies/3/3/123zz",
		"brownies/3/3",
		"brownies/3/3/",
		"brownies/4/4/A"
	],
	fileios: [
		// 3x3 dark cell with answer lines and packed qsub/qcmp marks.
		"pzprv3/brownies/3/3/# 8 . /. -2 0 /. . 3 /1 0 /0 -1 /1 0 /0 1 0 /0 0 0 /17 0 16 /0 2 0 /0 0 0 /",
		// 2x2 with every kind of cell token and no marks.
		"pzprv3/brownies/2/2/1 2 /3 4 /0 /0 /0 0 /0 0 /0 0 /",
		// 3x2 board: dark cells in the first row, no answer data.
		"pzprv3/brownies/2/3/# # # /. . . /0 0 /0 0 /0 0 0 /0 0 0 /0 0 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qnum = -1;
			cell.qnum2 = -1;
			if (x < 0.55) {
				cell.qnum = -1;
			} else if (x < 0.68) {
				cell.qnum = -2;
			} else if (x < 0.78) {
				cell.qnum2 = -2;
			} else {
				cell.qnum2 = Math.floor(r.rnd() * 9);
			}
			cell.qsub = Math.floor(r.rnd() * 16);
			cell.qcmp = r.rnd() < 0.3 ? 1 : 0;
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
	}
};

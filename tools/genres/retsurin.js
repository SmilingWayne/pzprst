// Fixture inputs for pzprjs pid "retsurin". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "retsurin",
	dataset: null,
	urls: [
		// Real puzzle from pzprjs test/script/retsurin.js.
		"retsurin/5/5/2i1n1i.m",
		"https://puzz.link/p?retsurin/5/5/2i1n1i.m",
		"https://puzz.link/p?retsurin_edit/5/5/2i1n1i.m",
		"http://pzv.jp/p.html?type=player&retsurin/5/5/2i1n1i.m",
		"retsurin/o/5/5/2i1n1i.m",
		"retsurin/b/5/5/2i1n1i.m",
		"retsurin/ob/5/5/2i1n1i.m",
		"retsurin/5/5/2i1n1i.m/",
		"retsurin/5/5/2i1n1i.mzz",
		"retsurin/x/5/5/2i1n1i.m",
		"retsurin/5/5/2i",
		"retsurin/5/5/",
		"retsurin/5/5",
		"retsurin/1/1/.",
		"retsurin/1/1/0",
		"retsurin/10/1/5",
		"retsurin/40/1/-14",
		"retsurin/1/40/-14",
		"retsurin/2/2/....",
		"retsurin/3/3/012345678",
		"retsurin/30/1/zzk"
	],
	fileios: [
		// Real solved 5x5 board from pzprjs test/script/retsurin.js, with answer data.
		"pzprv3/retsurin/5/5/2 . . . 1 /. . . . . /. . . 1 . /. . - . . /. . . . . /# . . . . /. . . . . /. . . # . /. . . . . /. # . . . /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /",
		// All three config flag lines, question mark, shading, background and a cross.
		"pzprv3/retsurin/2/2/o/m/b/2 . /. - /# . /+ . /0 /1 /0 1 /",
		// 1x1: no inner borders, so the border layer is one empty line.
		"pzprv3/retsurin/1/1/. //"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows) >> 1;
		const clues = [-1, -1, -1, -1, -2];
		for (let n = 0; n <= Math.min(maxnum, 6); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			const x = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.3) {
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
		p.setConfig("koburin_minesweeper", r.rnd() < 0.3);
		p.setConfig("disptype_yajilin", r.rnd() < 0.3 ? 2 : 1);
	}
};

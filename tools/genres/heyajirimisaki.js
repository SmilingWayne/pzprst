// Fixture inputs for pzprjs pid "heyajirimisaki". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "heyajirimisaki",
	dataset: null,
	urls: [
		// Canonical form, empty bodies and a body that ends inside the border layer.
		"heyajirimisaki/4/4",
		"heyajirimisaki/4/4/",
		"heyajirimisaki/4/4/000000",
		// Full URLs (player and editor) and a pzv.jp player URL.
		"https://puzz.link/p?heyajirimisaki/4/4/h20300g0h1q23e42g03a",
		"https://puzz.link/p?heyajirimisaki_edit/4/4/h20300g0h1q23e42g03a",
		"http://pzv.jp/p.html?heyajirimisaki/3/3/g0400m1a23a02e",
		// Unknown pflags are ignored (heyajirimisaki defines no rule flag).
		"heyajirimisaki/f/4/4/h20300g0h1q23e42g03a",
		"heyajirimisaki/xf/4/4/h20300g0h1q23e42g03a",
		// Trailing junk and a truncated arrow layer.
		"heyajirimisaki/3/3/g0400m1a23a02ezz",
		"heyajirimisaki/3/3/0000o5a0abcd9",
		// 1x1, N/1 and 1/N boards.
		"heyajirimisaki/1/1/",
		"heyajirimisaki/1/1/0",
		"heyajirimisaki/1/5/0000",
		"heyajirimisaki/5/1/0000",
		"heyajirimisaki/25/1/0000",
		"heyajirimisaki/1/25/0000",
		// `+` (an arrow without number), `.` (a `?`) and two-digit numbers.
		"heyajirimisaki/3/3/0000o+h",
		"heyajirimisaki/3/3/0000o2.0f",
		"heyajirimisaki/3/3/0000o23"
	],
	fileios: [
		// 3x3: walls, corner numbers, arrows, shading / marks and border lines.
		"pzprv3/heyajirimisaki/3/3/1 0 /0 0 /0 0 /0 0 1 /0 0 0 /5,0 2,3 . /0,2 . . /. . 5,1 /# . + /. . . /. . . /1 0 /0 -1 /0 0 /0 1 0 /0 0 0 /",
		// 4x4: many corner numbers and arrows, two shaded cells.
		"pzprv3/heyajirimisaki/4/4/1 0 0 /0 1 0 /0 0 1 /0 0 0 /0 0 0 1 /1 0 0 0 /0 0 0 0 /2,3 5,0 . . /5,1 . 4,2 . /. . . . /. . 0,3 . /# . . . /. + . . /. . . . /. . . # /1 0 0 /0 -1 0 /0 0 1 /0 0 0 /0 0 0 1 /1 0 0 0 /0 1 0 0 /",
		// 2x2: a wall, `5,0` corner number and the answer layers.
		"pzprv3/heyajirimisaki/2/2/1 /0 /0 1 /2,3 5,0 /. . /# . /. + /0 /-1 /1 0 /",
		// 1x1: every border range is empty.
		"pzprv3/heyajirimisaki/1/1//0,2 /. //"
	],
	randomize(p, r) {
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.25 ? 1 : 0;
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.line = 1;
			} else if (x < 0.3) {
				border.qsub = 2;
			}
		});
		const maxnum = Math.max(p.board.cols, p.board.rows);
		p.board.cell.each((cell) => {
			cell.qdir = 0;
			cell.qnum = -1;
			cell.qnum2 = -1;
			cell.qans = 0;
			cell.qsub = 0;
			const x = r.rnd();
			if (x < 0.25) {
				// Arrow clue; a corner number and an arrow clue are exclusive
				// in FileIO, so a cell never carries both.
				cell.qdir = Math.floor(r.rnd() * 5);
				cell.qnum = Math.floor(r.rnd() * (maxnum + 1));
			} else if (x < 0.35) {
				cell.qdir = Math.floor(r.rnd() * 5);
				cell.qnum = -2;
			} else if (x < 0.45) {
				cell.qnum = -3;
			} else if (x < 0.75) {
				cell.qnum2 = Math.floor(r.rnd() * (maxnum + 1));
			}
			if (r.rnd() < 0.2) {
				cell.qans = 1;
			}
			if (r.rnd() < 0.2) {
				cell.qsub = 1;
			}
		});
	}
};

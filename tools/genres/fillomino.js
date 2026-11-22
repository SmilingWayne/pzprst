// Fixture inputs for pzprjs pid "fillomino". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "fillomino",
	dataset: "Fillomino",
	urls: [
		// real 10x10 puzzles (puzz.link, re-canonicalised unchanged by pzprjs)
		"https://puzz.link/p?fillomino/10/10/9j86i2g2h87k6l6g5g2g2g8g55k87g57g64k57g4g4g8g8g6l8k62h5g5i34j6",
		"https://puzz.link/p?fillomino/10/10/h666g75h7m55g4g3o42g2g3h7i26g4h7g68i3h2g2g55o2g5g72m3h77g483h",
		// canonical board without any wall (encodeBorderIfPresent omits the layer)
		"fillomino/4/4/1u",
		// canonical board with a wall: the border layer follows the number layer
		"fillomino/2/2/1ig0",
		// the "t" (fillomino_tri) flag, alone and with a wall layer
		"fillomino/t/4/4/1u",
		"fillomino/t/2/2/1ig0",
		// full puzz.link and pzv.jp player URLs
		"https://puzz.link/p?fillomino/4/4/1ig0",
		"http://pzv.jp/p.html?type=player&fillomino/4/4/1u",
		// largest clue a 4x4 board allows (maxnum = 16, written as the "-" escape)
		"fillomino/4/4/-10",
		// 1/1, 1/N and N/1 boards
		"fillomino/1/1/1",
		"fillomino/1/5/12345",
		"fillomino/5/1/12345",
		// question mark
		"fillomino/4/4/.",
		// empty body
		"fillomino/4/4/",
		// truncated body
		"fillomino/3/3/12",
		// trailing junk
		"fillomino/4/4/1zz",
		// unknown pflag, ignored by pzprjs
		"fillomino/x/4/4/1",
		// run-length fill up to the last cell
		"fillomino/10/10/0000"
	],
	fileios: [
		// 2 columns x 3 rows; question clues, answer numbers / sub marks / "done"
		// marks and one border of every answer token: "-X", "X", "2", "1", "-1", "0"
		"pzprv3/fillomino/3/2/1 . /. 2 /. - /. + /. c /. . /-X /X /2 /1 -1 /0 0 /",
		// 3x3 with the config flag line; every clue value the triangular rule allows
		// (maxnum 3) and "?"; border walls and answer marks
		"pzprv3/fillomino/3/3/t/1 2 3 /3 . - /. . . /. 4 . /+ . . /c . . /X 0 /0 -1 /0 0 /0 0 1 /0 0 0 /",
		// 1x1 board: the border layer is a single empty line
		"pzprv3/fillomino/1/1/. /. /"
	],
	randomize(p, r) {
		const tri = r.rnd() < 0.3;
		p.setConfig("fillomino_tri", tri);
		// Cell.maxnum: 3 under the triangular rule, otherwise cols * rows.
		const maxnum = tri ? 3 : p.board.cols * p.board.rows;
		const values = [-1, -1, -1, -1, -1, -2];
		for (let v = 1; v <= maxnum; v++) {
			values.push(v);
		}
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(values);
			const x = r.rnd();
			if (x < 0.2) {
				cell.anum = r.pick([0, 1, 2, 3]);
			} else if (x < 0.3) {
				cell.qcmp = 1;
			} else if (x < 0.4) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.ques = 1;
				if (r.rnd() < 0.3) {
					border.qsub = 1;
				}
			} else if (x < 0.4) {
				border.qans = 1;
				if (r.rnd() < 0.4) {
					border.qsub = 1;
				}
			} else if (x < 0.55) {
				border.qsub = 1;
			}
		});
	}
};

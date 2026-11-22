// Fixture inputs for pzprjs pid "armyants". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "armyants",
	dataset: null,
	urls: [
		// Canonical boards and the URL host forms.
		"armyants/4/4/0000001234r",
		"armyants/4/4/0000001234",
		"https://puzz.link/p?armyants/4/4/0000001234",
		"http://pzv.jp/p.html?type=player&armyants/4/4/0000001234",
		"https://puzz.link/p?armyants_edit/4/4/0000001234",
		// Border layer only, no body at all and a truncated border body.
		"armyants/4/4/000000",
		"armyants/4/4",
		"armyants/4/4/",
		"armyants/4/4/00",
		// Every border wall set, then numbers.
		"armyants/4/4/ffffff1234",
		"armyants/4/4/vvvvvv1234",
		// Digits outside base 16 are skipped one cell at a time.
		"armyants/4/4/000000g5",
		"armyants/4/4/000000zz",
		// Trailing junk after the last cell.
		"armyants/4/4/0000001234zz",
		// An unknown number, then hex escapes for 16 and 31.
		"armyants/4/4/000000.",
		"armyants/4/4/000000.5",
		"armyants/4/4/000000-10",
		"armyants/4/4/000000-1f",
		"armyants/4/4/000000$7",
		// One-cell and small boards.
		"armyants/1/1/0",
		"armyants/2/2/00",
		"armyants/2/2/0012",
		"armyants/3/2/000",
		"armyants/2/3/00",
		// One-row and one-column boards.
		"armyants/25/1/00000",
		"armyants/1/25/00000",
		// A 10x10 board with numbers above 15.
		"armyants/10/10/0000000000000000000000000000000000000-10-11-12-13-14-15-16-17-18-19-1a-1b"
	],
	fileios: [
		// Numbers, unknown numbers, marks, border walls and border lines.
		"pzprv3/armyants/4/4/1 . 2 . /. 3 . . /. . - 5 /. . . 6 /1 . + . /. 1 . . /. . = . /. . . % /1 0 1 /0 1 0 /1 0 0 /0 0 1 /1 0 1 0 /0 1 0 1 /0 0 1 0 /1 0 0 /0 1 0 /0 0 1 /1 0 0 /0 1 0 1 /1 0 1 0 /0 1 1 0 /",
		// Numbers 16 and 30 on a small board, with answer marks.
		"pzprv3/armyants/2/2/16 . /30 2 /1 . /. + /1 /0 /1 1 /1 /0 /0 1 /",
		// 1x1 board: the two empty border layers.
		"pzprv3/armyants/1/1/1 /. ///",
		// A wide board with numbers 1, 26 and 36.
		"pzprv3/armyants/2/3/1 . - /. 26 36 /1 . + /. . = /1 0 /0 1 /1 0 1 /1 0 /0 1 /1 0 0 /",
		// 30 empty cells: the encoder must use the 20-cell run marker.
		"pzprv3/armyants/5/6/. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /"
	],
	randomize(p, r) {
		const maxnum = Math.min(p.board.cols * p.board.rows, 999);
		const nums = [-1, -1, -1, -1, -1, 1, 2, -2];
		if (maxnum >= 7) {
			nums.push(7);
		}
		if (maxnum >= 16) {
			nums.push(16);
		}
		nums.push(maxnum);
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(nums);
			cell.anum = -1;
			cell.qans = r.rnd() < 0.2 ? 1 : 0;
			cell.qsub = 0;
			cell.qcmp = 0;
			if (cell.qans === 0 && r.rnd() < 0.2) {
				cell.qsub = 1 + Math.floor(r.rnd() * 4);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.ques = x < 0.3 ? 1 : 0;
			border.line = x > 0.6 ? 1 : 0;
		});
	}
};

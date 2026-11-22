// Fixture inputs for pzprjs pid "turnaround". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "turnaround",
	dataset: null,
	urls: [
		"turnaround/4/4/1234",
		"https://puzz.link/p?turnaround/7/7/1g2g2g1m2g1g2g2m3g1g2g2m2g2g1g1",
		"https://puzz.link/p?turnaround/10/10/j3h2j3h1j1h1j1h0j3h2j3h1j2h0j2h1j2h1j2h1j3h2j3h2j",
		"https://puzz.link/p?turnaround/f/6/6/g3n3i3w3i",
		"turnaround/f/4/4/1234",
		"turnaround/f/2/2/12",
		"http://pzv.jp/p.html?type=player&turnaround/2/2/12",
		"turnaround_edit/3/3/123",
		"turnaround/4/4/1",
		"turnaround/4/4/1234zzzzzz",
		"turnaround/4/4",
		"turnaround/4/4/",
		"turnaround/4/4//",
		"turnaround/1/1/0",
		"turnaround/1/1/3",
		"turnaround/5/5/1.2.3.",
		"turnaround/2/2/--1",
		"turnaround/2/1/0g",
		"turnaround/1/2/g",
		"turnaround/30/1/z1",
		"turnaround/1/30/zp",
		"turnaround/x/4/4/12"
	],
	fileios: [
		"pzprv3/turnaround/2/2/f/1 2 /. . /1 /-1 /1 0 /",
		"pzprv3/turnaround/3/3/0 1 2 /3 . . /. . . /0 1 /-1 0 /0 1 /1 0 0 /0 1 0 /",
		"pzprv3/turnaround/1/1/0 //"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -2, 0, 1, 2, 3]);
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.3) {
				border.line = 1;
			} else if (x < 0.45) {
				border.qsub = 2;
			}
		});
		p.setConfig("loop_full", r.rnd() < 0.3);
	}
};

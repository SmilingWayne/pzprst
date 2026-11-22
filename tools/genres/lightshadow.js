// Fixture inputs for pzprjs pid "lightshadow". See tools/gen_fixtures.js for the format.
// "nibunnogo" shares pzprjs's variety/lightshadow.js but overrides Encode/FileIO, so it
// is a separate pid and is not covered here.
"use strict";

module.exports = {
	pid: "lightshadow",
	dataset: null,
	urls: [
		"lightshadow/4/4/2103r",
		"https://puzz.link/p?lightshadow/4/4/2103r",
		"https://puzz.link/p?lightshadow_edit/4/4/2103r",
		"http://pzv.jp/p.html?type=player&lightshadow/4/4/2103r",
		"lightshadow/f/4/4/2103r",
		"lightshadow/xf/4/4/2103r",
		"lightshadow/4/4/2103r/",
		"lightshadow/4/4/2103rzz",
		"lightshadow/6/6/0001",
		"lightshadow/4/4",
		"lightshadow/4/4/",
		"lightshadow/1/1/.",
		"lightshadow/1/1/3",
		"lightshadow/4/4/-20",
		"lightshadow/4/4/-21",
		"lightshadow/30/1/z2",
		"lightshadow/1/30/zp",
		"lightshadow/2/2/--1",
		"lightshadow/4/4/%001",
		"lightshadow/4/4/A",
		"lightshadow/4/4/z",
		"lightshadow/16/8/+100+101-c8zzzzzzk",
		"lightshadow/1/1/=000",
		"lightshadow/1/1/@001",
		"lightshadow/1/1/*0000",
		"lightshadow/1/1/$00000"
	],
	fileios: [
		"pzprv3/lightshadow/2/2/0,1 1,-2 /. . /1 0 /0 2 /",
		"pzprv3/lightshadow/2/1/0,-2 /1,-2 /0 /2 /",
		"pzprv3/lightshadow/1/1/. /0 /"
	],
	randomize(p, r) {
		const maxnum = p.board.cols * p.board.rows;
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.55) {
				cell.qnum = -1;
			} else if (x < 0.72) {
				cell.qnum = -2;
				cell.ques = r.rnd() < 0.5 ? 1 : 0;
			} else {
				cell.qnum = 1 + Math.floor(r.rnd() * maxnum);
				cell.ques = r.rnd() < 0.4 ? 1 : 0;
			}
			cell.qans = r.pick([0, 0, 0, 1, 2]);
		});
	}
};

// Fixture inputs for pzprjs pid "ruleofthree". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "ruleofthree",
	dataset: null,
	urls: [
		"https://puzz.link/p?ruleofthree/3/3/hh",
		"ruleofthree/3/3/hh",
		"ruleofthree/3/3/",
		"ruleofthree/2/2/h",
		"ruleofthree/2/2/g",
		"ruleofthree/3/3/z",
		"ruleofthree/3/3/zzzz",
		"ruleofthree/3/3/0",
		"ruleofthree/3/3/000",
		"ruleofthree/1/1/",
		"ruleofthree/1/4/",
		"ruleofthree/4/1/",
		"ruleofthree/2/3/h",
		"ruleofthree/3/2/0",
		"ruleofthree/2/2/hzz",
		"ruleofthree/2/2/h/",
		"ruleofthree/4/3/0000g",
		"ruleofthree/3/4/g0"
	],
	fileios: [
		"pzprv3/ruleofthree/2/2/l 2 /+ # /1 0 0 /0 0 0 /0 1 /-1 0 /0 0 /",
		"pzprv3/ruleofthree/2/3/o3 - # /2 . o+ /0 0 0 0 /1 0 0 0 /0 1 0 /-1 0 0 /0 0 2 /",
		"pzprv3/ruleofthree/1/1/# /0 0 /1 /-1 /"
	],
	randomize(p, r) {
		const bd = p.board;
		bd.cell.each((cell) => {
			const bx = cell.bx;
			const by = cell.by;
			const col = (bx < bd.maxbx >> 1 ? bd.maxbx - bx : bx) >> 1;
			const row = (by < bd.maxby >> 1 ? bd.maxby - by : by) >> 1;
			const maxnum = Math.max(col, row);
			const kind = r.rnd();
			if (kind < 0.15) {
				cell.ques = 7;
				cell.qans = 0;
				cell.qnum = -1;
			} else if (kind < 0.45) {
				cell.ques = 0;
				cell.qans = r.pick([11, 12, 13]);
				cell.qnum = -1;
			} else {
				cell.ques = 0;
				cell.qans = 0;
				cell.qnum =
					maxnum >= 2 && r.rnd() < 0.5
						? 2 + Math.floor(r.rnd() * (maxnum - 1))
						: r.pick([-1, -1, -1, -2]);
			}
			cell.qsub = r.pick([0, 0, 0, 0, 1]);
		});
		bd.border.each((border) => {
			if (r.rnd() < 0.2) {
				border.line = 1;
			} else if (r.rnd() < 0.15) {
				border.qsub = 2;
			}
		});
	}
};

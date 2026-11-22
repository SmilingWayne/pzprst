// Fixture inputs for pzprjs pid "mukkonn". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "mukkonn",
	dataset: null,
	urls: [
		// Real puzz.link boards.
		"https://puzz.link/p?mukkonn/8/8/.1..i.11.h.2..zi..1.1..2zi1...h2..1i2.1./",
		"https://puzz.link/p?mukkonn/10/10/j..13u.4.1g35..j..53m..32m2...j..4.h1..2j52..m4...m..35j53..g3...u..22j",
		"https://puzz.link/p?mukkonn/8/8/i2222t1111q3333h2222q1111t3333i",
		"mukkonn_edit/8/8/i2222t1111q3333h2222q1111t3333i",
		"http://pzv.jp/p.html?type=player&mukkonn/8/8/i2222t1111q3333h2222q1111t3333i",
		// Unknown flag and trailing junk.
		"mukkonn/x/8/8/i2222t1111q3333h2222q1111t3333i",
		"mukkonn/8/8/i2222t1111q3333h2222q1111t3333i/zz",
		// 1/1: empty, a compass with four `?` slots and a black cell.
		"mukkonn/1/1",
		"mukkonn/1/1/",
		"mukkonn/1/1/....",
		"mukkonn/1/1/_",
		// Every clue slot filled on a 2x2 board.
		"mukkonn/2/2/0123456701234567",
		// Hex escapes in every slot of a 2x2 board.
		"mukkonn/2/2/-0f-1f+0a0+7ff",
		// Base-36 run lengths, 1/N and N/1 boards.
		"mukkonn/4/4/z",
		"mukkonn/4/4/zn",
		"mukkonn/1/4/z",
		"mukkonn/4/1/z",
		// Truncated body and empty body.
		"mukkonn/4/4/0123",
		"mukkonn/4/4",
		"mukkonn/4/4/"
	],
	fileios: [
		// 2x2: two compass cells (one with `?` slots), a black cell and border lines.
		"pzprv3/mukkonn/2/2/0,1,-1,2 # /-1,-1,-1,-1 . /0 /0 /1 -1 /",
		// 3x2: numbers above maxnum, `?` slots, a black cell and a plain empty cell.
		"pzprv3/mukkonn/2/3/0,1,2,3 # 4,5,6,7 /. . . /0 0 /-1 0 /-1 1 0 /",
		// 1x1 with a `?` slot.
		"pzprv3/mukkonn/1/1/2,3,-1,-2 /"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows) - 1;
		p.board.cell.each((cell) => {
			cell.qnum = -1;
			cell.qnum2 = -1;
			cell.qnum3 = -1;
			cell.qnum4 = -1;
			const x = r.rnd();
			if (x < 0.12) {
				cell.ques = 7;
			} else if (x < 0.7) {
				cell.ques = 51;
				for (const prop of ["qnum", "qnum2", "qnum3", "qnum4"]) {
					cell[prop] = r.pick([-1, -1, -1, -2, 0, 1, 2, 3, maxnum]);
				}
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			border.line = 0;
			border.qsub = 0;
			if (x < 0.2) {
				border.line = r.pick([1, 2]);
			} else if (x < 0.3) {
				border.qsub = 2;
			}
		});
	}
};

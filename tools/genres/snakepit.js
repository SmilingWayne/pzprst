// Fixture inputs for pzprjs pid "snakepit". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "snakepit",
	// No "Snake Pit" folder in puzzlekit-dataset; the three real puzzle URLs
	// below come from .pzprst-samples.js and are canonical already.
	dataset: null,
	urls: [
		"snakepit/3/3/02m000",
		"snakepit/2/2/2i00g0",
		"snakepit/3/3/02m000g000",
		"https://puzz.link/p?snakepit/9/9/4g5j4q2g3h3q8g4j6q4g4h5q4g6j7gg0n0005bf000g0n0005bf000g0n",
		"snakepit/10/10/q56j54h43j63zzh36j64h45j53q6200060i60000000000000002200i00060",
		"https://puzz.link/p?snakepit/6/6/s2l3l4l5g009in37k0f0l",
		"http://pzv.jp/p.html?type=player&snakepit/6/6/s2l3l4l5g009in37k0f0l",
		"snakepit_edit/3/3/02m000",
		"snakepit/t/3/3/02m000",
		"snakepit/6/6/0001",
		"snakepit/6/6/00016f769000zz",
		"snakepit/3/3/.",
		"snakepit/4/4/-10u",
		"snakepit/1/1/",
		"snakepit/1/5/2222200",
		"snakepit/5/1/2222200",
		"snakepit/2/2/2i00g"
	],
	fileios: [
		"pzprv3/snakepit/3/3/2 . 3/. 4 ./- . 5/0 1 6/7 0 0/0 0 6/. 5 ./c . +/. - ./X 0/0 -1/-X 1/0 2 0/1 0 0",
		"pzprv3/snakepit/2/2/t/2 ./. 3/0 6/1 0/3 ./c ./X/-1/0 -X",
		"pzprv3/snakepit/1/1/-/6/."
	],
	randomize(p, r) {
		const tri = r.rnd() < 0.3;
		p.setConfig("fillomino_tri", tri);
		const maxnum = tri ? 3 : p.board.cols * p.board.rows;
		p.board.cell.each((cell) => {
			cell.qnum = -1;
			cell.anum = -1;
			cell.qsub = 0;
			cell.qcmp = 0;
			const x = r.rnd();
			if (x < 0.08) {
				cell.ques = 7;
			} else if (x < 0.2) {
				cell.ques = r.pick([1, 6]);
			} else {
				cell.ques = 0;
			}
			if (cell.ques === 0) {
				const y = r.rnd();
				if (maxnum >= 2 && y < 0.45) {
					cell.qnum = 2 + Math.floor(r.rnd() * (maxnum - 1));
				} else if (y < 0.55) {
					cell.qnum = -2;
				}
			}
			const z = r.rnd();
			if (maxnum >= 2 && z < 0.2) {
				cell.anum = 2 + Math.floor(r.rnd() * (maxnum - 1));
			} else if (z < 0.3) {
				cell.qcmp = 1;
			} else if (z < 0.45) {
				cell.qsub = 1 + Math.floor(r.rnd() * 4);
			}
		});
		p.board.border.each((border) => {
			border.ques = r.pick([0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
			const x = r.rnd();
			if (x < 0.15) {
				border.qans = 1;
			} else if (x < 0.3) {
				border.qsub = 1;
			} else if (x < 0.45) {
				border.qans = 1;
				border.qsub = 1;
			}
		});
	}
};

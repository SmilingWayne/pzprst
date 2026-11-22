// Fixture inputs for pzprjs pid "kakuro". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kakuro",
	dataset: "Kakuro",
	urls: [
		"kakuro/1/1/12",
		"kakuro/1/1/00",
		"kakuro/1/1/ZZ",
		"kakuro/1/1/.",
		"kakuro/1/1/12zz",
		"kakuro/2/2/21l6524",
		"kakuro/2/2/21l65",
		"kakuro/3/3/",
		"kakuro/3/3/k",
		"kakuro/3/3/z",
		"kakuro/3/3/.",
		"kakuro/1/4/0000",
		"kakuro/4/1/0000",
		"kakuro/3/2/43o6--",
		"kakuro/4/4/",
		"https://puzz.link/p?kakuro/11/11/nS00044l7000oEan3033l0hmAcoW00hlHbqDOoT0000JncKng0f0Dgo0Eqg4l00DhogEm3gl400Kn0go000hl00000hnaBd83WaP46JD",
		"https://puzz.link/p?kakuro/11/11/00JhlK00DmK000EKnhKng0mPEmPEoAOoDgq0Kn0000KglO000KhlI0E0gJnhPqEOoBhohEmfEm0Kn0Jn00000Em000dl0000EgOgEEcOhC",
		"https://puzz.link/p?kakuro/11/11/mjDm07tc6r4An70.VdlSCpdfldbnS4q4Oqjanfhl.pDhlA0D09bnhOr0Vt0Dm0AmEhBEgWbO5bVPIgB6D",
		"http://pzv.jp/p.html?type=player&kakuro/1/1/12"
	],
	fileios: [
		"pzprv3/kakuro/1/1/0 1 /-1 45,2 /./",
		"pzprv3/kakuro/2/2/0 1 2 /3 1,2 . /4 . 5,6 /. . /3 . /",
		"pzprv3\nkakuro\n2\n3\n0 5 6 -1 \n2 3,4 . . \n-1 . . . \n. c + \n. . - "
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			if (r.rnd() < 0.45) {
				cell.ques = 51;
				const pick = () => r.pick([-1, -1, 0, 1, 2, 5, 10, 19, 20, 44, 45]);
				cell.qnum = pick();
				cell.qnum2 = pick();
			} else {
				cell.ques = 0;
				cell.qnum = -1;
				cell.qnum2 = -1;
			}
			// `decodeCellAnumsub` stores one of these per cell; the encoder
			// writes the first non-empty one, so only one may be set.
			const ans = r.rnd();
			if (ans < 0.5) {
				cell.anum = r.pick([-1, -1, -1, 1, 2, 3, 9]);
				cell.qcmp = 0;
				cell.qsub = 0;
			} else if (ans < 0.75) {
				cell.anum = -1;
				cell.qcmp = 1;
				cell.qsub = 0;
			} else if (ans < 0.9) {
				cell.anum = -1;
				cell.qcmp = 0;
				cell.qsub = r.pick([1, 2, 3, 4]);
			} else {
				cell.anum = -1;
				cell.qcmp = 0;
				cell.qsub = 0;
			}
		});
		p.board.excell.each((excell) => {
			// The top-left corner always encodes as "0"; only the top row's
			// `qnum2` and the left column's `qnum` are stored.
			if (excell.bx === -1 && excell.by === -1) {
				return;
			}
			const pick = () => r.pick([-1, -1, 0, 1, 2, 5, 10, 19, 20, 44, 45]);
			if (excell.by === -1) {
				excell.qnum2 = pick();
			} else if (excell.bx === -1) {
				excell.qnum = pick();
			}
		});
	}
};

"use strict";
module.exports = {
	pid: "sudoku",
	dataset: "Sudoku",
	square: true,
	urls: [
		"https://puzz.link/p?sudoku/9/9/g23h6g7g1h45i8n15l4h7h6h9h4l32n3i87h6g6g1h95g",
		"https://puzz.link/p?sudoku/9/9/k13m27h476h39h825zg841h29h538h18m96k",
		"https://puzz.link/p?sudoku/9/9/p87i65g1h6g7h42h5g8h3g34i12p3m2g7h5h3i24g95h",
		"sudoku/9/9/1g2g3g4g5g6g7g8g9",
		"sudoku/6/6/1j2j3j4j5j6",
		"sudoku/4/4/1g2g3g4",
		"sudoku/1/1/.",
		"sudoku/4/4/",
		"sudoku/4/4/-10g+100"
	],
	fileios: [
		"pzprv3/sudoku/2/1 . /. 2 /.[1,,3,] 2 /+ - /",
		"pzprv3/sudoku/1/- /.[,,,4] /"
	],
	randomize(p, r) {
		const max = Math.max(p.board.cols, p.board.rows);
		p.board.cell.each((c) => {
			c.qnum = r.rnd() < 0.35 ? 1 + Math.floor(r.rnd() * max) : -1;
			if (r.rnd() < 0.2) c.anum = 1 + Math.floor(r.rnd() * max);
			else if (r.rnd() < 0.3) c.snum = [1, -1, 2, -1];
		});
	}
};

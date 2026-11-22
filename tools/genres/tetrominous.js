// Fixture inputs for pzprjs pid "tetrominous". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tetrominous",
	dataset: null,
	urls: [
		"tetrominous/4/4/0123.456789abcde",
		"https://puzz.link/p?tetrominous/4/4/0123.456789abcde",
		"http://pzv.jp/p.html?type=player&tetrominous/4/4/0123.456789abcde",
		"tetrominous/4/4/0123.456789abcde/",
		"tetrominous/4/4/0123.456789abcde//",
		"tetrominous/4/4/0123.456789abcdezzz",
		"tetrominous/t/4/4/0123.456789abcde",
		"tetrominous/x/4/4/0123.456789abcde",
		"tetrominous/4/4/4444444444444444",
		"tetrominous/4/4/................",
		"tetrominous/4/4/",
		"tetrominous/4/4/01",
		"tetrominous/4/4/0123.456789abcdevvvvvv",
		"tetrominous/4/4/0123.456789abcdeAAAAAA",
		"tetrominous/1/1/0",
		"tetrominous/1/1/c",
		"tetrominous/1/4/",
		"tetrominous/1/4/0c0c",
		"tetrominous/4/1/0c0c",
		"tetrominous/10/10/1023401234012340123401234012340123401234012340123401234012340123401234012340123401234012340123",
		"tetrominous/6/6/0.1234.3210.4.4.01234543210.1.2.3.4",
		"tetrominous/5/4/012.34.012344.210.0."
	],
	fileios: [
		"pzprv3/tetrominous/2/2/t/1 0 /- * /c 3 /+ . /-X /X /1 2 /",
		"pzprv3/tetrominous/3/2/0 1 /2 3 /- * /. - /% . /. + /1 /-1 /2 /0 1 /-X 1 /",
		"pzprv3/tetrominous/2/2/. . /. . /. . /. . /0 /0 /0 0 /"
	],
	randomize(p, r) {
		// pzprjs throws while decoding *any* 2x1 tetrominous URL (its
		// AreaNumBlockGraph calls getnodeobjs on a node that does not exist),
		// and the generator re-reads every random board as a URL, so replace
		// that one shape before filling the board.
		if (p.board.cols === 2 && p.board.rows === 1) {
			p.open("tetrominous/2/2");
		}
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.ques = 0;
			cell.qnum = -1;
			if (x < 0.15) {
				cell.ques = 7;
			} else if (x < 0.25) {
				cell.qnum = -2;
			} else if (x < 0.6) {
				cell.qnum = Math.floor(r.rnd() * 5);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.ques = 1;
			} else if (x < 0.35) {
				border.qans = 1;
			} else if (x < 0.45) {
				border.qsub = 1;
			}
		});
		p.setConfig("fillomino_tri", r.rnd() < 0.3);
	}
};

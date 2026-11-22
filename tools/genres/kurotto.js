// Fixture inputs for pzprjs pid "kurotto". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "kurotto",
	dataset: "Kurotto",
	urls: [
		// Real puzzles, all verified as already-canonical with tools/pzprjs.js.
		"kurotto/10/10/7g3h.x1h3h3n4j4h5r3g8i2h2h3j2g4s9h1i",
		"kurotto/10/10/k2v6l3gah.2j.l65r.g2i1g1ak6k4h5j2n7",
		"kurotto/10/10/h6g5h8g.g2j4i0g2z1m4k0g4g2j3i2l1t5.h3h6",
		"https://puzz.link/p?kurotto/10/10/7g3h.x1h3h3n4j4h5r3g8i2h2h3j2g4s9h1i",
		"http://pzv.jp/p.html?type=player&kurotto/10/10/h6g5h8g.g2j4i0g2z1m4k0g4g2j3i2l1t5.h3h6",
		"kurotto/10/10/7g3h.x1h3h3n4j4h5r3g8i2h2h3j2g4s9h1i/",
		"kurotto/10/10/7g3h.x1h3h3n4j4h5r3g8i2h2h3j2g4s9h1i//",
		"kurotto/10/10/7g3h.x1h3h3n4j4h5r3g8i2h2h3j2g4s9h1izz",
		"kurotto/10/10/7g3h.x1h3h3n4j4h5r3g8i2h2h3j2g4s9h1",
		// The largest clue is cell count - 1; 15 and 35 are hex "f" and "-23".
		"kurotto/4/4/f",
		"kurotto/6/6/-23",
		"kurotto/2/2/-3",
		// `minnum` is 0, so zero clues are legal ("0" in hex is the digit 0).
		"kurotto/3/3/000000000",
		"kurotto/3/3/023456789",
		// `?` is a dot.
		"kurotto/3/3/.",
		"kurotto/3/3/.........",
		// Skipped cells (g-z), short bodies and unknown characters.
		"kurotto/5/5/g1h2i3",
		"kurotto/5/5/j",
		"kurotto/5/5/zzzzz",
		"kurotto/4/4/A",
		"kurotto/6/6/2",
		// 1/1, 1/N and N/1 boards (1x1 has maxnum 0).
		"kurotto/1/1/.",
		"kurotto/1/1/0",
		"kurotto/1/6/0",
		"kurotto/6/1/0",
		"kurotto/1/20/v",
		"kurotto/20/1/v",
		// Empty bodies still canonicalise with a run-length marker.
		"kurotto/4/4",
		"kurotto/4/4/",
		"kurotto/x/4/4/0"
	],
	fileios: [
		// Question and answer layers, including the legal zero clue.
		"pzprv3/kurotto/3/3/0 3 . /. 0 . /. . 8 /. # . /# . # /. . .",
		// Question marks, two-digit clues and every answer token.
		"pzprv3/kurotto/2/3/- 0 5 /. 3 . /+ # . /. . # /+ . .",
		// Single-cell boards.
		"pzprv3/kurotto/1/1/0 /#",
		"pzprv3/kurotto/1/2/. 1 /. ."
	],
	randomize(p, r) {
		const maxnum = Math.min(p.board.cols * p.board.rows - 1, 999);
		const clues = [-1, -1, -1, -1, -2, 0, 0];
		for (let n = 0; n <= Math.min(maxnum, 12); n++) {
			clues.push(n);
		}
		clues.push(maxnum);
		p.board.cell.each((cell) => {
			cell.qnum = r.pick(clues);
			const x = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (x < 0.2) {
				cell.qans = 1;
			} else if (x < 0.3) {
				cell.qsub = 1;
			}
		});
	}
};

// Fixture inputs for pzprjs pid "yosenabe". See tools/gen_fixtures.js for the format.
// There is no Yosenabe folder under ../puzzlekit-dataset/assets/data, so the
// three real puzzles below come from the pzprst sample list (pzprjs-canonical).
"use strict";

module.exports = {
	pid: "yosenabe",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"yosenabe/10/10/5v4gdc5f1k5guk6m14vkmi6o4o4j4li8q6j5o7ri6m3o6j7u2j5ki8l1t",
		"yosenabe/10/10/s8belfmhius0mklkmm1nl4m4mi4n4i81iaw5t3zi4j1j5o4s33j2p",
		"yosenabe/10/10/e28ve0i8stn7290ev28e2ifoi4n7t1li9j6libt9k8wi55n6t5v1",
		"https://puzz.link/p?yosenabe/10/10/5v4gdc5f1k5guk6m14vkmi6o4o4j4li8q6j5o7ri6m3o6j7u2j5ki8l1t",
		"http://pzv.jp/p.html?type=player&yosenabe/3/3/005",
		// Ice layer ('0'/'g' for the 3x3 and 10x10 boards) plus clue layer.
		"yosenabe/3/3/005",
		"yosenabe/3/3/g05",
		"yosenabe/3/3/g0",
		"yosenabe/3/3/00.",
		// The largest URL clue: '-' escapes 16-255; 'i'/'g' carry a second number.
		"yosenabe/3/3/00-ff",
		"yosenabe/3/3/00i5",
		"yosenabe/3/3/00g1f",
		// Unexpected character, trailing junk, truncated clue layer.
		"yosenabe/3/3/00A5",
		"yosenabe/3/3/005zz",
		"yosenabe/4/4/0",
		// Smallest and lopsided boards, empty bodies.
		"yosenabe/1/1/05",
		"yosenabe/1/4/01234",
		"yosenabe/4/1/01234",
		"yosenabe/4/4",
		// An unknown pflag is dropped, and 'x' falls back to the default size.
		"yosenabe/f/3/3/005",
		"yosenabe/x/3/3/005"
	],
	fileios: [
		// The real 10x10 puzzle with answer border lines and the qsub/qcmp layer.
		"pzprv3.1/yosenabe/10/10/. . i . i6 i i i i i /. o4 i . . i . . o4 . /o4 i i . i8 . i i . . /. . i o6 i o5 i i i i /. . o7 . i i . i . . /. . i6 . i i . o3 . . /i i i i o6 i o7 i . . /. . i i . i . i i o2 /. o5 . . i8 . . i o1 . /i i i i i i . i . . /1 0 0 0 0 0 0 1 0 /0 0 0 0 0 0 1 0 0 /0 0 0 0 0 1 0 0 0 /0 0 0 0 1 0 0 0 -1 /0 0 0 1 0 0 0 -1 0 /0 0 1 0 0 0 -1 0 0 /0 1 0 0 0 -1 0 0 1 /1 0 0 0 -1 0 0 1 0 /0 0 0 -1 0 0 1 0 0 /0 0 -1 0 0 1 0 0 0 /1 0 0 0 0 1 0 0 0 0 /0 0 0 0 1 0 0 0 0 1 /0 0 0 1 0 0 0 -1 1 0 /0 0 1 0 -1 0 0 1 0 0 /0 1 0 0 0 0 1 0 0 0 /1 0 0 0 0 1 0 0 0 0 /0 0 0 0 1 0 0 0 -1 1 /0 0 0 1 0 -1 0 0 1 0 /0 0 1 0 0 0 0 1 0 0 /3 0 0 0 0 0 0 0 16 3 /0 0 0 0 0 0 16 0 3 0 /0 0 0 0 16 0 0 3 0 0 /0 0 16 0 0 0 3 0 0 0 /16 0 0 0 0 3 0 0 16 0 /0 0 0 0 3 0 16 0 0 0 /0 0 0 3 16 0 0 0 0 0 /0 0 3 0 0 0 0 0 0 0 /16 3 0 0 0 0 0 0 16 255 /3 0 0 0 0 0 16 0 255 3 /",
		// Ice, a question mark, a second number and every answer token (filever 1).
		"pzprv3.1/yosenabe/3/3/io5 . o /. i3 . /2 . i /1 0 /0 -1 /0 0 /0 1 0 /1 0 0 /1 17 0 /32 0 3 /0 255 0 /",
		// filever 0 has no qsub/qcmp layer; the encoder writes pzprv3.1 regardless.
		"pzprv3/yosenabe/2/2/io1 2 / o . /1 /-1 /0 1 /",
		// 1x1 has no inner borders, so only the cell layer and the qsub/qcmp layer.
		"pzprv3.1/yosenabe/1/1/i /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = -1;
			cell.qnum2 = -1;
			cell.ques = 0;
			if (r.rnd() < 0.1) {
				cell.ques = 6;
			}
			const x = r.rnd();
			if (x < 0.4) {
				cell.qnum = r.pick([-2, 1, 2, 3, 9, 15, 16, 255]);
			} else if (x < 0.55) {
				cell.qnum2 = r.pick([1, 2, 15, 16, 255]);
			}
			const y = r.rnd();
			cell.qsub = y < 0.08 ? r.pick([1, 3, 15]) : 0;
			cell.qcmp = y > 0.94 ? r.pick([1, 2, 15]) : 0;
		});
		p.board.border.each((border) => {
			border.line = 0;
			border.qsub = 0;
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

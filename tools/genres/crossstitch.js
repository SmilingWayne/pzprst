// Fixture inputs for pzprjs pid "crossstitch". See tools/gen_fixtures.js for the format.
// The URL is `decodeArrowNumber16`; FileIO is `decodeCellDirecQnum`, a layer with
// `o`-`u` for `qsub` 1-7 / a digit for `qans`-30 / `c` for `qcmp`, and the cross
// dot layer. No dataset folder exists for this pid.
"use strict";

module.exports = {
	pid: "crossstitch",
	dataset: null,
	urls: [
		"crossstitch/4/4",
		"crossstitch/1/1",
		"crossstitch/1/4",
		"crossstitch/4/1",
		// Every cell kind: clue with direction, `?`, and empty runs.
		"crossstitch/2/2/0.1.2.3.",
		"crossstitch/2/2/00",
		// `+` is a cell with an arrow but no number.
		"crossstitch/2/2/+",
		"crossstitch/2/2/+0.",
		"crossstitch/1/1/+",
		"crossstitch/3/3/+4.1.",
		// Truncated bodies and unexpected characters. A `dir` character with no
		// hex digit after it stores NaN in pzprjs, so truncation is only tested
		// between cells.
		"crossstitch/3/3/0.",
		"crossstitch/3/3/0.!1.",
		// Base-36 blank runs, including the 26-cell flush.
		"crossstitch/6/6/z",
		"crossstitch/6/6/zz",
		"crossstitch/3/3/0.zz",
		// The clue number escape forms: `-` (256-4095) and `5`-`9` (16-255).
		"crossstitch/2/2/-1abc",
		"crossstitch/2/2/5ff",
		// Unknown flags are ignored, as in pzprjs; a flag without a body too.
		"crossstitch/f/2/2/0.0.0.0.",
		"crossstitch/f/2/2",
		"crossstitch_edit/3/3/0.1.",
		// Real puzz.link puzzles.
		"https://puzz.link/p?crossstitch/8/8/40f00b31b03e+h+f31h+e03b01b00f30",
		"https://puzz.link/p?crossstitch/9/9/j03e41n12e02a11++g+n31e31j",
		"https://puzz.link/p?crossstitch/10/10/22h01b+01b4.i+d+2.e+f01h11f+e++d3.i00b+4.b+h+/"
	],
	fileios: [
		// 2x2: clue with direction, `?`, `+`, empty, qsub bits, qans 31/32 and qcmp.
		"pzprv3/crossstitch/2/2/0,1 3,- /# . /o1 u2 /c . /0 1 2 /1 1 2 /2 2 0 /",
		// 3x3: every direction, `?`/`+`/clue, qsub 1-7, qans 31-33, dots 0-2.
		"pzprv3/crossstitch/3/3/0,0 1,1 2,- /3,2 4,3 # /. 0,- 1,4 /o1 u2 c /. o3 1 /u1 . 2 /0 1 2 0 /1 2 0 1 /2 0 1 2 /0 1 2 0 /",
		// One-row board (4x1): cross grid 5x2.
		"pzprv3/crossstitch/1/4/3,1 # 2,- 4,0 /o1 . u2 c /0 1 2 0 1 /2 0 1 2 0 /",
		// 1x1 with a `?` clue and dots on the cross grid.
		"pzprv3/crossstitch/1/1/2,- /o3 /1 2 /2 0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, -2, -3, 0, 1, 2, 3, 4]);
			// `#` (`qnum` -3) and an empty cell can only keep a direction of 0:
			// the FileIO writer stores `#` / `.` and drops anything else.
			cell.qdir = cell.qnum === -1 || cell.qnum === -3 ? 0 : r.pick([0, 0, 0, 1, 2, 3, 4]);
			cell.qans = r.pick([0, 0, 0, 31, 31, 32, 32, 33]);
			// `qsub` is the `o`-`u` prefix of the answer token; a clue is needed
			// for pzprjs's `prehook` to accept it.
			cell.qsub = cell.qnum === -1 ? 0 : r.pick([0, 0, 0, 1, 2, 3, 4, 5, 6, 7]);
			// `qcmp` is only written when there is no slash answer.
			cell.qcmp = cell.qans === 0 ? r.pick([0, 0, 0, 1]) : 0;
		});
		p.board.cross.each((cross) => {
			cross.qsub = r.pick([0, 0, 0, 1, 1, 2]);
		});
	}
};

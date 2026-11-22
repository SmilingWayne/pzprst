// Fixture inputs for pzprjs pid "interbd". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "interbd",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"interbd/10/10/-14lav-1eiag-1en-3ck-32v-28r-14g-28t-14k-3cg",
		"interbd/9/9/-28i1i-34h-2bi-14i-2ak-14p1i-16i1p-14kfi-14i-22h-3di3i-1e",
		"interbd/10/10/h2h2q-1fi1m2i-14l2i0p0i2lai2m1i1q2h2h",
		"https://puzz.link/p?interbd/10/10/h2h2q-1fi1m2i-14l2i0p0i2lai2m1i1q2h2h",
		"http://pzv.jp/p.html?type=player&interbd/10/10/h2h2q-1fi1m2i-14l2i0p0i2lai2m1i1q2h2h",
		"https://puzz.link/p?interbd_edit/10/10/h2h2q-1fi1m2i-14l2i0p0i2lai2m1i1q2h2h",
		// Packed single values: 0 white `?`, 10 colour 1 `?`, 65 colour 6 clue 4.
		"interbd/1/1/0",
		"interbd/1/1/10",
		"interbd/1/1/65",
		// Large escapes and non-canonical spellings of the same values.
		"interbd/1/1/+100",
		"interbd/2/2/65",
		"interbd/1/1/70",
		// Empty and cleared cells, run lengths, truncated and trailing data.
		"interbd/1/1/.",
		"interbd/1/1/--1",
		"interbd/2/2/-zz",
		"interbd/1/8/z1",
		"interbd/8/1/z1",
		"interbd/3/3",
		"interbd/3/3/",
		"interbd/3/3/-14",
		"interbd/3/3/1zz",
		"interbd/4/4/z",
		"interbd/1/1/A"
	],
	fileios: [
		// Real 3x3 question data with answer shading and a dot.
		"pzprv3/interbd/3/3/20 # . /. + . /. 10 65 /",
		// One token of every packed kind on a 2x2 board.
		"pzprv3/interbd/2/2/0 1 /11 22 /",
		// 4x4 with colours 0-6, numbers 0-4, answer marks and an empty cell.
		"pzprv3/interbd/4/4/0 1 2 3 /10 11 12 13 /20 21 22 23 /64 65 # + /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.45) {
				cell.qnum = -1;
				cell.ques = 0;
			} else {
				cell.ques = Math.floor(r.rnd() * 7);
				cell.qnum = r.rnd() < 0.3 ? -2 : Math.floor(r.rnd() * 5);
				if (cell.qnum === -1 && cell.ques !== 0) {
					cell.qnum = -2;
				}
			}
			const z = r.rnd();
			cell.qans = z < 0.2 ? 1 : 0;
			cell.qsub = z >= 0.2 && z < 0.3 ? 1 : 0;
		});
	}
};

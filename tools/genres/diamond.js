// Fixture inputs for pzprjs pid "diamond". See tools/gen_fixtures.js for the format.
// There is no Diamond folder under ../puzzlekit-dataset/assets/data, so the last
// three real URLs come from ../.pzprst-samples.js.
"use strict";

module.exports = {
	pid: "diamond",
	dataset: null,
	urls: [
		"diamond/6/6/gcg.sdrb",
		"https://puzz.link/p?diamond/6/6/gcg.sdrb",
		"http://pzv.jp/p.html?type=player&diamond/6/6/gcg.sdrb",
		"diamond_edit/6/6/gcg.sdrb",
		"diamond/3/3/1234",
		"diamond/3/3/....",
		"diamond/4/4/0123",
		"diamond/2/2/44",
		"diamond/2/2/..",
		"diamond/1/1/0",
		"diamond/1/4/0",
		"diamond/4/1/0",
		"diamond/4/4/0",
		"diamond/4/4/",
		"diamond/4/4",
		"diamond/4/4/01zz",
		"diamond/4/4/A",
		"diamond/6/6/gcg.sdrbzz",
		"https://puzz.link/p?diamond/9/9/.g.r.i.o.x.o.g.i",
		"https://puzz.link/p?diamond/10/10/obzgbdzjeczhbic"
	],
	fileios: [
		// 3x3 with a clue, `?`, shade, the plus mark, three diamonds and a dot
		// grid holding an X, a `?` dot and several marks.
		"pzprv3/diamond/3/3/1 2 3 /4 . . /. # + /1 2 /0 1 /...4./.X..-/...../32.../67.../",
		// 2x2 with a clue, a border mark and a dot grid with a dot, a `?` dot
		// and an X on the cross.
		"pzprv3/diamond/2/2/1 2 /. . /0 /.3./4X./.../",
		// 2x2 without a dot grid: the plus mark is stored in the cell layer.
		"pzprv3/diamond/2/2/1 - /. + /1 /"
	],
	randomize(p, r) {
		const bd = p.board;
		bd.cell.each((cell) => {
			cell.qnum = -1;
			cell.qans = 0;
			cell.qsub = 0;
			cell.ques = 0;
			const x = r.rnd();
			if (x < 0.12) {
				cell.qnum = -2;
			} else if (x < 0.45) {
				cell.qnum = r.pick([0, 1, 2, 3, 4]);
			} else if (x < 0.55) {
				cell.qans = 1;
			} else if (x < 0.65) {
				cell.qsub = 1;
			}
		});
		// pzprjs `Cross.setDiamond`: 1 is a diamond (`qans`), 2 an auxiliary
		// mark (`qsub`); `force` skips the overlap check exactly like the FileIO
		// decoder does.
		bd.cross.each((cross) => {
			const val = r.pick([0, 0, 0, 0, 1, 1, 2]);
			cross.setDiamond(val, true);
			cross.ques = 0;
		});
		// The dot grid is the pieces' `qsub`; `Dot.setDot` refuses hidden
		// pieces (a number, a shade, or a diamond nearby), so only the encoder's
		// view is legal. An X only survives when the piece is empty and some dot
		// in the same layer holds a value of 2 or more, because pzprjs writes
		// the dot layer only then.
		const dots = [];
		for (let s = 0; s < bd.dotsmax; s++) {
			const dot = bd.dots[s];
			dots.push(dot);
			const x = r.rnd();
			if (x < 0.08 && dot.piece.qsub === 0) {
				dot.piece.ques = 7;
			} else if (x < 0.3) {
				dot.piece.ques = 0;
				dot.setDot(r.pick([-2, 1, 2, 3, 4, 5, 9]));
			} else {
				dot.piece.ques = 0;
				dot.setDot(0);
			}
		}
		if (!dots.some((dot) => dot.getDot() >= 2)) {
			dots.forEach((dot) => {
				dot.piece.ques = 0;
			});
		}
	}
};

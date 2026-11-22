// Fixture inputs for pzprjs pid "parquet". See tools/gen_fixtures.js for the format.
// There is no Parquet folder under ../puzzlekit-dataset/assets/data, so the real
// puzzle below comes from pzprjs test/state/native_codec_fixtures.js.
"use strict";

module.exports = {
	pid: "parquet",
	dataset: null,
	urls: [
		// Real 6x6 puzzle; canonicalises to itself.
		"parquet/6/6/836360147471658380141411832520030360",
		"https://puzz.link/p?parquet/6/6/836360147471658380141411832520030360",
		"http://pzv.jp/p.html?type=player&parquet/6/6/836360147471658380141411832520030360",
		"parquet_edit/6/6/836360147471658380141411832520030360",
		// Real puzz.link puzzles (native-derived, re-canonicalise unchanged).
		"https://puzz.link/p?parquet/12/12/802223252362171174236520602863652362771174236520626223652362171174147411056323080280636363717111636563222620652363117171656365602280036303660000",
		"https://puzz.link/p?parquet/12/10/225625283222114114114111630230502360837746377462140837408311237146371460656838368380141141411411625623083280003063603000",
		// 1x1, 1xN and Nx1 boards.
		"parquet/1/1/0",
		"parquet/1/3/0",
		"parquet/3/1/0",
		// Truncated body, trailing junk, empty body.
		"parquet/2/2/8",
		"parquet/6/6/836360147471658380141411832520030360zz",
		"parquet/3/3/",
		"parquet/6/6",
		// Largest single-digit value: right 2, bottom 2.
		"parquet/3/3/888888888",
		// A `.` in the number layer stores -2 in the bottom border (0 * 3 + -2).
		"parquet/2/2/.",
		"parquet/2/2/..",
		// Out-of-domain border values still round-trip through the number layer.
		"parquet/4/4/0123456789abcdef",
		"parquet/2/2/+1f-1f"
	],
	fileios: [
		// The real 6x6 puzzle, answered: shaded cells and two background marks.
		"pzprv3/parquet/6/6/2 1 2 1 2 /0 1 2 1 2 /2 1 2 1 2 /0 1 0 1 0 /2 1 0 1 0 /0 1 0 1 2 /2 0 0 0 0 0 /1 1 1 1 1 1 /0 2 2 0 2 0 /1 1 1 1 1 1 /2 0 2 2 2 0 /# # . . . . /# # . . . . /. . # . . . /. . # # . . /. . . . + . /. . . . . + /",
		// 2x2: the -2 quirk on a bottom border, plus every answer mark.
		"pzprv3/parquet/2/2/0 /0 /-2 0 /# . /. + /",
		// 1x1 has no inner borders; only the cell answer layer.
		"pzprv3/parquet/1/1//. /"
	],
	randomize(p, r) {
		p.board.border.each((border) => {
			border.ques = r.pick([0, 0, 0, 1, 1, 2]);
		});
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.qans = 0;
			cell.qsub = 0;
			if (x < 0.25) {
				cell.qans = 1;
			} else if (x < 0.4) {
				cell.qsub = 1;
			}
		});
	}
};

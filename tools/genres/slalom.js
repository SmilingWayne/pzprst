// Fixture inputs for pzprjs pid "slalom". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "slalom",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical, `d` flag).
		"slalom/d/10/10/42c261331513142a241413315162a1415313142c1515141729151425h2715h17p/4",
		"slalom/d/10/10/424241821242411341114242a1415133315292526131133133a2533314142141925h25k154a3ai2cj/38",
		"slalom/d/10/10/728331414131816161c28162131526282433182433333151382738g4232n44/87",
		"https://puzz.link/p?slalom/d/10/10/42c261331513142a241413315162a1415313142c1515141729151425h2715h17p/4",
		"suraromu/d/3/3/21a01/2",
		"http://pzv.jp/p.html?type=player&slalom/d/10/10/728331414131816161c28162131526282433182433333151382738g4232n44/87",
		// The `p` flag (URL version 1, wall clue numbers without a direction) and
		// its combination with `d` (version 2 wins).
		"slalom/p/3/3/21a1/2",
		"slalom/dp/3/3/21a01/2",
		"slalom/pd/3/3/21a01/2",
		// No flag: URL version 0, one number per gate component that is
		// distributed to the adjacent wall cells.
		"slalom/3/3/25181/1",
		"slalom/4/4/22511d35/2",
		"slalom/4/4/22511d-c81/2",
		// Every clue size: one hex digit, 5+hex, and the `-dir` three-digit form.
		"slalom/d/3/3/1b0f/1",
		"slalom/d/3/3/1b510/1",
		"slalom/d/3/3/1b5ff/1",
		"slalom/d/3/3/1b-03e7/1",
		// Gates, walls without clues and a start on an empty cell.
		"slalom/d/3/3/21a01/2",
		"slalom/d/2/2/1234g/3",
		// 1x1, 1xN and Nx1 boards.
		"slalom/d/1/1/4/0",
		"slalom/d/1/3/141h/1",
		"slalom/d/3/1/141h/1",
		// Truncated body and trailing junk after the start id.
		"slalom/d/3/3/4",
		"slalom/d/3/3/c/0zz"
	],
	fileios: [
		// Start, both gates, walls with and without clues, empty runs, and every
		// border-arrow token (`-1`, `-n`, `u`/`d`/`r` marks).
		"pzprv3.3/slalom/3/3/o . . /0,3 i . /. # 0,200 /-1 2u -2 /0 3d 0 /4r 0 11 /0 1 0 /0 0 0 /",
		// filever 1: `decodeBoard_pzpr1` and the older `decodeBorderLine` layer.
		"pzprv3.1/slalom/3/3/o . . /5 . . /- . # /0 0 /1 0 /0 -1 /0 1 0 /0 0 0 /",
		// filever 0: `decodeBoard_old` (`i3`/`w7` gate tokens) and line borders.
		"pzprv3/slalom/3/3/i3 o . /# . . /. # . /-1 0 /0 1 /0 0 /0 0 0 /0 0 0 /"
	],
	randomize(p, r) {
		const bd = p.board;
		const cells = [];
		bd.cell.each((cell) => {
			cell.ques = 0;
			cell.qnum = -1;
			cell.qdir = 0;
			cells.push(cell);
		});
		cells.forEach((cell) => {
			const x = r.rnd();
			if (x < 0.3) {
				cell.ques = 1;
				cell.qdir = r.pick([0, 0, 1, 2, 3, 4]);
				cell.qnum = r.pick([-1, -1, -1, -1, -1, 1, 2, 3, 15, 16, 255, 256, 999]);
			} else if (x < 0.45) {
				cell.ques = r.pick([21, 22]);
			}
		});
		// The FileIO start token hides everything else on its cell, so keep the
		// start on an empty cell.
		let start = cells.find((cell) => cell.ques === 0 && cell.qnum === -1);
		if (!start) {
			start = cells[Math.floor(r.rnd() * cells.length)];
			start.ques = 0;
			start.qnum = -1;
			start.qdir = 0;
		}
		bd.startpos.set(start);
		bd.border.each((border) => {
			border.line = r.pick([0, 0, 0, 0, 0, 1, 2, 3]);
			border.qsub = r.pick([0, 0, 0, 0, 0, 0, 2, 11, 12, 13, 14]);
		});
	}
};

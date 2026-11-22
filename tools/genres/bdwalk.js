// Fixture inputs for pzprjs pid "bdwalk". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "bdwalk",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"bdwalk/6/6/1256g1g2g02g0g3h3i40g4g0h1g5g0j6g",
		"bdwalk/9/9/1991p0j2o0l0m6m3l4o0j5p",
		"bdwalk/7/7/11772h0j5h0g7g0h0i0h0s3h4l",
		"https://puzz.link/p?bdwalk/6/6/1256g1g2g02g0g3h3i40g4g0h1g5g0j6g",
		// The `m` flag / bdwalk_height config, on and off.
		"bdwalk/m/6/6/1256g1g2g02g0g3h3i40g4g0h1g5g0j6g",
		"bdwalk/m/1/1/1111g",
		// Question mark, largest clue, blank runs, junk after the last cell.
		"bdwalk/6/6/1256.g1",
		"bdwalk/6/6/1256-64g1",
		"bdwalk/6/6/1256zz",
		"bdwalk/6/6/1351zv",
		"bdwalk/6/6/1256g1g2g02g0g3h3i40g4g0h1g5g0j6gzz",
		// Truncated cell layer and an unknown character pzprjs skips.
		"bdwalk/6/6/1256g1",
		"bdwalk/6/6/1256g!g1",
		// 1x1, 1xN and Nx1 boards.
		"bdwalk/1/1/1111g",
		"bdwalk/1/4/1114j",
		"bdwalk/4/1/1141j"
	],
	fileios: [
		// `m` config line, question markers, all clue kinds and answers.
		"pzprv3/bdwalk/6/6/m/1 3 9 11/- . D . . . /. . U 2 . . /. . . . 3 . /U . 99 . U . /. D . 4 . - /1 . . . 5 . /0 1 0 0 -1 /0 0 0 2 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 1 0 0 0 0 /0 0 2 0 0 0 /0 0 0 3 0 0 /",
		// No config line, 3x4 board with direction marks on borders.
		"pzprv3/bdwalk/4/3/1 1 5 7/- 1 2 /U . D /. . 3 /- . U /0 0 /-1 1u /0 0 /0 2d /0 0 0 /1l 0 0 /0 0 0 /",
		// 1x1 board: start and goal are the same cell.
		"pzprv3/bdwalk/1/1/1 1 1 1/- /"
	],
	randomize(p, r) {
		const cols = p.board.cols,
			rows = p.board.rows;
		const pos = () => [2 * (1 + Math.floor(r.rnd() * cols)) - 1, 2 * (1 + Math.floor(r.rnd() * rows)) - 1];
		let start = pos(),
			goal = pos();
		if (cols * rows === 1) {
			start = [1, 1];
			goal = [1, 1];
		} else {
			// pzprjs snaps coinciding positions to the board corners after every
			// board operation, which is not invertible; keep them apart.
			while (start[0] === goal[0] && start[1] === goal[1]) {
				goal = pos();
			}
		}
		p.board.startpos.bx = start[0];
		p.board.startpos.by = start[1];
		p.board.goalpos.bx = goal[0];
		p.board.goalpos.by = goal[1];
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, -1, -1, -1, -2, -3, -4, 1, 2, 3, 99]);
		});
		p.board.border.each((border) => {
			border.line = r.pick([0, 0, 0, 0, 0, 1, 2]);
			border.qsub = r.pick([0, 0, 0, 0, 0, 0, 2, 11, 12, 13, 14]);
		});
		p.setConfig("bdwalk_height", r.rnd() < 0.3);
	}
};

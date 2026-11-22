// Fixture inputs for pzprjs pid "tateyoko". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "tateyoko",
	dataset: null,
	urls: [
		// Real puzzles from the pzprst sample list (pzprjs-canonical).
		"tateyoko/10/10/n27i2on12i2qi312nxi91211n43r2i3xi2xi23i311i4qi2xi4p22n13i32i63pn1n11nxn4i3qi222n",
		"tateyoko/10/10/1i2qi23i41i22ri2p12npi4qi3p2i2xn1i31nqn1i22n2i5q2n3i22np1i2rn1nq2i3rn2nxi2qn1i32i5pn1n",
		"tateyoko/10/10/n2npi23i33q2i2qi2qi21n1i22i22q2npn1p1nxi32i3pn2n1pi54npi2qi2ri2pi32i32n1npi2pi3xi23i32p2n1",
		"https://puzz.link/p?tateyoko/10/10/n27i2on12i2qi312nxi91211n43r2i3xi2xi23i311i4qi2xi4p22n13i32i63pn1n11nxn4i3qi222n",
		"http://pzv.jp/p.html?type=player&tateyoko/10/10/1i2qi23i41i22ri2p12npi4qi3p2i2xn1i31nqn1i22n2i5q2n3i22np1i2rn1nq2i3rn2nxi2qn1i32i5pn1n",
		// Every token class: blocks `o`–`s`, numbers, `?`, the `-` escape, `x`,
		// `n` and `i` runs, and the unrecognized-character fall-through.
		"tateyoko/3/3/ops",
		"tateyoko/3/3/qs",
		"tateyoko/3/3/xyz",
		"tateyoko/3/3/.",
		"tateyoko/3/3/1",
		"tateyoko/3/3/i3",
		"tateyoko/3/3/n",
		"tateyoko/3/3/-10",
		"tateyoko/3/3/-ff",
		"tateyoko/1/1/1",
		"tateyoko/1/1/x",
		"tateyoko/1/1/.",
		"tateyoko/1/5/nn",
		"tateyoko/5/1/i5",
		// Truncated body and trailing junk.
		"tateyoko/3/3/s",
		"tateyoko/3/3/1zz"
	],
	fileios: [
		// Every block letter (`a`–`f`), numbers, `?` and both bar answers.
		"pzprv3/tateyoko/3/3/a b c /? 4 f /e . . /. . . /0 0 . /. 1 2 /",
		// Numbers, a block with length 4 and vertical/horizontal answers.
		"pzprv3/tateyoko/2/2/0 2 /d . /1 2 /. 0 /",
		// 1x1 board: a `?` cell with a vertical-bar answer.
		"pzprv3/tateyoko/1/1/? /1 /"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows);
		p.board.cell.each((cell) => {
			if (r.rnd() < 0.25) {
				cell.ques = 1;
				cell.qnum = r.pick([-1, -1, 0, 1, 2, 3, 4]);
				cell.qans = 0;
			} else {
				cell.ques = 0;
				cell.qnum = r.pick([-1, -1, -1, -2, 0, 1, 2, 3, 4, maxnum]);
				cell.qans = r.pick([0, 0, 0, 0, 12, 13]);
			}
		});
	}
};

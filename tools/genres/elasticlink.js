// Fixture inputs for pzprjs pid "elasticlink". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "elasticlink",
	dataset: null,
	urls: [
		"elasticlink/4/4/0g004k2n100g0",
		"elasticlink/4/4/0040i.r",
		"elasticlink/3/3/10og0",
		"elasticlink/1/1/01",
		"elasticlink/1/5/0h2h",
		"elasticlink/5/1/0i3g",
		"https://puzz.link/p?elasticlink/4/4/0g004k2n100g0",
		"elasticlink/4/4/0g004",
		"elasticlink/4/4/0g004k2n100g0zz",
		"elasticlink/4/4/",
		"elasticlink/1/1/0",
		"elasticlink/1/1/",
		"elasticlink/2/2/01",
		"elasticlink/5/5/zzzzzzzz",
		"http://pzv.jp/p.html?type=player&elasticlink/4/4/0g004k2n100g0"
	],
	fileios: [
		"pzprv3\nelasticlink\n2\n3\n3 O2 # \n- O- O \n1 0 \n-1 0 \n1 0 0 \n",
		"pzprv3\nelasticlink\n2\n2\n1 . \n. # \n0 \n0 \n0 0 \n",
		"pzprv3\nelasticlink\n1\n1\nO- \n\n"
	],
	randomize(p, r) {
		const maxnum = Math.max(p.board.cols, p.board.rows);
		const clues = [-1, -1, -1, -1, -1, -2];
		for (let n = 1; n <= maxnum; n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 0, 6, 7]);
			cell.qnum = r.pick(clues);
		});
		p.board.border.each((border) => {
			border.line = r.rnd() < 0.2 ? 1 : 0;
			border.qsub = r.rnd() < 0.1 ? 2 : 0;
		});
	}
};

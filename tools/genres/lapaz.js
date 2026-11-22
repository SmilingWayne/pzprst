// Fixture inputs for pzprjs pid "lapaz". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "lapaz",
	dataset: null,
	urls: [
		// Real puzzles: three verified with .pzprst-samples.js and one from the
		// pzprjs test data (test/script/lapaz.js).
		"lapaz/8/8/l23n13p32j32p13n10l",
		"lapaz/10/10/g2j2m5j4h2j2h3j3m2j1h2j4m1j3h3j2h2j2m1j3g",
		"lapaz/6/6/200o2p1o001",
		"lapaz/6/6/00j20l1l2l02j02",
		"https://puzz.link/p?lapaz/6/6/00j20l1l2l02j02",
		"http://pzv.jp/p.html?type=player&lapaz/8/8/l23n13p32j32p13n10l",
		"lapaz_edit/8/8/l23n13p32j32p13n10l",
		// The `l` (liar) flag and an unknown flag.
		"lapaz/l/6/6/00j20l1l2l02j02",
		"lapaz/lx/6/6/00j20l1l2l02j02",
		"lapaz/x/6/6/00j20l1l2l02j02",
		// Truncated body, trailing junk and a trailing slash.
		"lapaz/8/8/l23n13p32j",
		"lapaz/8/8/l23n13p32j32p13n10lzz",
		"lapaz/8/8/l23n13p32j32p13n10l/",
		// `?` is a dot; shaded cells come from the binary empty layer.
		"lapaz/5/5/.",
		"lapaz/5/5/g",
		"lapaz/5/5/g1h2i3",
		"lapaz/4/4/A",
		// maxnum is max(cols,rows)/2 (+1 with the liar flag): 3 on 6x6, 4 with `l`.
		"lapaz/6/6/333333333333333333333333333333333333",
		"lapaz/l/6/6/444444444444444444444444444444444444",
		// 1/1, 1/N and N/1 boards.
		"lapaz/1/1/.",
		"lapaz/1/1/0",
		"lapaz/1/5/00000",
		"lapaz/5/1/00000",
		"lapaz/1/20/v",
		"lapaz/20/1/v",
		// Empty bodies still canonicalise with run-length markers.
		"lapaz/4/4",
		"lapaz/4/4/",
		"lapaz/x/4/4/0"
	],
	fileios: [
		// The 6x6 solved pzprjs test state (question, border and cell layers).
		"pzprv3/lapaz/6/6/0 0 . . . . /2 0 . . . . /. . 1 . . . /. . . 2 . . /. . . . 0 2 /. . . . 0 2 /0 1 0 1 0 /1 1 1 0 0 /1 1 1 1 0 /0 0 1 0 0 /0 1 0 1 1 /0 0 1 1 1 /1 1 1 1 1 0 /0 0 0 1 1 0 /1 1 1 1 1 1 /0 1 1 1 1 1 /0 1 1 0 0 0 /+ + + + + + /+ + + + + # /+ + + # + + /# + + + + # /+ + # + + + /# + + + + + /",
		// The same 6x6 puzzle with the `l` config flag line.
		"pzprv3/lapaz/6/6/l/0 0 . . . . /2 0 . . . . /. . 1 . . . /. . . 2 . . /. . . . 0 2 /. . . . 0 2 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /0 0 0 0 0 0 /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /. . . . . . /",
		// Shaded (`#`), question mark (`-`), answer border `2` and answer cells.
		"pzprv3/lapaz/3/3/# - 2 /. 1 . /2 . . /2 1 /0 0 /1 0 /1 0 0 /0 0 0 /# + . /. # . /. . + /",
		// 1x1 has one empty inner-vertical layer and no horizontal row.
		"pzprv3/lapaz/1/1/0 //. /"
	],
	randomize(p, r) {
		const liar = r.rnd() < 0.3;
		const maxnum = (liar ? 1 : 0) + Math.max(p.board.cols, p.board.rows) / 2;
		const clues = [-1, -1, -1, -1, -2];
		for (let n = 0; n <= Math.min(maxnum, 6); n++) {
			clues.push(n);
		}
		p.board.cell.each((cell) => {
			// A `#` token wins over the number, so never set both.
			if (r.rnd() < 0.15) {
				cell.ques = 7;
			} else {
				cell.qnum = r.pick(clues);
			}
			const x = r.rnd();
			if (x < 0.15) {
				cell.qans = 1;
			} else if (x < 0.3) {
				cell.qsub = 1;
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.qans = 1;
			} else if (x < 0.4) {
				border.qsub = 1;
			}
		});
		p.setConfig("lapaz_liar", liar);
	}
};

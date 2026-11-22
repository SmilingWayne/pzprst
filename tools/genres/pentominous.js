// Fixture inputs for pzprjs pid "pentominous". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "pentominous",
	dataset: null,
	urls: [
		"https://puzz.link/p?pentominous/10/10/v2u2n2g22j2i2l22zj2s",
		"pentominous/10/10/v2u2n2g22j2i2l22zj2s",
		"https://puzz.link/p?pentominous/19/19/ccccccckcccccccccccccjchcccccccccccckcbcccccccn4ncjco2gci6ic5pc1zlcccccp0icccccmckccccccckch3gchcccccjcmcgccccckcm8zhcycalcwcccccccg7kccccccccccci9iccccccccccccckccccccc0000000000010802200000110000000040000000000044g10800000g50000000000000000000000002100000000000000003c0r80000002000012h00090001000000000000",
		"https://puzz.link/p?pentominous/10/9/z2911z92z2712t2k",
		"http://pzv.jp/p.html?type=player&pentominous/10/9/z2911z92z2712t2k",
		"pentominous/10/9/z2911z92z2712t2k/",
		"pentominous/10/9/z2911z92z2712t2kzz",
		"pentominous/10/9/z291",
		"pentominous/5/5/",
		"pentominous/1/5/",
		"pentominous/4/4/0123456789ab....",
		"pentominous/4/4/bbbbbbbbbbbbbbbb",
		"pentominous/4/4/........",
		"pentominous/3/3/123456789",
		"pentominous/3/3/A1",
		"pentominous/3/3/q1",
		"pentominous/1/1/0",
		"pentominous/1/1/b",
		"pentominous/1/1/c",
		"pentominous/1/6/012345",
		"pentominous/6/1/012345",
		"pentominous/t/4/4/0123456789ab....",
		"pentominous/x/4/4/0123456789ab....",
		"pentominous/4/4/0123456789ab....vvvvvv"
	],
	fileios: [
		"pzprv3/pentominous/2/2/t/0 10 /11 * /c 3 /+ . /-X /X /1 2 /",
		"pzprv3/pentominous/3/2/0 1 /2 3 /- * /. - /% . /. + /1 /-1 /2 /0 1 /-X 1 /",
		"pzprv3/pentominous/2/2/. . /. . /. . /. . /0 /0 /0 0 /"
	],
	randomize(p, r) {
		// pzprjs throws while decoding *any* 2x1 pentominous URL (its
		// AreaNumBlockGraph calls getnodeobjs on a node that does not exist),
		// and the generator re-reads every random board as a URL, so replace
		// that one shape before filling the board.
		if (p.board.cols === 2 && p.board.rows === 1) {
			p.open("pentominous/2/2");
		}
		p.board.cell.each((cell) => {
			const x = r.rnd();
			cell.ques = 0;
			cell.qnum = -1;
			if (x < 0.12) {
				cell.ques = 7;
			} else if (x < 0.22) {
				cell.qnum = -2;
			} else if (x < 0.6) {
				cell.qnum = Math.floor(r.rnd() * 12);
			}
		});
		p.board.border.each((border) => {
			const x = r.rnd();
			if (x < 0.2) {
				border.ques = 1;
			} else if (x < 0.35) {
				border.qans = 1;
			} else if (x < 0.45) {
				border.qsub = 1;
			}
		});
		p.setConfig("fillomino_tri", r.rnd() < 0.3);
	}
};

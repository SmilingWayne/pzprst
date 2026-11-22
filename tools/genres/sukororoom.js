// Fixture inputs for pzprjs pid "sukororoom". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "sukororoom",
	dataset: null,
	urls: [
		"https://puzz.link/p?sukororoom/6/6/dfrcvrre5vecf2i2e3m",
		"sukororoom/6/6/dfrcvrre5vecf2i2e3m",
		"https://puzz.link/p?sukororoom/10/10/nrnfdbp5timpmpdnns4svecvuufnvsbvtst7g1zzzn",
		"https://puzz.link/p?sukororoom/6/6/etnbmktlnnkvb1r4i1d",
		"http://pzv.jp/p.html?type=player&sukororoom/6/6/etnbmktlnnkvb1r4i1d",
		"sukororoom/6/6/etnbmktlnnkvb1r4i1d/",
		"sukororoom/6/6/etnbmktlnnkvb1r4i1dzz",
		"sukororoom/6/6/etnb",
		"sukororoom/4/4/",
		"sukororoom/4/4/000000",
		"sukororoom/4/4/0000004444",
		"sukororoom/4/4/vvvvvv",
		"sukororoom/4/4/zzzzzz1234",
		"sukororoom/1/1/",
		"sukororoom/1/1/1",
		"sukororoom/1/4/01",
		"sukororoom/4/1/g0123",
		"sukororoom/3/3/v0....",
		"sukororoom/3/3/zzzz1234",
		"sukororoom/8/8/000000/....4444",
		"sukororoom/8/8/000000/"
	],
	fileios: [
		"pzprv3/sukororoom/2/2/1 /0 /0 1 /1 . /. 4 /c . /. + /",
		"pzprv3/sukororoom/3/2/1 /0 /1 /0 1 /1 1 /1 . /- 4 /. 2 /c + /. 3 /- % /",
		"pzprv3/sukororoom/1/1//. /0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.qnum = r.pick([-1, -1, -1, -1, 1, 2, 3, 4, -2]);
			const x = r.rnd();
			if (x < 0.2) {
				cell.anum = Math.floor(r.rnd() * 6);
			} else if (x < 0.35) {
				cell.qcmp = 1;
			} else if (x < 0.5) {
				cell.qsub = r.pick([1, 2, 3, 4]);
			}
		});
		p.board.border.each((border) => {
			if (r.rnd() < 0.25) {
				border.ques = 1;
			}
		});
	}
};

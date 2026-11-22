// Fixture inputs for pzprjs pid "nanameguri". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "nanameguri",
	dataset: "GokigenNaname",
	urls: [
		// Hand-built 2x2 and 3x3: inner walls, both slash directions.
		"nanameguri/2/2/g0h9",
		"nanameguri/3/3/40g0360",
		"https://puzz.link/p?nanameguri/3/3/40g0360",
		"https://puzz.link/p?nanameguri_edit/3/3/40g0360",
		"http://pzv.jp/p.html?type=player&nanameguri/3/3/40g0360",
		// Empty 1x1 and a single slash.
		"nanameguri/1/1/0",
		"nanameguri/1/1/9",
		// Base-27 escape, 1/N and N/1 boards.
		"nanameguri/1/5/000",
		"nanameguri/5/1/000",
		"nanameguri/4/4/000000",
		// Truncated body, trailing junk and empty bodies.
		"nanameguri/3/3/00",
		"nanameguri/3/3/0000000000zz",
		"nanameguri/3/3",
		"nanameguri/3/3/",
		// Out-of-range base-27 digit decodes as empty.
		"nanameguri/1/1/0z",
		// Real sample URLs.
		"https://puzz.link/p?nanameguri/5/5/03t4c04200p000020",
		"https://puzz.link/p?nanameguri/9/9/020o31c5g40g4008s4310jk4200ic0000180040163036180340786000",
		"https://puzz.link/p?nanameguri/18/10/00002010g08402008g00c15b00000g0000000000004vg6000000301000003fo02003a09000037nf33100012090a0032j300000621266i6i9o060f100000i000"
	],
	fileios: [
		// 3x3 with a wall, both slashes and a partially drawn loop.
		"pzprv3/nanameguri/3/3/0 0 0 0 /0 1 0 0 /0 0 0 0 /0 0 0 /1 0 0 /0 0 0 /0 0 0 /. 1 . /. 2 . /. . . /0 0 0 0 /0 1 0 0 /0 0 1 0 /0 0 0 /1 0 0 /0 1 0 /0 0 0 /",
		// 2x2 with a wall on the outer frame, both slashes and answer lines.
		"pzprv3/nanameguri/2/2/1 0 0 /0 0 0 /0 0 /0 0 /0 0 /1 2 /2 1 /0 1 0 /0 0 0 /0 0 /1 0 /0 0 /",
		// 1x1: only the frame borders.
		"pzprv3/nanameguri/1/1/0 0 /0 /0 /1 /0 0 /0 /0 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			cell.ques = r.pick([0, 0, 0, 0, 31, 32]);
		});
		p.board.border.each((border) => {
			border.ques = r.rnd() < 0.2 ? 1 : 0;
			border.line = 0;
			border.qsub = 0;
			// A line on the outer frame is not representable (pzprjs `isLineNG`).
			if (!border.inside) {
				return;
			}
			const x = r.rnd();
			if (x < 0.25) {
				border.line = 1;
			} else if (x < 0.4) {
				border.qsub = 2;
			}
		});
	}
};

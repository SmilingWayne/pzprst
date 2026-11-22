// Fixture inputs for pzprjs pid "yajitatami". See tools/gen_fixtures.js for the format.
"use strict";

module.exports = {
	pid: "yajitatami",
	dataset: null,
	urls: [
		"yajitatami/4/4",
		"yajitatami/4/4/",
		"yajitatami/4/4//",
		"https://puzz.link/p?yajitatami/4/4/10",
		"https://puzz.link/p?yajitatami_edit/4/4/10",
		"yajitatami/x/4/4/10",
		"yajitatami/4/4/0.",
		"yajitatami/4/4/+",
		"yajitatami/4/4/510",
		"yajitatami/4/4/-1100",
		"yajitatami/4/4/-13e7",
		"yajitatami/4/4/-1fff",
		"yajitatami/4/4/10zz",
		"yajitatami/4/4/p1g1",
		"yajitatami/4/4/p1g1zz",
		"yajitatami/4/4/g",
		"yajitatami/1/1/10",
		"yajitatami/1/5/1020304050",
		"yajitatami/5/1/1020304050",
		"yajitatami/25/1/10101010101010101010101010101010101010101010101010",
		"yajitatami/1/25/10101010101010101010101010101010101010101010101010",
		"https://puzz.link/p?yajitatami/10/10/e33h33k42c12d33f13d34a13n33b42b14d42j33g43e",
		"https://puzz.link/p?yajitatami/10/10/44n42j43g13g43g45o164233d12h18f32d32d12",
		"https://puzz.link/p?yajitatami/10/10/d43g32d42i33b24h324312m33j1623b13h33d43f16f32a",
		"yajitatami/10/10/39a1.b00a014aa+35c25964-012c+-3100-33e7a404.a0fe3fd00+b0.610b+c710+d+b0a-4100a0.-43e739a3.a+a3.a124.10-412cb2500a4.a0.1.a+20b2fb3aa1.0a1.+0a35a06800047068s00av83gh8q04g5e48610ock1",
		"yajitatami/8/6/a25401.-4100b+-212c4.+35764-212c9ff+0.39a610510b31d-3100b1.c-41001a4a29-23e7a0f30+2.ch0k0c0tj02u18f825",
		"yajitatami/6/8/29++b41b292.710+a6100.c-1100+b4.0.a+35b+a564+a0.b-03e72.710c+-33e7-412c42-112c76c518g9cc1n07h30"
	],
	fileios: [
		"pzprv3/yajitatami/3/2/0,1 2,- /3,4 # /0,10 . /0 /1 /0 /0 1 /0 0 /1 /0 /2 /0 1 /-1 0 /",
		"pzprv3/yajitatami/4/4/0,1 2,2 3,3 4,4 /# . 0,0 1,- /2,5 3,6 . 1,0 /4,7 0,8 . . /0 0 0 /0 1 0 /1 0 0 /0 0 0 /0 1 0 0 /1 0 0 0 /0 0 0 1 /0 0 0 /1 0 0 /0 0 1 /0 0 0 /2 0 1 0 /0 -1 0 0 /1 0 0 0 /",
		"pzprv3/yajitatami/2/2/0,1 2,- /# 3,4 /1 /0 /0 1 /2 /-1 /0 0 /",
		"pzprv3/yajitatami/1/1/3,5 /"
	],
	randomize(p, r) {
		p.board.cell.each((cell) => {
			const x = r.rnd();
			if (x < 0.1) {
				cell.qnum = -3;
				cell.qdir = 0;
			} else if (x < 0.2) {
				cell.qnum = -2;
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			} else if (x < 0.6) {
				cell.qnum = r.pick([0, 1, 2, 3, 5, 9, 10, 15, 16, 100, 255, 256, 300, 999]);
				cell.qdir = r.pick([0, 1, 2, 3, 4]);
			}
		});
		p.board.border.each((border) => {
			if (r.rnd() < 0.3) {
				border.ques = 1;
			}
			if (r.rnd() < 0.2) {
				border.qans = 1;
			}
			if (r.rnd() < 0.15) {
				border.qsub = 1;
			}
		});
	}
};

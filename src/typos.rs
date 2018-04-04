use words::Words;

pub struct Typo<'a> {
	word: &'a str,
	chance: f32,
}

impl<'a> Typo<'a> {
	fn new(word: &'a str, words: &Words<'a>) -> Typo<'a> {
		Typo {
			word: word,
			chance: get_chance(word, words),
		}
	}
}

fn get_chance<'a>(word: &str, words: &Words<'a>) -> f32 {
	let count: f32 = words.map.keys()
		.filter(|&x| word != x)
		.map(|other| {
			let freq = |x: &str| words.map.get(x).unwrap().len() as f32;
			let r = freq(other) / freq(word);

			let s = calc_similarity(word, other);

			if word.len() < 4 { return 0.; }

			r * s
		})
		.sum();

	const N: f32 = 1000f32;

	100f32 * count / (N + count)
}

fn one_char_off(a: &str, b: &str) -> bool {
	if a.len() + 1 != b.len() { return false; }

	for i in 0..a.len() {
		let mut s = a.to_string();
		s.remove(i);
		if &s == b {
			return true;
		}
	}

	false
}

fn sym_one_char_off(a: &str, b: &str) -> bool {
	one_char_off(a, b) || one_char_off(b, a)
}

fn one_switch_off(a: &str, b: &str) -> bool {
	if a.len() != b.len() { return false; }

	for i in 0..(a.len()-1) {
		let mut s = a.to_string();
		let c = s.remove(i);
		s.insert(i+1, c);
		if &s == b {
			return true;
		}
	}

	false
}

fn calc_similarity(a: &str, b: &str) -> f32 {
	if sym_one_char_off(a, b) {
		return 1.;
	} else if one_switch_off(a, b) {
		return 2.;
	}

	return 0.;
}

pub fn find_typos<'a>(words: &'a Words<'a>) -> Vec<Typo<'a>> {
	let mut typos = Vec::new();

	let l = words.map.keys().len();

	for (i, word) in words.map.keys().enumerate() {
		typos.push(Typo::new(word, &words));
		println!("{}/{}", i, l);
	}

	typos.sort_unstable_by(|a, b| b.chance.partial_cmp(&a.chance).unwrap());

	typos
}

pub fn dump_typos<'a>(words: &Words<'a>, typos: &[Typo<'a>]) {
	for typo in typos {
		if typo.chance > 0. {
			println!("{:?}: {:.2}%", typo.word, typo.chance);
			for occ in words.map.get(typo.word).unwrap() {
				println!(" @ {:?}:{}:{}-{}", occ.file, occ.line, occ.column, occ.column + typo.word.len());
			}
		}
	}
}

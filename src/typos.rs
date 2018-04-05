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
	let freq = |x: &str| words.map.get(x).unwrap().len();

	100.
		* words.map.keys()
			.filter(|&x| word != x)
			.map(|other| calc_similarity(word, other))
			.fold(0.0, |x: f32, y: f32| x.max(y))
		* 1. / (10f32).powi((freq(word) - 1) as i32)
		* match word.len() {
			0 => 0.0,
			1 => 0.0,
			2 => 0.001,
			3 => 0.01,
			4 => 0.5,
			_ => 1.0,
		}
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
		return 0.5;
	} else if one_switch_off(a, b) {
		return 0.9;
	}

	return 0.0;
}

pub fn find_typos<'a>(words: &'a Words<'a>) -> Vec<Typo<'a>> {
	let mut typos = Vec::new();

	let l = words.map.keys().len();

	for (i, word) in words.map.keys().enumerate() {
		typos.push(Typo::new(word, &words));
		println!("{}/{}", i+1, l);
	}

	typos.sort_unstable_by(|a, b| b.chance.partial_cmp(&a.chance).unwrap());

	typos
}

pub fn dump_typos<'a>(words: &Words<'a>, typos: &[Typo<'a>]) {
	for typo in typos {
		if typo.chance >= 1.0 {
			println!("{:?}: {:.2}%", typo.word, typo.chance);
			for occ in words.map.get(typo.word).unwrap() {
				println!(" @ {:?}:{},{}-{}", occ.file, occ.line, occ.column, occ.column + typo.word.len() - 1);
			}
		}
	}
}

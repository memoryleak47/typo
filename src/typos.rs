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
		.map(|other| {
			let freq = |x: &str| words.map.get(x).unwrap().len() as f32;
			let r = freq(other) / freq(word);

			let s = calc_similarity(word, other);

			r * s
		})
		.sum();

	const N: f32 = 1000f32;

	100f32 * (count / (N + count))
}

fn calc_similarity(a: &str, b: &str) -> f32 { // TODO
	1f32
}

pub fn find_typos<'a>(words: &'a Words<'a>) -> Vec<Typo<'a>> {
	let mut typos = Vec::new();

	for word in words.map.keys() {
		typos.push(Typo::new(word, &words));
	}

	typos.sort_unstable_by(|a, b| b.chance.partial_cmp(&a.chance).unwrap());

	typos
}

pub fn dump_typos<'a>(words: &Words<'a>, typos: &[Typo<'a>]) {
	for typo in typos {
		println!("{:?}: {:.2}%", typo.word, typo.chance);
		for occ in words.map.get(typo.word).unwrap() {
			println!(" @ {:?}:{}:{}-{}", occ.file, occ.line, occ.column, occ.column + typo.word.len());
		}
	}
}

use std::path::Path;

use words::{Words, Occurence};

pub struct Typo<'a> {
	word: String,
	file: &'a Path,
	line: usize,
	column: usize,
	// TODO suggestion: String,
}

impl<'a> Typo<'a> {
	fn from_occurence(name: String, occ: &Occurence<'a>) -> Typo<'a> {
		Typo {
			word: name,
			file: occ.file,
			line: occ.line,
			column: occ.column,
		}
	}
}

pub fn find_typos<'a>(words: Words<'a>) -> Vec<Typo<'a>> {
	// TODO find better algorithm
	// this currently just searchs seldomly-used words, it should also check, whether there are similar other words

	let mut typos = Vec::new();

	for (name, occurences) in words.map.iter() {
		if occurences.len() < 3 {
			for occ in occurences {
				typos.push(Typo::from_occurence(name.to_string(), occ));
			}
		}
	}

	typos
}

pub fn dump_typos<'a>(typos: Vec<Typo<'a>>) {
	for typo in typos {
		println!("{}", typo.word);
	}
}

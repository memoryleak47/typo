use std::{
	path::{Path, PathBuf},
	collections::HashMap,
	fs::File,
	io::Read,
};

#[derive(Clone)]
pub struct Occurence<'a> {
	pub file: &'a Path,
	pub line: usize,
	pub column: usize,
}

#[derive(Clone)]
pub struct Words<'a> {
	pub map: HashMap<String, Vec<Occurence<'a>>>
}

impl<'a> Words<'a> {
	pub fn new() -> Words<'a> {
		Words { map: HashMap::new() }
	}

	fn from(path: &'a Path) -> Words<'a> {
		let s = read(path);
		let mut words = Words::new();

		let mut current_word = String::new();
		for (line, line_string) in s.lines().enumerate() {

			for (column, c) in line_string.chars().rev().enumerate() {
				if c.is_alphabetic() || c == '_' {
					current_word.insert(0, c);
				} else if !current_word.is_empty() {
					words.add_occurence(current_word, Occurence { file: path, line, column: column + 1 });
					current_word = String::new();
				}
			}
			if !current_word.is_empty() {
				words.add_occurence(current_word, Occurence { file: path, line, column: 0 });
				current_word = String::new();
			}
		}

		words
	}

	fn add_occurence(&mut self, name: String, occ: Occurence<'a>) {
		let mut occurences = self.map.remove(&name)
			.unwrap_or_else(|| Vec::new());
		occurences.push(occ);
		self.map.insert(name, occurences);
	}

	fn merge(mut a: Words<'a>, b: Words<'a>) -> Words<'a> {
		for (name, mut b_occurences) in b.map.into_iter() {
			if let Some(a_occurences) = a.map.remove(&name) {
				b_occurences.extend(a_occurences);
			}

			a.map.insert(name, b_occurences);
		}
		a
	}
}

fn read(file: &Path) -> String {
	let mut s = String::new();
	File::open(file).unwrap()
		.read_to_string(&mut s).unwrap();
	s
}

pub fn find_words<'a>(files: &'a [PathBuf]) -> Words<'a> {
	files.iter()
		.map(|file| Words::from(file.as_path()))
		.fold(Words::new(),
			|old_words, new_words| Words::merge(old_words, new_words)
		)
}

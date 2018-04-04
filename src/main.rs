pub type Res<T> = Result<T, String>;

mod files;
mod words;
mod typos;

use std::env;

fn main() {
	let path = env::args().nth(1)
		.unwrap_or_else(|| panic!("Missing argument"));
	let files = files::find_files(path)
		.unwrap_or_else(|x| panic!(x));
	let words = words::find_words(&files[..]);
	let typos = typos::find_typos(words);

	println!("typo candidates:\n{}", typos);
}

pub type Res<T> = Result<T, String>;

mod files;
mod words;
mod typos;

use std::env;

fn main() {
	let path = env::args().nth(1)
		.unwrap_or_else(|| panic!("Missing argument"));
	println!("did path");
	let files = files::find_files(path)
		.unwrap_or_else(|x| panic!(x));
	println!("did files");
	let words = words::find_words(&files[..]);
	println!("did words");
	let typos = typos::find_typos(&words);
	println!("done!");
	typos::dump_typos(&words, &typos[..]);
}

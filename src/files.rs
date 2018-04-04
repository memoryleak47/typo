use std::{
	path::PathBuf,
	fs::read_dir,
};

use ::Res;

pub fn find_files<T: Into<PathBuf>>(t: T) -> Res<Vec<PathBuf>> {
	let pbuf = t.into();

	return if pbuf.is_file() {
		Ok(vec![pbuf])
	} else if pbuf.is_dir() {
		let rd = read_dir(pbuf)
			.map_err(|x| x.to_string())?;
		rd.map(|entry| -> Res<Vec<PathBuf>> {
			find_files(entry.unwrap().path())
		})
		.fold(Ok(Vec::new()), |res_v: Res<Vec<PathBuf>>, res_new: Res<Vec<PathBuf>>| {
			res_v.and_then(|mut v|
				res_new.map(|new| { v.extend(new); v })
			)
		})
	} else if !pbuf.exists() {
		Err(format!("File {:?} does not exist", pbuf))
	} else {
		panic!("unknown case, this is a bug")
	};
}

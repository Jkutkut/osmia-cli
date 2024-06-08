use std::io::{Read, stdin};

pub fn read_stdin() -> Result<String, String> {
	let mut buffer = String::new();
	match stdin().read_to_string(&mut buffer) {
		Ok(_) => Ok(buffer),
		Err(e) => Err(e.to_string())
	}
}

pub fn read_file(file_path: &str) -> Result<String, String> {
	let mut buffer = String::new();
	match std::fs::File::open(file_path) {
		Ok(mut file) => match file.read_to_string(&mut buffer) {
			Ok(_) => Ok(buffer),
			Err(e) => Err(e.to_string())
		},
		Err(e) => Err(e.to_string())
	}
}

#[macro_export]
macro_rules! fail {
	($($arg:tt)*) => {
		{
			eprintln!($($arg)*);
			std::process::exit(1);
		}
	}
}

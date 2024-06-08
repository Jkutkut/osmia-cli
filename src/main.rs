use osmia::Osmia;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod macros;
mod utils;
mod constants;

use utils::{
	read_stdin, read_file,
	// fail!
};
use constants::{VERSION, HELP, BIN_NAME};

fn main() {
	let mut ctx: Option<String> = None;
	let mut code: Option<String> = None;

	let mut args = std::env::args();
	args.next().unwrap();
	while let Some(arg) = args.next() {
		match arg.as_str() {
			"-h" | "--help" => {
				println!("{}", HELP);
				return;
			},
			"-v" | "--version" => {
				println!(
					"{} v{} using osmia v{}",
					BIN_NAME,
					VERSION,
					Osmia::VERSION
				);
				return;
			},
			"--ctx" => ctx = match args.next() {
				None => fail!("Expected a json file after --ctx"),
				Some(s) => match read_file(&s) {
					Ok(s) => Some(s),
					Err(err) => fail!("Error reading file {}: {}", s, err)
				}
			},
			"--ctx-in" => ctx = match read_stdin() {
				Ok(s) => Some(s),
				Err(err) => fail!("{}", err)
			},
			"--ctx-str" => ctx = match args.next() {
				None => fail!("Expected a json string after --ctx-str"),
				s => s
			},
			"--code" => code = match args.next() {
				None => fail!("Expected an osmia file after --code"),
				Some(s) => match read_file(&s) {
					Ok(s) => Some(s),
					Err(err) => fail!("Error reading file {}: {}", s, err)
				}
			},
			"--code-in" => code = match read_stdin() {
				Ok(s) => Some(s),
				Err(err) => fail!("{}", err)
			},
			"--code-str" => code = match args.next() {
				None => fail!("Expected osmia code after --code-str"),
				s => s
			},
			_ => fail!("Invalid argument: {}", arg),
		}
	}

	let mut interpreter = match ctx {
		Some(ctx) => match Osmia::from_json(&ctx) {
			Ok(interpreter) => interpreter,
			Err(err) => fail!("{}", err)
		},
		None => Osmia::new()
	};

	let code_str = match code {
		Some(code) => code,
		None => fail!("No code provided")
	};
	let osmia_code = match Osmia::code(&code_str) {
		Ok(code) => code,
		Err(err) => fail!("{}", err)
	};
	let result = match interpreter.run(&osmia_code) {
		Ok(result) => result,
		Err(err) => fail!("{}", err)
	};
	print!("{}", result);
}

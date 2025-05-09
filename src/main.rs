#[cfg(test)]
use macro_test::macro_tests;
use osmia::Osmia;
use osmia::CodeInterpreter;

#[cfg(test)]
mod tests;
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
					Err(err) => fail!("Error reading json file {}: {}", s, err)
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
					Err(err) => fail!("Error reading osmia file {}: {}", s, err)
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

	if let None = code {
		fail!("No code provided");
	}
	let mut osmia = match ctx {
		None => Osmia::default(),
		Some(ctx) => match Osmia::try_from(ctx.as_str()) {
			Ok(osmia) => osmia,
			Err(err) => fail!("Invalid context json: {}", err)
		}
	};
	osmia.run_code(&format!("{{{{ _OSMIA_CLI_VERSION = \"{}\" }}}}", VERSION)).unwrap();
	match osmia.run_code(&code.unwrap()) {
		Ok(result) => print!("{}", result),
		Err(err) => fail!("{}", err)
	}
}

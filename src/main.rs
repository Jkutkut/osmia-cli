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

enum CtxLang {
	JSON,
	YAML
}

impl std::fmt::Display for CtxLang {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			CtxLang::JSON => write!(f, "json"),
			CtxLang::YAML => write!(f, "yaml")
		}
	}
}

struct CtxArg {
	lang: CtxLang,
	data: String
}

impl CtxArg {
	fn new(lang: CtxLang, data: String) -> CtxArg {
		CtxArg {
			lang,
			data
		}
	}
}

fn main() {
	let mut ctx: Option<CtxArg> = None;
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
			"--ctx" | "--ctx-json" => ctx = match args.next() {
				None => fail!("Expected a json file after --ctx-json"),
				Some(s) => match read_file(&s) {
					Ok(s) => Some(CtxArg::new(CtxLang::JSON, s)),
					Err(err) => fail!("Error reading json file {}: {}", s, err)
				}
			},
			"--ctx-in" | "--ctx-json-in" => ctx = match read_stdin() {
				Ok(s) => Some(CtxArg::new(CtxLang::JSON, s)),
				Err(err) => fail!("{}", err)
			},
			"--ctx-str" | "--ctx-json-str" => ctx = match args.next() {
				None => fail!("Expected a json string after --ctx-str"),
				Some(s) => Some(CtxArg::new(CtxLang::JSON, s)),
			},
			"--ctx-yaml" => ctx = match args.next() {
				None => fail!("Expected a yaml file after --ctx-yaml"),
				Some(s) => match read_file(&s) {
					Ok(s) => Some(CtxArg::new(CtxLang::YAML, s)),
					Err(err) => fail!("Error reading yaml file {}: {}", s, err)
				}
			},
			"--ctx-yaml-in" => ctx = match read_stdin() {
				Ok(s) => Some(CtxArg::new(CtxLang::YAML, s)),
				Err(err) => fail!("{}", err)
			},
			"--ctx-yaml-str" => ctx = match args.next() {
				None => fail!("Expected a yaml string after --ctx-str"),
				Some(s) => Some(CtxArg::new(CtxLang::YAML, s)),
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
		Some(ctx) => {
			let osmia_result = match ctx.lang {
				CtxLang::JSON => Osmia::try_from_json(&ctx.data),
				CtxLang::YAML => Osmia::try_from_yaml(&ctx.data)
			};
			match osmia_result {
				Ok(osmia) => osmia,
				Err(err) => fail!("Invalid context {}: {}", ctx.lang.to_string(), err)
			}
		}
	};
	osmia.run_code(&format!("{{{{ _OSMIA_CLI_VERSION = \"{}\" }}}}", VERSION)).unwrap();
	match osmia.run_code(&code.unwrap()) {
		Ok(result) => print!("{}", result),
		Err(err) => fail!("{}", err)
	}
}

use osmia::Osmia;
use std::io::Read;

fn read_stdin() -> String {
	let mut buffer = String::new();
	std::io::stdin().read_to_string(&mut buffer).unwrap();
	buffer
}

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const HELP: &str =
"--code          -- Add code as a JSON file                  
--code-in       -- Add code as a JSON string from stdin     
--code-str      -- Add code as a JSON string                
--ctx           -- Add context as a JSON file               
--ctx-in        -- Add context as a JSON string from stdin  
--ctx-str       -- Add context as a JSON string             
--help      -h  -- Display help information                 
--version   -v  -- Display current version";

fn main() {
	let mut ctx: Option<String> = None;

	let mut args = std::env::args();
	args.next().unwrap();
	while let Some(arg) = args.next() {
		if arg == "--ctx" {
			ctx = Some(args.next().unwrap_or_else(|| {
				println!("Ctx:");
				let c = read_stdin();
				println!("\nCode:");
				c
			}));
		}
		else if arg == "--help" {
			println!("{}", HELP);
			return;
		}
		else if arg == "-v" {
			println!(
				"osmia cli v{} using osmia v{}",
				VERSION.unwrap_or("unknown"),
				Osmia::VERSION
			);
			return;
		}
		else {
			eprintln!("Invalid argument: {}", arg);
			println!("{}", HELP);
			return;
		}
	}

	let mut interpreter = match ctx {
		Some(ctx) => match Osmia::from_json(&ctx) {
			Ok(interpreter) => interpreter,
			Err(err) => {
				eprintln!("{}", err);
				return;
			}
		},
		None => Osmia::new()
	};
	let code = read_stdin();

	let code = Osmia::code(code.as_str()).unwrap();
	let result = interpreter.run(&code).unwrap();
	println!("{}", result);
}

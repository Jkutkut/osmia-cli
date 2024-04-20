use osmia::Osmia;
use std::io::Read;

fn read_stdin() -> String {
	let mut buffer = String::new();
	std::io::stdin().read_to_string(&mut buffer).unwrap();
	buffer
}

fn main() {
	let mut ctx: Option<String> = None;

	let mut args = std::env::args();
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
			println!("Usage: osmia [--ctx [json-object]] [--help]");
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

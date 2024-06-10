use std::process::{
	Command, Stdio, Output
};
use std::io::{ Write, Error };
use std::thread;

use super::{CmdArg, BIN_NAME};

fn calc_bin_path(filename: &str) -> String {
	if cfg!(debug_assertions) {
		format!("target/debug/{}", filename)
	}
	else {
		format!("target/release/{}", filename)
	}
}

fn run_command_stdin(mut cmd: Command, stdin: &str) -> Result<Output, Error> {
	let mut cmd = cmd
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn().unwrap();
	let mut child_stdin = cmd.stdin.take().unwrap();
	let stdin = stdin.to_string();
	thread::spawn(move || {
		child_stdin.write_all(stdin.as_bytes()).unwrap();
	});
	cmd.wait_with_output()
}

fn run_command(
	args: CmdArg,
	stdin: Option<&str>
) -> Result<String, String> {
	println!("Running command: {:?}", args);
	let args = match args {
		CmdArg::Str(s) => s.split(' ').collect::<Vec<&str>>(),
		CmdArg::Arr(a) => a
	};
	let bin_path = calc_bin_path(BIN_NAME);
	let mut cmd = Command::new(bin_path);
	if args.len() > 0 {
		cmd.args(args);
	}
	let output = match stdin {
		Some(stdin) => run_command_stdin(cmd, stdin),
		None => cmd.output()
	};
	match output {
		Ok(output) => match output.status.success() {
			true => Ok(String::from_utf8(output.stdout).unwrap()),
			false => Err(String::from_utf8(output.stderr).unwrap())
		},
		Err(err) => Err(err.to_string())
	}
}

fn fail(msg: &str) {
	assert!(false, "{}", msg);
}

pub fn test_valid_contains(args: CmdArg, should_contain: &str) {
	let r = match run_command(args, None) {
		Err(err) => return fail(&format!("Unexpected error: {:?}", err)),
		Ok(r) => r
	};
	assert!(r.contains(should_contain));
}

pub fn test_valid_exact(args: CmdArg, stdin: Option<&str>, should_equal: &str) {
	let r = match run_command(args, stdin) {
		Err(err) => return fail(&format!("Unexpected error: {:?}", err)),
		Ok(r) => r
	};
	assert_eq!(r, should_equal);
}

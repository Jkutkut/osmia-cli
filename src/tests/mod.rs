use std::process::Command;
use std::process::Stdio;
use std::io::Write;
use std::thread;

use crate::macro_tests;
use crate::constants::{VERSION, BIN_NAME, HELP};

fn calc_bin_path(filename: &str) -> String {
	if cfg!(debug_assertions) {
		format!("target/debug/{}", filename)
	}
	else {
		format!("target/release/{}", filename)
	}
}

#[derive(Debug)]
enum CmdArg<'a> {
	Str(&'a str),
	Arr(Vec<&'a str>),
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
	let output;
	if let Some(stdin) = stdin {
		let mut cmd = cmd
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn().unwrap();
		let mut child_stdin = cmd.stdin.take().unwrap();
		let stdin = stdin.to_string();
		thread::spawn(move || {
			child_stdin.write_all(stdin.as_bytes()).unwrap();
		});
		output = cmd.wait_with_output();
	}
	else {
		output = cmd.output();
	}
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

fn test_valid_contains(args: CmdArg, should_contain: &str) {
	let r = match run_command(args, None) {
		Err(err) => return fail(&format!("Unexpected error: {:?}", err)),
		Ok(r) => r
	};
	assert!(r.contains(should_contain));
}

fn test_valid_exact(args: CmdArg, stdin: Option<&str>, should_equal: &str) {
	let r = match run_command(args, stdin) {
		Err(err) => return fail(&format!("Unexpected error: {:?}", err)),
		Ok(r) => r
	};
	assert_eq!(r, should_equal);
}

macro_tests!(
	test_valid_contains,
	( arg_v_01, CmdArg::Str("-v"), VERSION ),
	( arg_v_02, CmdArg::Str("--version"), VERSION ),
	( arg_h_01, CmdArg::Str("-h"), HELP ),
	( arg_h_02, CmdArg::Str("--help"), HELP )
);

macro_tests!(
	test_valid_exact,
	(
		arg_ctx_01,
		CmdArg::Arr(vec![
			"--ctx", "src/tests/data/data.json",
			"--code-str", "{{ program }}"
		]),
		None,
		"osmia-cli"
	),
	(
		arg_ctx_str_01,
		CmdArg::Arr(vec![
			"--ctx-str", r#"{"program": "osmia-cli"}"#,
			"--code-str", "{{ program }}"
		]),
		None,
		"osmia-cli"
	),
	(
		arg_ctx_in_01,
		CmdArg::Arr(vec![
			"--ctx-in",
			"--code-str", "{{ program }}"
		]),
		Some(r#"{"program": "osmia-cli"}"#),
		"osmia-cli"
	),
	(
		arg_code_01,
		CmdArg::Arr(vec![
			"--ctx-str", r#"{"program": "osmia-cli"}"#,
			"--code", "src/tests/data/program.osmia"
		]),
		None,
		"osmia-cli\n"
	),
	(
		arg_code_str_01,
		CmdArg::Arr(vec![
			"--ctx-str", r#"{"program": "osmia-cli"}"#,
			"--code-str", "{{ program }}"
		]),
		None,
		"osmia-cli"
	),
	(
		arg_code_in_01,
		CmdArg::Arr(vec![
			"--code-in"
		]),
		Some("osmia-cli {{ 1 + 2 }}\n"),
		"osmia-cli 3\n"
	)
);

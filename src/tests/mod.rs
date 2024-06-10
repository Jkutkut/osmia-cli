mod utils;
mod models;

use crate::macro_tests;
use crate::constants::{VERSION, BIN_NAME, HELP};
use utils::{
	test_valid_contains, test_valid_exact,
	test_invalid_contains
};
use models::{
	CmdArg
};

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
	),
	(
		empty_code,
		CmdArg::Arr(vec![
			"--code-in"
		]),
		None,
		""
	)
);

macro_tests!(
	test_invalid_contains,
	(
		no_code_01,
		CmdArg::Arr(vec![]),
		"code"
	),
	(
		no_code_02,
		CmdArg::Arr(vec![
			"--ctx", "src/tests/data/data.json",
		]),
		"code"
	),
	(
		invalid_ctx_01,
		CmdArg::Arr(vec![
			"--ctx", "not-valid"
		]),
		"json"
	),
	(
		invalid_ctx_02,
		CmdArg::Arr(vec![
			"--ctx-str", "not a json"
		]),
		"json"
	),
	(
		invalid_ctx_03,
		CmdArg::Arr(vec![
			"--ctx-in"
		]),
		"json"
	),
	(
		invalid_code_01,
		CmdArg::Arr(vec![
			"--code", "not a file"
		]),
		"osmia"
	)
);

#[derive(Debug)]
pub enum CmdArg<'a> {
	Str(&'a str),
	Arr(Vec<&'a str>),
}

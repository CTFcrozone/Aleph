use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliCmd {
	#[command(subcommand)]
	pub command: Option<CliSubCmd>,
}

#[derive(Subcommand, Debug)]
pub enum CliSubCmd {
	/// Show binary info
	Info(FileArgs),

	/// List sections
	Sections(FileArgs),

	/// List segments
	Segments(FileArgs),

	/// List symbols
	Symbols(FileArgs),
}

#[derive(Args, Debug)]
pub struct FileArgs {
	pub path: String,
}

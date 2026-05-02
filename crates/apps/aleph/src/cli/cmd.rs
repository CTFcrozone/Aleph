use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "Aleph — binary analysis toolkit")]
pub struct CliCmd {
	#[arg(long)]
	pub path: String,

	#[command(subcommand)]
	pub command: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
	Disasm {
		#[arg(long)]
		section: String,
		#[arg(long = "max-insns")]
		max_insns: Option<usize>,
	},
	Info,
	Sections,
	Segments,
	Symbols,
}

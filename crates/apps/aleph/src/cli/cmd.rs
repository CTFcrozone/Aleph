use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about = "Aleph — binary analysis toolkit")]
pub struct CliCmd {
	/// Path to binary
	#[arg(long)]
	pub path: String,

	/// Operation mode
	#[arg(long)]
	pub mode: Mode,

	/// Section (used for disasm)
	#[arg(long)]
	pub section: Option<String>,

	/// Maximum instructions for disassembly
	#[arg(long = "max-insns")]
	pub max_insns: Option<usize>,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Mode {
	Info,
	Sections,
	Segments,
	Symbols,
	Disasm,
}

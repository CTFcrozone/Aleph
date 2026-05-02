use clap::Parser as _;
use lib_elf::{Binary, arch, disasm, entry, parse_binary, sections, segments, symbols};

use crate::{
	Result,
	cli::cmd::{CliCmd, Mode},
};

pub fn execute() -> Result<()> {
	let cli = CliCmd::parse();

	let bin = Binary::new(&cli.path)?;
	let file = parse_binary(&bin)?;

	match cli.command {
		Mode::Info => {
			println!("Format: {:?}", lib_elf::format(&file));
			println!("Arch:   {:?}", arch(&file));
			println!("Entry:  0x{:x}", entry(&file));
		}

		Mode::Sections => {
			for s in sections(&file) {
				println!("{s}");
			}
		}

		Mode::Segments => {
			for s in segments(&file) {
				println!("{s}");
			}
		}

		Mode::Symbols => {
			for s in symbols(&file) {
				println!("{s}");
			}
		}

		Mode::Disasm { section, max_insns } => {
			let insns = disasm(&file, &section, max_insns)?;

			for i in insns {
				println!("{i}");
			}
		}
	}

	Ok(())
}

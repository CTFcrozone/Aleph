use clap::Parser as _;
use lib_elf::{Binary, arch, entry, parse_binary, sections, segments, symbols};

use crate::{
	Result,
	cli::cmd::{CliCmd, CliSubCmd},
};

pub fn execute() -> Result<()> {
	let cli_cmd = CliCmd::parse();
	let Some(sub_cmd) = cli_cmd.command else {
		println!("Aleph — binary analysis toolkit");
		return Ok(());
	};

	match sub_cmd {
		CliSubCmd::Info(args) => {
			let bin = Binary::new(args.path)?;
			let file = parse_binary(&bin)?;

			println!("Format: {:?}", lib_elf::format(&file));
			println!("Arch:   {:?}", arch(&file));
			println!("Entry:  0x{:x}", entry(&file));
		}

		CliSubCmd::Sections(args) => {
			let bin = Binary::new(args.path)?;
			let file = parse_binary(&bin)?;

			for s in sections(&file) {
				println!("{s}");
			}
		}

		CliSubCmd::Segments(args) => {
			let bin = Binary::new(args.path)?;
			let file = parse_binary(&bin)?;

			for s in segments(&file) {
				println!("{s}");
			}
		}

		CliSubCmd::Symbols(args) => {
			let bin = Binary::new(args.path)?;
			let file = parse_binary(&bin)?;

			for s in symbols(&file) {
				println!("{s}");
			}
		}
	}

	Ok(())
}

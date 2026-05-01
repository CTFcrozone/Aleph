use capstone::{Capstone, arch::BuildsCapstone};
use object::{
	Architecture, BinaryFormat, Endianness, File, Object, ObjectKind, ObjectSection, ObjectSegment, ObjectSymbol,
	Permissions, SectionFlags, SectionKind, SegmentFlags, SymbolKind, SymbolScope,
};

use crate::{Error, error::Result};

#[derive(Debug)]
pub struct SectionInfo {
	pub name: String,
	pub addr: u64,
	pub size: u64,
	pub align: u64,
	pub kind: SectionKind,
	pub flags: SectionFlags,
}

#[derive(Debug)]
pub struct SegmentInfo {
	pub name: String,
	pub addr: u64,
	pub size: u64,
	pub align: u64,
	pub flags: SegmentFlags,
	pub permissions: Permissions,
}

#[derive(Debug)]
pub struct SymbolInfo {
	pub name: String,
	pub addr: u64,
	pub size: u64,
	pub kind: SymbolKind,
	pub scope: SymbolScope,
}

#[derive(Debug)]
pub struct InstructionInfo {
	pub addr: u64,
	pub bytes: Vec<u8>,
	pub mnemonic: String,
	pub op_str: String,
}

// region:    --- Functions
pub fn format(file: &File) -> BinaryFormat {
	file.format()
}

pub fn arch(file: &File) -> Architecture {
	file.architecture()
}

pub fn entry(file: &File) -> u64 {
	file.entry()
}

pub fn kind(file: &File) -> ObjectKind {
	file.kind()
}

pub fn endianness(file: &File) -> Endianness {
	file.endianness()
}

pub fn segments(file: &File) -> Vec<SegmentInfo> {
	let infos = file
		.segments()
		.map(|s| SegmentInfo {
			name: s.name().ok().flatten().unwrap_or("<unknown>").to_string(),
			addr: s.address(),
			size: s.size(),
			align: s.align(),
			flags: s.flags(),
			permissions: s.permissions(),
		})
		.collect();
	infos
}

pub fn sections(file: &File) -> Vec<SectionInfo> {
	let infos = file
		.sections()
		.map(|s| SectionInfo {
			name: s.name().unwrap_or("<unknown>").to_string(),
			addr: s.address(),
			size: s.size(),
			align: s.align(),
			kind: s.kind(),
			flags: s.flags(),
		})
		.collect();
	infos
}

pub fn symbols(file: &File) -> Vec<SymbolInfo> {
	let infos = file
		.symbols()
		.map(|s| SymbolInfo {
			name: s.name().unwrap_or("<unknown>").to_string(),
			addr: s.address(),
			size: s.size(),
			kind: s.kind(),
			scope: s.scope(),
		})
		.collect();
	infos
}

pub fn disasm(file: &File, section_name: &str, max_insns: Option<usize>) -> Result<Vec<InstructionInfo>> {
	let section = file.section_by_name(section_name).ok_or(Error::SectionNotFound {
		section: section_name.into(),
	})?;

	let code = section.uncompressed_data()?;
	let addr = section.address();

	let mut cs = match file.architecture() {
		Architecture::X86_64 => Capstone::new().x86().mode(capstone::arch::x86::ArchMode::Mode64).build()?,
		Architecture::X86_64_X32 | Architecture::I386 => {
			Capstone::new().x86().mode(capstone::arch::x86::ArchMode::Mode32).build()?
		}
		_ => {
			return Err(Error::UnsupportedArch {
				arch: file.architecture(),
			});
		}
	};

	let endian = match file.endianness() {
		Endianness::Little => capstone::Endian::Little,
		Endianness::Big => capstone::Endian::Big,
	};

	cs.set_endian(endian)?;

	let insns = match max_insns {
		Some(n) => cs.disasm_count(code.as_ref(), addr, n)?,
		None => cs.disasm_all(code.as_ref(), addr)?,
	};

	let infos: Vec<_> = insns
		.iter()
		.map(|i| InstructionInfo {
			addr: i.address(),
			bytes: i.bytes().to_vec(),
			mnemonic: i.mnemonic().unwrap_or("<invalid>").to_string(),
			op_str: i.op_str().unwrap_or("").to_string(),
		})
		.collect();

	Ok(infos)
}
// endregion: --- Functions

// region:    --- Tests
#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use crate::{binary::Binary, parse::parse_binary};

	use super::*;

	const BIN_PATH: &str = "test-data/kdd";

	#[test]
	fn parse_basic_metadata_ok() -> Result<()> {
		// -- Setup & Fixtures
		let binary = Binary::new(BIN_PATH)?;
		let file = parse_binary(&binary)?;
		// -- Exec
		let fmt = format(&file);
		let arch = arch(&file);
		let entry = entry(&file);
		// -- Check
		assert_ne!(format!("{:?}", fmt), "");
		assert_ne!(format!("{:?}", arch), "");
		assert_ne!(entry, 0);

		Ok(())
	}

	#[test]
	fn parse_structural_data_ok() -> Result<()> {
		// -- Setup & Fixtures
		let binary = Binary::new(BIN_PATH)?;
		let file = parse_binary(&binary)?;
		// -- Exec
		let secs = sections(&file);
		let segs = segments(&file);
		let syms = symbols(&file);
		// -- Check
		assert!(!secs.is_empty());
		assert!(!segs.is_empty());
		assert!(!syms.is_empty());
		Ok(())
	}

	#[test]
	fn text_section_exists_and_valid() -> Result<()> {
		// -- Setup & Fixtures
		let binary = Binary::new(BIN_PATH)?;
		let file = parse_binary(&binary)?;
		// -- Exec
		let secs = sections(&file);
		let text = secs.iter().find(|s| s.name == ".text");
		// -- Check
		assert!(text.is_some());
		let text = text.unwrap();
		assert!(text.size > 0);
		assert!(text.addr != 0);

		Ok(())
	}

	#[test]
	fn entry_point_is_in_executable_range() -> Result<()> {
		// -- Setup & Fixtures
		let binary = Binary::new(BIN_PATH)?;
		let file = parse_binary(&binary)?;
		// -- Exec
		let entry = entry(&file);
		let secs = sections(&file);
		let text = secs.iter().find(|s| s.name == ".text");
		// -- Check
		assert!(text.is_some());
		let text = text.unwrap();
		assert!(entry >= text.addr);
		assert!(entry < text.addr + text.size);
		Ok(())
	}
}

// endregion: --- Tests

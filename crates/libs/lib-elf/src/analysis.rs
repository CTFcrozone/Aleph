use crate::error::Result;
use object::{Architecture, BinaryFormat, File, Object, ObjectSection};

#[derive(Debug)]
pub struct SectionInfo {
	pub name: String,
	pub addr: u64,
	pub size: u64,
}

pub fn format(file: &File) -> BinaryFormat {
	file.format()
}

pub fn arch(file: &File) -> Architecture {
	file.architecture()
}

pub fn sections(file: &File) -> Vec<SectionInfo> {
	let infos = file
		.sections()
		.map(|s| SectionInfo {
			name: s.name().unwrap_or("<unknown>").to_string(),
			addr: s.address(),
			size: s.size(),
		})
		.collect();
	infos
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use crate::{binary::Binary, parse::parse_binary};

	use super::*;

	const BIN_PATH: &str = "test-data/kdd";

	#[test]
	fn parse_sections_ok() -> Result<()> {
		// -- Setup & Fixtures
		let binary = Binary::new(BIN_PATH)?;
		let file = parse_binary(&binary)?;
		// -- Exec
		let secs = sections(&file);
		// -- Check
		assert_eq!(secs.len(), 34);
		Ok(())
	}
}

// endregion: --- Tests

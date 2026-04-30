use std::ops::Deref;

use object::{Architecture, BinaryFormat, File, Object, ObjectSection, Section};

use crate::error::Result;

pub struct Binary {
	pub path: String,
	data: Vec<u8>,
}

#[derive(Debug)]
pub struct SectionInfo {
	pub name: String,
	pub addr: u64,
	pub size: u64,
}

pub fn parse_binary(binary: &Binary) -> Result<File<'_>> {
	Ok(File::parse(binary.data.as_ref())?)
}

impl Binary {
	pub fn new(path: impl Into<String>) -> Result<Self> {
		let path_str = path.into();
		let data = std::fs::read(&path_str)?;

		Ok(Self { path: path_str, data })
	}

	pub fn new_from_bytes(data: impl AsRef<[u8]>) -> Result<Self> {
		Ok(Self {
			path: "<memory>".into(),
			data: data.as_ref().to_vec(),
		})
	}

	pub fn format(file: &File) -> Result<BinaryFormat> {
		Ok(file.format())
	}

	pub fn arch(file: &File) -> Result<Architecture> {
		Ok(file.architecture())
	}

	pub fn sections(file: &File) -> Result<Vec<SectionInfo>> {
		let infos = file
			.sections()
			.map(|s| SectionInfo {
				name: s.name().unwrap_or("<unknown>").to_string(),
				addr: s.address(),
				size: s.size(),
			})
			.collect();
		Ok(infos)
	}
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use super::*;

	const BIN_PATH: &str = "test-data/kdd";

	#[test]
	fn parse_sections_ok() -> Result<()> {
		// -- Setup & Fixtures
		let binary = Binary::new(BIN_PATH)?;
		let file = parse_binary(&binary)?;
		// -- Exec
		let sections = Binary::sections(&file)?;
		// -- Check
		assert_eq!(sections.len(), 34);
		Ok(())
	}
}

// endregion: --- Tests

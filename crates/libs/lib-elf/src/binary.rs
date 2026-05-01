use std::ops::Deref;

use object::{Architecture, BinaryFormat, File, Object, ObjectSection, Section};

use crate::error::Result;

pub struct Binary {
	path: String,
	data: Vec<u8>,
}

impl Binary {
	pub fn new(path: impl Into<String>) -> Result<Self> {
		let path_str = path.into();
		let data = std::fs::read(&path_str)?;

		Ok(Self { path: path_str, data })
	}

	pub fn new_from_bytes(data: impl AsRef<[u8]>) -> Self {
		Self {
			path: "<memory>".into(),
			data: data.as_ref().to_vec(),
		}
	}

	pub fn path(&self) -> &str {
		&self.path
	}

	pub fn data(&self) -> &[u8] {
		&self.data
	}
}

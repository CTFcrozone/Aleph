use object::File;

use crate::{binary::Binary, error::Result};

pub fn parse_binary(binary: &Binary) -> Result<File<'_>> {
	Ok(File::parse(binary.data())?)
}

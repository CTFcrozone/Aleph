use std::fmt;

use crate::analysis::{SectionInfo, SegmentInfo, SymbolInfo};

impl fmt::Display for SectionInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:<20} addr=0x{:08x} size=0x{:x} align=0x{:x} kind={:?}",
			self.name, self.addr, self.size, self.align, self.kind
		)
	}
}

impl fmt::Display for SegmentInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:<20} addr=0x{:08x} size=0x{:x} align=0x{:x} perms={:?}",
			self.name, self.addr, self.size, self.align, self.permissions
		)
	}
}

impl fmt::Display for SymbolInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:<30} addr=0x{:08x} size=0x{:x} kind={:?}",
			self.name, self.addr, self.size, self.kind
		)
	}
}

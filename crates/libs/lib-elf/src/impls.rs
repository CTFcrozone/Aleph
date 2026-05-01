use std::fmt;

use crate::{
	InstructionInfo,
	analysis::{SectionInfo, SegmentInfo, SymbolInfo},
};

impl fmt::Display for SectionInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:<20} addr=0x{:016x} size=0x{:x} align=0x{:x} kind={:?} flags={:?}",
			self.name, self.addr, self.size, self.align, self.kind, self.flags
		)
	}
}

impl fmt::Display for SegmentInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:<20} addr=0x{:016x} size=0x{:x} align=0x{:x} perms={:?} flags={:?}",
			self.name, self.addr, self.size, self.align, self.permissions, self.flags
		)
	}
}

impl fmt::Display for SymbolInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:<30} addr=0x{:016x} size=0x{:x} kind={:?} scope={:?}",
			self.name, self.addr, self.size, self.kind, self.scope
		)
	}
}

impl fmt::Display for InstructionInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "0x{:016x}: {:<8} {:<20}", self.addr, self.mnemonic, self.op_str)
	}
}

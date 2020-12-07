#[repr(C)]
#[derive(Debug)]
pub(crate) struct PartitionEntry {
	// A GUID contains 4 "parts" in this order: 4 bytes, 2 byte, 2 byte, 8
	// However, they are read as little endian, so we must reverse
	// these in place (kind of. guids are stupid)
	pub(crate) kind_p1: [u8; 4],
	pub(crate) kind_p2: [u8; 2],
	pub(crate) kind_p3: [u8; 2],
	pub(crate) kind_p4: [u8; 8],
	// 5-part unique guid??? idek its like weird or smthn
	pub(crate) ukind_p1: [u8; 4],
	pub(crate) ukind_p2: [u8; 2],
	pub(crate) ukind_p3: [u8; 2],
	pub(crate) ukind_p4: [u8; 2],
	pub(crate) ukind_p5: [u8; 6],
	// lba ptr thingies
	pub(crate) first_lba: [u8; 8],
	pub(crate) last_lba: [u8; 8],
	// Ignore flags
	_offset2: [u8; 8],
	pub(crate) name: [u8; 72],
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct HeaderEntry {
	pub(crate) signature: [u8; 8],
	// stuff we dont need
	_offset: [u8; 32],
	// first/last usable lbas
	pub(crate) first_lba: [u8; 8],
	pub(crate) last_lba: [u8; 8],
	// 5-part disk guid??? idek its like weird or smthn
	pub(crate) kind_p1: [u8; 4],
	pub(crate) kind_p2: [u8; 2],
	pub(crate) kind_p3: [u8; 2],
	pub(crate) kind_p4: [u8; 2],
	pub(crate) kind_p5: [u8; 6],
}

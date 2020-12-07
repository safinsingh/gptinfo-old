use crate::{errors::Error, guids, table::Table, OPTS};
use anyhow::{Context as _, Result};
use colored::{ColoredString, Colorize};
use std::{fs::File, os::unix::fs::FileExt};

const MBR_OFFSET: u16 = 512;
const GPT_HEADER_SIG: u64 = 0x5452415020494645;
const UNUSED_PARTITION: &str = "Unused entry";

pub(crate) enum ReaderKind {
	Header,
	Entry,
}

#[repr(C)]
#[derive(Debug)]
struct PartitionEntry {
	// A GUID contains 4 "parts" in this order: 4 bytes, 2 byte, 2 byte, 8
	// However, they are read as little endian, so we must reverse
	// these in place (kind of. guids are stupid)
	kind_p1: [u8; 4],
	kind_p2: [u8; 2],
	kind_p3: [u8; 2],
	kind_p4: [u8; 8],
	// 5-part unique guid??? idek its like weird or smthn
	ukind_p1: [u8; 4],
	ukind_p2: [u8; 2],
	ukind_p3: [u8; 2],
	ukind_p4: [u8; 2],
	ukind_p5: [u8; 6],
	// lba ptr thingies
	first_lba: [u8; 8],
	last_lba: [u8; 8],
	// Ignore flags
	_offset2: [u8; 8],
	name: [u8; 72],
}

#[repr(C)]
#[derive(Debug)]
struct HeaderEntry {
	signature: [u8; 8],
	// stuff we dont need
	_offset: [u8; 32],
	// first/last usable lbas
	first_lba: [u8; 8],
	last_lba: [u8; 8],
	// 5-part disk guid??? idek its like weird or smthn
	kind_p1: [u8; 4],
	kind_p2: [u8; 2],
	kind_p3: [u8; 2],
	kind_p4: [u8; 2],
	kind_p5: [u8; 6],
}

fn string_from_bytes(bytes: &[u8; 72]) -> Result<ColoredString> {
	let (front, slice, back) = unsafe { bytes.align_to::<u16>() };
	if front.is_empty() && back.is_empty() {
		Ok(ColoredString::from(String::from_utf16(slice)?.as_str()))
	} else {
		Err(Error::UTF16.into())
	}
}

fn guid_from_bytes(
	kind_p1: &[u8; 4],
	kind_p2: &[u8; 2],
	kind_p3: &[u8; 2],
	kind_p4: &[u8; 2],
	kind_p5: &[u8; 6],
) -> String {
	let mut guid = String::new();
	for ch in kind_p1 {
		guid.push_str(&format!("{:01$x?}", ch, 2));
	}
	guid.push('-');
	for ch in kind_p2 {
		guid.push_str(&format!("{:01$x?}", ch, 2));
	}
	guid.push('-');
	for ch in kind_p3 {
		guid.push_str(&format!("{:01$x?}", ch, 2));
	}
	guid.push('-');
	for ch in kind_p4 {
		guid.push_str(&format!("{:01$x?}", ch, 2));
	}
	guid.push('-');
	for ch in kind_p5 {
		guid.push_str(&format!("{:01$x?}", ch, 2));
	}
	guid.to_ascii_uppercase()
}

impl ReaderKind {
	fn analyze(
		&mut self,
		bytes: &mut [u8; 512],
		loc: &str,
		writer: &mut Table,
	) -> Result<()> {
		match self {
			Self::Header => {
				// SAFETY: guaranteed to be safe because we guarantee the layout
				// of the struct
				let entry =
					unsafe { &mut *(bytes.as_ptr() as *mut HeaderEntry) };

				// Magic signature in the first 8 bytes of LBA1 to verify EFI
				// system integrity
				let signature = u64::from_le_bytes(entry.signature);

				if signature == GPT_HEADER_SIG {
					log!("Validated signature of GPT header");

					// Begin disk section
					writer.push_cell("Name".bold());
					if OPTS.guid {
						writer.push_cell("Unique GUID".bold());
					}

					writer.push_cells(vec![
						"Type".bold(),
						"Start".bold(),
						"End".bold(),
						"Sectors".bold(),
						"Size".bold(),
					]);

					let start = u64::from_le_bytes(entry.first_lba);
					let end = u64::from_le_bytes(entry.last_lba);

					// reverse guid parts for endianness
					entry.kind_p1.reverse();
					entry.kind_p2.reverse();
					entry.kind_p3.reverse();

					writer.push_cell(format!("Disk ({})", loc).as_str().into());
					if OPTS.guid {
						writer.push_cell(
							guid_from_bytes(
								&entry.kind_p1,
								&entry.kind_p2,
								&entry.kind_p3,
								&entry.kind_p4,
								&entry.kind_p5,
							)
							.as_str()
							.into(),
						);
					}

					writer.push_cells(vec![
						"Block Device".into(),
						start.to_string().as_str().into(),
						end.to_string().as_str().into(),
						(end - start + 1).to_string().as_str().into(),
						format_bytes(end, start).as_str().into(),
					]);
					Ok(())
				} else {
					Err(Error::InvalidSignature.into())
				}
			}
			Self::Entry => {
				// SAFETY: guaranteed to be safe because we guarantee the layout
				// of the struct
				let entry =
					unsafe { &mut *(bytes.as_ptr() as *mut PartitionEntry) };

				// account for endianness
				entry.kind_p1.reverse();
				entry.kind_p2.reverse();
				entry.kind_p3.reverse();
				entry.kind_p4.reverse();

				// Get partition type from GUID
				let kind = guids::match_partition_guid(
					&entry.kind_p1,
					&entry.kind_p2,
					&entry.kind_p3,
					&entry.kind_p4,
				)?;

				if kind != UNUSED_PARTITION {
					let start = u64::from_le_bytes(entry.first_lba);
					let end = u64::from_le_bytes(entry.last_lba);

					writer.push_cell(string_from_bytes(&entry.name)?);
					if OPTS.guid {
						entry.ukind_p1.reverse();
						entry.ukind_p2.reverse();
						entry.ukind_p3.reverse();

						writer.push_cell(
							guid_from_bytes(
								&entry.ukind_p1,
								&entry.ukind_p2,
								&entry.ukind_p3,
								&entry.ukind_p4,
								&entry.ukind_p5,
							)
							.as_str()
							.into(),
						);
					}

					writer.push_cells(vec![
						kind.into(),
						start.to_string().as_str().into(),
						end.to_string().as_str().into(),
						(end - start + 1).to_string().as_str().into(),
						format_bytes(end, start).as_str().into(),
					]);
				}
				Ok(())
			}
		}
	}
}

fn format_bytes(end: u64, start: u64) -> String {
	let raw = (end - start + 1) * 512;
	match raw {
		// 1024 bytes in a MB
		1..=1023 => {
			format!("{}B", raw)
		}
		// 1048576 bytes in a KB
		1024..=1048575 => {
			format!("{}K", raw / 1024)
		}
		// 1073741824 bytes in a GB
		1048576..=1073741823 => {
			format!("{}M", raw / 1024 / 1024)
		}
		// 1099511627776 bytes in a TB
		1073741824..=1099511627775 => {
			format!("{}G", raw / 1024 / 1024 / 1024)
		}
		// 1125899906842624 bytes in a PB
		1099511627776..=1125899906842624 => {
			format!("{}T", raw / 1024 / 1024 / 1024 / 1024)
		}
		// if you have more than 1023 terabytes of
		// storage... go do something useful instead
		// of reading this
		x => panic!("a meaningful error {}", x),
	}
}

pub(crate) struct Reader<'a> {
	loc: &'a str,
	lba: u16,
	bytes: [u8; 512],
	writer: Table,
}

impl<'a> Reader<'a> {
	pub(crate) fn new(loc: &'a str) -> Result<Reader<'a>> {
		Ok(Self {
			loc,
			lba: 1,
			bytes: [0u8; 512],
			writer: Table::new(if OPTS.guid { 7 } else { 6 }),
		})
	}

	pub(crate) fn read(&mut self) -> Result<ReaderKind> {
		let entry_size = match self.lba {
			1 => 512,
			2..=33 => 128,
			pos => return Err(Error::UndefLbaPtr(pos).into()),
		};
		let offset = MBR_OFFSET + entry_size * (self.lba - 1);

		File::open(self.loc)
			.context("Failed to open EFI system for reading")?
			.read_at(&mut self.bytes, offset.into())
			.with_context(|| {
				format!("Failed read EFI system at offset: {}b", offset)
			})?;

		match self.lba {
			1 => Ok(ReaderKind::Header),
			2..=33 => Ok(ReaderKind::Entry),
			pos => Err(Error::UndefLbaPtr(pos).into()),
		}
	}

	pub(crate) fn run(mut self) -> Result<()> {
		while let Ok(mut reader) = self.read() {
			reader.analyze(&mut self.bytes, self.loc, &mut self.writer)?;
			self.lba += 1;
		}

		self.writer.draw();
		Ok(())
	}
}
